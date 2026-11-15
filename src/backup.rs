//! Stream metadata snapshots before importing an installed package into the native workflow.
use std::{fs, io::{self, Read}, path::Path};
const RESERVE: u64 = 256 << 20;

fn copy(src: &Path, dst: &Path, bytes: &mut u64, budget: u64) -> io::Result<()> {
    let m = match fs::symlink_metadata(src) {
        Ok(m) => m,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(()),
        Err(e) => return Err(e),
    };
    if m.file_type().is_symlink() { return Err(io::Error::other("Metadata snapshot refuses symlinks")); }
    if m.is_dir() {
        fs::create_dir_all(dst)?;
        for e in fs::read_dir(src)? { let e = e?; copy(&e.path(), &dst.join(e.file_name()), bytes, budget)?; }
    } else if m.is_file() {
        let next = bytes.checked_add(m.len()).filter(|b| *b <= budget).ok_or_else(|| {
            io::Error::new(io::ErrorKind::StorageFull, format!(
                "Insufficient metadata snapshot space for {} ({} bytes); backup budget is {} bytes after reserving 256 MiB",
                src.display(), m.len(), budget))
        })?;
        if let Some(p) = dst.parent() { fs::create_dir_all(p)?; }
        let mut output = fs::OpenOptions::new().write(true).create_new(true).open(dst)?;
        // io::copy streams with bounded buffering; do not allocate the complete metadata file.
        let mut input = fs::File::open(src)?.take(m.len().saturating_add(1));
        let copied = io::copy(&mut input, &mut output)?;
        if copied != m.len() || fs::metadata(src)?.len() != m.len() {
            return Err(io::Error::other(format!("Metadata changed during snapshot: {}", src.display())));
        }
        output.sync_all()?;
        fs::set_permissions(dst, m.permissions())?;
        *bytes = next;
    } else { return Err(io::Error::other("Metadata snapshot refuses special files")); }
    Ok(())
}

pub fn snapshot(app: &Path, meta: &Path, dest: &Path, budget: u64) -> io::Result<()> {
    if dest.join("complete").is_file() { return Ok(()); }
    if dest.exists() { return Err(io::Error::other("Incomplete metadata backup; preserved for inspection")); }
    let parent = dest.parent().ok_or_else(|| io::Error::other("Invalid backup directory"))?;
    fs::create_dir_all(parent)?;
    let mut chosen = None;
    for i in 0..1000 {
        let temp = parent.join(format!("pending-{}-{}-{i}", dest.file_name().unwrap().to_string_lossy(), std::process::id()));
        match fs::create_dir(&temp) {
            Ok(()) => { chosen = Some(temp); break; }
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {}
            Err(e) => return Err(e),
        }
    }
    let temp = chosen.ok_or_else(|| io::Error::other("No metadata snapshot directory available"))?;
    let result = (|| {
        let mut bytes = 0;
        copy(&app.join("sce_sys"), &temp.join("app/sce_sys"), &mut bytes, budget)?;
        copy(&app.join("icon0.png"), &temp.join("app/icon0.png"), &mut bytes, budget)?;
        copy(meta, &temp.join("appmeta"), &mut bytes, budget)?;
        fs::write(temp.join("complete"), b"Metadata only. Original app.pkg is not moved or copied.\n")?;
        fs::File::options().write(true).open(temp.join("complete"))?.sync_all()?;
        fs::rename(&temp, dest)
    })();
    if let Err(error) = result {
        // Clean only this new attempt, never original files, completed backups or older pending snapshots.
        if let Err(cleanup) = fs::remove_dir_all(&temp) {
            return Err(io::Error::new(error.kind(), format!("{error}; snapshot cleanup failed at {}: {cleanup}", temp.display())));
        }
        return Err(error);
    }
    Ok(())
}

