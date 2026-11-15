//! Keep installed PKGs in place and make their read-only sandbox mount accessible.
use std::{fs, io, path::{Path, PathBuf}};
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use serde_json::{json, Value};
pub const ROOT: &str = "/data/titan-core/installed-sources";

pub fn prepare(path: &Path, tid: &str, size: u64) -> io::Result<PathBuf> {
    if !crate::library::installed_package_path(path) {
        return Err(io::Error::other("Not a supported installed package path"));
    }
    prepare_at(path, tid, size, Path::new(ROOT))
}

fn prepare_at(path: &Path, tid: &str, size: u64, receipts: &Path) -> io::Result<PathBuf> {
    let parent = path.parent().ok_or_else(|| io::Error::other("Missing package directory"))?;
    if !crate::scan::valid_tid(tid) || parent.file_name().is_none_or(|n| n != tid) || path.file_name().is_none_or(|n| n != "app.pkg") {
        return Err(io::Error::other("Installed package path/title mismatch"));
    }
    let file = fs::symlink_metadata(path)?;
    let dir = fs::symlink_metadata(parent)?;
    if !file.is_file() || file.file_type().is_symlink() || !dir.is_dir() || dir.file_type().is_symlink() || file.len() != size {
        return Err(io::Error::other("Installed source changed or is not a regular package"));
    }
    if fs::symlink_metadata(receipts).is_ok_and(|m| m.file_type().is_symlink()) {
        return Err(io::Error::other("Installed source receipt root is a symlink"));
    }
    fs::create_dir_all(receipts)?;
    let receipt = receipts.join(format!("{tid}.json"));
    let previous = match fs::symlink_metadata(&receipt) {
        Ok(m) if !m.is_file() || m.file_type().is_symlink() => return Err(io::Error::other("Invalid installed source receipt")),
        Ok(_) => Some(serde_json::from_slice::<Value>(&fs::read(&receipt)?).map_err(io::Error::other)?),
        Err(e) if e.kind() == io::ErrorKind::NotFound => None,
        Err(e) => return Err(e),
    };
    if previous.as_ref().is_some_and(|v| v["owner"] != "TITAN CORE" || v["title_id"] != tid || v["source"] != path.to_string_lossy().as_ref()) {
        return Err(io::Error::other("Installed source receipt belongs to another source"));
    }
    let same = previous.as_ref().is_some_and(|v| v["inode"].as_u64() == Some(file.ino()) && v["device"].as_u64() == Some(file.dev()) && v["size"].as_u64() == Some(size));
    let record = if same { previous.unwrap() } else {
        json!({"owner":"TITAN CORE", "title_id":tid, "source":path.to_string_lossy(), "inode":file.ino(), "device":file.dev(), "size":size,
            "original_mode":file.permissions().mode() & 0o7777, "original_directory_mode":dir.permissions().mode() & 0o7777, "mode":"in-place"})
    };
    // Persist original permissions before changing them. Never copy, rename or link the PKG.
    if !same {
        use std::io::Write;
        let temporary = receipts.join(format!("{tid}.next"));
        let mut f = fs::OpenOptions::new().write(true).create_new(true).open(&temporary)?;
        let saved = (|| { f.write_all(record.to_string().as_bytes())?; f.sync_all()?; fs::rename(&temporary, &receipt) })();
        drop(f);
        if let Err(error) = saved { let _ = fs::remove_file(&temporary); return Err(error); }
    }
    let mode = file.permissions().mode() & 0o7777;
    let directory_mode = dir.permissions().mode() & 0o7777;
    if directory_mode & 0o555 != 0o555 { fs::set_permissions(parent, fs::Permissions::from_mode(directory_mode | 0o555))?; }
    if mode & 0o444 != 0o444 { fs::set_permissions(path, fs::Permissions::from_mode(mode | 0o444))?; }
    let now = fs::symlink_metadata(path)?;
    if !now.is_file() || now.file_type().is_symlink() || now.ino() != file.ino() || now.dev() != file.dev() || now.len() != size {
        return Err(io::Error::other("Installed source changed during preparation"));
    }
    Ok(path.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn root(n: &str) -> PathBuf { std::env::temp_dir().join(format!("titan-in-place-{}-{n}",std::process::id())) }
    #[test]
    fn owner_only_package_stays_at_original_path_with_one_link_and_original_permissions_recorded() {
        let r=root("access"); let p=r.join("PPSA12345/app.pkg"); fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(&p,b"original package bytes").unwrap(); fs::set_permissions(&p,fs::Permissions::from_mode(0o600)).unwrap();
        fs::set_permissions(p.parent().unwrap(),fs::Permissions::from_mode(0o700)).unwrap(); let before=fs::metadata(&p).unwrap();
        assert_eq!(prepare_at(&p,"PPSA12345",22,&r.join("receipts")).unwrap(),p);
        let after=fs::metadata(&p).unwrap(); assert_eq!(before.ino(),after.ino()); assert_eq!(after.nlink(),1);
        assert_eq!(fs::read(&p).unwrap(),b"original package bytes"); assert_eq!(after.permissions().mode()&0o777,0o644);
        assert_eq!(fs::metadata(p.parent().unwrap()).unwrap().permissions().mode()&0o777,0o755);
        prepare_at(&p,"PPSA12345",22,&r.join("receipts")).unwrap();
        let v:Value=serde_json::from_slice(&fs::read(r.join("receipts/PPSA12345.json")).unwrap()).unwrap();
        assert_eq!(v["original_mode"],0o600); assert_eq!(v["original_directory_mode"],0o700);
        assert_eq!(fs::read_dir(p.parent().unwrap()).unwrap().count(),1); fs::remove_dir_all(r).unwrap();
    }
    #[test]
    fn wrong_title_size_and_symlink_are_refused_without_moving_the_original() {
        let r=root("invalid"); let p=r.join("PPSA12345/app.pkg"); fs::create_dir_all(p.parent().unwrap()).unwrap(); fs::write(&p,b"bytes").unwrap();
        assert!(prepare_at(&p,"PPSA99999",5,&r.join("receipts")).is_err());
        assert!(prepare_at(&p,"PPSA12345",99,&r.join("receipts")).is_err());
        let link=r.join("PPSA99999/app.pkg"); fs::create_dir_all(link.parent().unwrap()).unwrap(); std::os::unix::fs::symlink(&p,&link).unwrap();
        assert!(prepare_at(&link,"PPSA99999",5,&r.join("receipts")).is_err()); assert_eq!(fs::read(p).unwrap(),b"bytes"); fs::remove_dir_all(r).unwrap();
    }
}