pub fn installed(tid: &str, available: u64) -> io::Result<()> {
    snapshot(&Path::new("/user/app").join(tid), &Path::new("/user/appmeta").join(tid),
        &Path::new("/data/titan-core/installed-backups").join(tid), available.saturating_sub(RESERVE))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{io::{Seek, SeekFrom, Write}, path::PathBuf};
    fn root(name: &str) -> PathBuf { std::env::temp_dir().join(format!("titan-backup-{name}-{}", std::process::id())) }
    #[test]
    fn snapshot_excludes_package_and_preserves_original_metadata() {
        let r=root("preserve"); let app=r.join("app"); let meta=r.join("meta"); let dest=r.join("backup");
        fs::create_dir_all(app.join("sce_sys")).unwrap(); fs::create_dir_all(&meta).unwrap();
        fs::write(app.join("app.pkg"),b"owner-package").unwrap(); fs::write(app.join("sce_sys/param.json"),b"original").unwrap(); fs::write(meta.join("icon0.png"),b"icon").unwrap();
        snapshot(&app,&meta,&dest,1024).unwrap(); assert!(!dest.join("app/app.pkg").exists()); assert_eq!(fs::read(app.join("app.pkg")).unwrap(),b"owner-package");
        fs::write(app.join("sce_sys/param.json"),b"new").unwrap(); snapshot(&app,&meta,&dest,0).unwrap();
        assert_eq!(fs::read(dest.join("app/sce_sys/param.json")).unwrap(),b"original"); fs::remove_dir_all(r).unwrap();
    }
    #[test]
    fn metadata_above_64_mib_is_backed_up_without_a_package_copy() {
        let r=root("large"); let app=r.join("app"); let meta=r.join("meta"); let dest=r.join("backup");
        fs::create_dir_all(&app).unwrap(); fs::create_dir_all(&meta).unwrap(); fs::write(app.join("app.pkg"),b"original-package").unwrap();
        let len=(64<<20)+17; let mut f=fs::File::create(meta.join("large.dat")).unwrap(); f.set_len(len).unwrap();
        f.write_all(b"head").unwrap(); f.seek(SeekFrom::End(-4)).unwrap(); f.write_all(b"tail").unwrap(); drop(f);
        snapshot(&app,&meta,&dest,len).unwrap(); let mut saved=fs::File::open(dest.join("appmeta/large.dat")).unwrap(); assert_eq!(saved.metadata().unwrap().len(),len);
        let mut b=[0;4]; saved.read_exact(&mut b).unwrap(); assert_eq!(&b,b"head"); saved.seek(SeekFrom::End(-4)).unwrap(); saved.read_exact(&mut b).unwrap(); assert_eq!(&b,b"tail");
        assert!(dest.join("complete").is_file()); assert!(!dest.join("app/app.pkg").exists()); assert_eq!(fs::read(app.join("app.pkg")).unwrap(),b"original-package"); fs::remove_dir_all(r).unwrap();
    }
    #[test]
    fn failed_attempt_is_cleaned_and_retry_preserves_older_pending_backups() {
        let r=root("retry"); let app=r.join("app"); let meta=r.join("meta"); let dest=r.join("backups/title");
        fs::create_dir_all(app.join("sce_sys")).unwrap(); fs::create_dir_all(&meta).unwrap(); fs::create_dir_all(r.join("backups/pending-older")).unwrap();
        fs::write(r.join("backups/pending-older/keep"),b"prior attempt").unwrap(); fs::write(app.join("sce_sys/param.json"),b"original").unwrap(); fs::write(meta.join("icon.png"),b"metadata").unwrap();
        let e=snapshot(&app,&meta,&dest,8).unwrap_err(); assert_eq!(e.kind(),io::ErrorKind::StorageFull); assert!(!dest.exists()); assert_eq!(fs::read_dir(r.join("backups")).unwrap().count(),1);
        assert_eq!(fs::read(app.join("sce_sys/param.json")).unwrap(),b"original"); snapshot(&app,&meta,&dest,16).unwrap(); assert!(dest.join("complete").is_file());
        assert_eq!(fs::read(r.join("backups/pending-older/keep")).unwrap(),b"prior attempt"); fs::remove_dir_all(r).unwrap();
    }
    #[test]
    fn symlinks_are_refused_without_publishing_a_partial_snapshot() {
        let r=root("symlink"); let app=r.join("app"); let meta=r.join("meta"); let dest=r.join("backups/title");
        fs::create_dir_all(app.join("sce_sys")).unwrap(); fs::create_dir_all(&meta).unwrap(); fs::write(r.join("outside"),b"leave intact").unwrap();
        std::os::unix::fs::symlink(r.join("outside"),app.join("sce_sys/link")).unwrap(); assert!(snapshot(&app,&meta,&dest,1024).is_err());
        assert!(!dest.exists()); assert_eq!(fs::read_dir(r.join("backups")).unwrap().count(),0); assert_eq!(fs::read(r.join("outside")).unwrap(),b"leave intact"); fs::remove_dir_all(r).unwrap();
    }
}
