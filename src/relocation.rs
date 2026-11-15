//! Recoverable, non-overwriting move into the proven homebrew launch workflow.
use std::{fs,io::{self,Write},path::{Path,PathBuf}};
use serde_json::{json,Value};
use std::os::unix::fs::PermissionsExt;
pub const ROOT:&str="/data/homebrew/titan-core";
fn regular(path:&Path)->bool{fs::symlink_metadata(path).is_ok_and(|m|m.is_file()&&!m.file_type().is_symlink())}
fn save(dir:&Path,v:&Value)->io::Result<()>{let tmp=dir.join("receipt.next");let mut f=fs::File::create(&tmp)?;f.write_all(v.to_string().as_bytes())?;f.sync_all()?;fs::rename(tmp,dir.join("receipt.json"))}
fn valid_receipt(v:&Value,tid:&str)->bool{v["owner"]=="TITAN CORE"&&v["title_id"]==tid&&v["size"].as_u64().is_some()&&v["original"].as_str().is_some_and(|p|crate::library::installed_package_path(Path::new(p))&&Path::new(p).parent().and_then(Path::file_name).is_some_and(|n|n==tid))}
pub fn move_package(src:&Path,tid:&str,size:u64,root:&Path)->io::Result<PathBuf>{
    if !crate::scan::valid_tid(tid)||!regular(src)||fs::metadata(src)?.len()!=size{return Err(io::Error::other("Package source is unavailable, changed, or not a regular file"));}
    if fs::symlink_metadata(root).is_ok_and(|m|m.file_type().is_symlink()){return Err(io::Error::other("Import root is a symlink"));}fs::create_dir_all(root)?;
    let dir=root.join(tid);let dst=dir.join(format!("{tid}.pkg"));
    match fs::create_dir(&dir){Ok(())=>{},Err(e)if e.kind()==io::ErrorKind::AlreadyExists=>{
        if fs::symlink_metadata(&dir)?.file_type().is_symlink(){return Err(io::Error::other("Import directory is a symlink"));}
        let v:Value=serde_json::from_slice(&fs::read(dir.join("receipt.json"))?).map_err(io::Error::other)?;
        if !valid_receipt(&v,tid)||v["original"]!=src.to_string_lossy().as_ref(){return Err(io::Error::other("Import directory belongs to another source"));}
    },Err(e)=>return Err(e)}
    if fs::symlink_metadata(&dst).is_ok(){return Err(io::Error::other("Destination already exists; no package is overwritten"));}
    fs::set_permissions(&dir,fs::Permissions::from_mode(0o755))?;
    let mut receipt=json!({"owner":"TITAN CORE","title_id":tid,"original":src.to_string_lossy(),"size":size,"original_mode":fs::metadata(src)?.permissions().mode() & 0o7777,"phase":"pending"});save(&dir,&receipt)?;
    match fs::rename(src,&dst){Ok(())=>{receipt["method"]=json!("atomic-rename");},Err(e)if e.raw_os_error()==Some(libc_exdev())=>{
        // /data is normally a nullfs alias of /user/data. Prove the parent identity before
        // trying the same rename through that alias; never assume it resolves elsewhere.
        let alias=dst.to_str().and_then(|p|p.strip_prefix("/data/")).map(|p|PathBuf::from(format!("/user/data/{p}")));
        let renamed=alias.filter(|a|same_directory(a.parent().unwrap(),dst.parent().unwrap())).is_some_and(|a|fs::rename(src,a).is_ok());
        if renamed{receipt["method"]=json!("atomic-user-data-alias");}else{
            return Err(io::Error::new(io::ErrorKind::CrossesDevices,"Single-copy import: direct rename is unavailable; original package remains in place. No full package copy was created."));
        }
    },Err(e)=>return Err(e)}
    // Imported bytes must be readable by the game process, including when the installed
    // source originally carried owner-only permissions. The source mode is in the receipt.
    fs::set_permissions(&dst,fs::Permissions::from_mode(0o644))?;
    receipt["phase"]=json!("moved");save(&dir,&receipt)?;Ok(dst)
}
fn libc_exdev()->i32{18}
fn same_directory(a:&Path,b:&Path)->bool{use std::os::unix::fs::MetadataExt;match(fs::metadata(a),fs::metadata(b)){(Ok(a),Ok(b))=>a.is_dir()&&b.is_dir()&&a.ino()==b.ino(),_=>false}}
/// A receipt may remain pending if power failed just after rename. Validate the exact final
/// path, size and absent old source, then let the normal PKG parser validate its title.
pub fn scan_roots(root:&Path)->Vec<String>{let mut roots=Vec::new();let Ok(entries)=fs::read_dir(root)else{return roots};for e in entries.flatten(){let dir=e.path();let tid=e.file_name().to_string_lossy().into_owned();if !crate::scan::valid_tid(&tid)||fs::symlink_metadata(&dir).is_ok_and(|m|m.file_type().is_symlink()){continue}let Ok(bytes)=fs::read(dir.join("receipt.json"))else{continue};let Ok(v)=serde_json::from_slice::<Value>(&bytes)else{continue};let pkg=dir.join(format!("{tid}.pkg"));if valid_receipt(&v,&tid)&&regular(&pkg)&&fs::metadata(&pkg).is_ok_and(|m|Some(m.len())==v["size"].as_u64())&&v["original"].as_str().is_some_and(|p|!Path::new(p).exists())&&matches!(v["phase"].as_str(),Some("pending"|"moved")){if fs::metadata(&dir).is_ok_and(|m|m.permissions().mode()&0o777!=0o755)&&fs::set_permissions(&dir,fs::Permissions::from_mode(0o755)).is_err(){continue}
if fs::metadata(&pkg).is_ok_and(|m|m.permissions().mode()&0o777!=0o644)&&fs::set_permissions(&pkg,fs::Permissions::from_mode(0o644)).is_err(){continue}
roots.push(dir.to_string_lossy().into_owned());}}roots.sort();roots}
#[cfg(test)]mod tests{use super::*;
fn root(s:&str)->PathBuf{std::env::temp_dir().join(format!("titan-import-{}-{s}",std::process::id()))}
#[test]fn rename_preserves_bytes_and_refuses_overwrite(){let r=root("rename");fs::create_dir_all(&r).unwrap();let src=r.join("original.pkg");fs::write(&src,b"owner bytes").unwrap();let dst=move_package(&src,"PPSA12345",11,&r.join("dest")).unwrap();assert!(!src.exists());assert_eq!(fs::read(&dst).unwrap(),b"owner bytes");assert_eq!(fs::metadata(&dst).unwrap().permissions().mode()&0o777,0o644);fs::write(&src,b"new content").unwrap();assert!(move_package(&src,"PPSA12345",11,&r.join("dest")).is_err());assert_eq!(fs::read(dst).unwrap(),b"owner bytes");assert_eq!(fs::read(src).unwrap(),b"new content");fs::remove_dir_all(r).unwrap();}
#[test]fn pending_rename_receipt_recovers_only_exact_final_size(){let r=root("recover");let dir=r.join("PPSA12345");fs::create_dir_all(&dir).unwrap();fs::write(dir.join("PPSA12345.pkg"),b"bytes").unwrap();fs::set_permissions(dir.join("PPSA12345.pkg"),fs::Permissions::from_mode(0o600)).unwrap();save(&dir,&json!({"owner":"TITAN CORE","title_id":"PPSA12345","original":"/user/app/PPSA12345/app.pkg","size":5,"phase":"pending"})).unwrap();assert_eq!(scan_roots(&r),vec![dir.to_string_lossy().into_owned()]);assert_eq!(fs::metadata(dir.join("PPSA12345.pkg")).unwrap().permissions().mode()&0o777,0o644);fs::write(dir.join("PPSA12345.pkg"),b"changed").unwrap();assert!(scan_roots(&r).is_empty());fs::remove_dir_all(r).unwrap();}
#[cfg(target_os="linux")]
#[test]fn cross_filesystem_refusal_preserves_original_and_creates_no_package_copy(){use std::os::unix::fs::MetadataExt;let r=root("cross-device");fs::create_dir_all(&r).unwrap();let src=PathBuf::from(format!("/dev/shm/titan-one-copy-{}.pkg",std::process::id()));assert_ne!(fs::metadata("/dev/shm").unwrap().dev(),fs::metadata(&r).unwrap().dev());fs::write(&src,b"single original").unwrap();let e=move_package(&src,"PPSA12345",15,&r.join("dest")).unwrap_err();assert_eq!(e.kind(),io::ErrorKind::CrossesDevices);assert_eq!(fs::read(&src).unwrap(),b"single original");assert!(!r.join("dest/PPSA12345/PPSA12345.pkg").exists());let names=fs::read_dir(r.join("dest/PPSA12345")).unwrap().map(|e|e.unwrap().file_name().to_string_lossy().into_owned()).collect::<Vec<_>>();assert_eq!(names,vec!["receipt.json"]);fs::remove_file(src).unwrap();fs::remove_dir_all(r).unwrap();}
#[test]fn incomplete_or_foreign_receipts_are_not_scanned(){let r=root("invalid");fs::create_dir_all(r.join("PPSA12345")).unwrap();fs::write(r.join("PPSA12345/PPSA12345.pkg"),b"owner").unwrap();assert!(scan_roots(&r).is_empty());fs::write(r.join("PPSA12345/receipt.json"),b"{}").unwrap();assert!(scan_roots(&r).is_empty());fs::remove_dir_all(r).unwrap();}
}

/// Return only receipt-proven internal imports to their original system package path.
/// Reserve the absent destination and rename only over that owned reservation.
/// Recheck its identity immediately before rename. Never install/update/uninstall the
/// same title concurrently with restoration; this SDK has no atomic no-replace rename.
/// /data's verified /user/data alias permits rename across nullfs path aliases.
pub fn restore_legacy(root: &Path, apps: &Path, paused: impl Fn() -> bool, validate: impl Fn(&Path, &str) -> io::Result<()>) -> Vec<(String, io::Result<PathBuf>)> {
    let mut results=Vec::new();
    if fs::symlink_metadata(root).is_ok_and(|m|m.file_type().is_symlink()){return results;}
    let Ok(entries)=fs::read_dir(root) else{return results};
    for e in entries.flatten() {
        if paused(){break;}
        let tid=e.file_name().to_string_lossy().into_owned(); let dir=e.path();
        if !crate::scan::valid_tid(&tid) || !fs::symlink_metadata(&dir).is_ok_and(|m|m.is_dir()&&!m.file_type().is_symlink()){continue;}
        let receipt=dir.join("receipt.json");
        if !regular(&receipt){continue;}
        let Ok(bytes)=fs::read(&receipt) else{continue}; let Ok(v)=serde_json::from_slice::<Value>(&bytes) else{continue};
        if !valid_receipt(&v,&tid) || v["original"]!=format!("/user/app/{tid}/app.pkg") || !matches!(v["phase"].as_str(),Some("pending"|"moved"|"restoring"|"restored")){continue;}
        let pkg=dir.join(format!("{tid}.pkg"));
        if fs::symlink_metadata(&pkg).is_err(){continue;}
        results.push((tid.clone(),validate(&pkg,&tid).and_then(|_|{if paused(){return Err(io::Error::other("A game started; restoration deferred"));}restore_one(&dir,&pkg,&apps.join(&tid).join("app.pkg"),v)})));
    }
    results
}
fn same_file(a: &Path,b: &Path) -> bool {
    use std::os::unix::fs::MetadataExt;
    match(fs::symlink_metadata(a),fs::symlink_metadata(b)){
        (Ok(a),Ok(b))=>a.is_file()&&b.is_file()&&!a.file_type().is_symlink()&&!b.file_type().is_symlink()&&a.dev()==b.dev()&&a.ino()==b.ino()&&a.len()==b.len(),_=>false
    }
}
fn restore_one(dir:&Path,pkg:&Path,original:&Path,mut receipt:Value)->io::Result<PathBuf>{
    use std::os::unix::fs::MetadataExt;
    let size=receipt["size"].as_u64().ok_or_else(||io::Error::other("Invalid legacy package size"))?;
    if !regular(pkg)||fs::metadata(pkg)?.len()!=size{return Err(io::Error::other("Legacy package changed; kept in homebrew"));}
    let parent=original.parent().ok_or_else(||io::Error::other("Missing original package parent"))?;
    if !fs::symlink_metadata(parent).is_ok_and(|m|m.is_dir()&&!m.file_type().is_symlink()){
        return Err(io::Error::other("Original installed title directory is missing; legacy package kept"));
    }
    let owned=fs::metadata(pkg)?;
    let alias=pkg.to_str().and_then(|p|p.strip_prefix("/data/")).map(|p|PathBuf::from(format!("/user/data/{p}")))
        .filter(|p|same_directory(p.parent().unwrap(),dir)&&regular(p)&&fs::metadata(p).is_ok_and(|m|m.len()==size&&m.ino()==owned.ino()&&m.modified().ok()==owned.modified().ok()));
    // Recover an interrupted link from 0.5.6 without creating any new hard links.
    if same_file(pkg,original)||alias.as_ref().is_some_and(|a|same_file(a,original)){
        receipt["phase"]=json!("restoring");save(dir,&receipt)?;fs::remove_file(pkg)?;
    }else{
        let token=match receipt["restore_reservation"].as_str(){Some(s) if s.starts_with("TITAN CORE restore reservation\n")=>s.to_owned(),_=>{
            let now=std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_err(io::Error::other)?.as_nanos();
            format!("TITAN CORE restore reservation\n{}\n{}\n{now}\n",receipt["title_id"].as_str().unwrap_or(""),std::process::id())
        }};
        // Never turn an existing package into a reservation. A journaled reservation can
        // be retried after interruption; its inode and contents are rechecked before rename.
        if fs::symlink_metadata(original).is_ok()&&!is_reservation(original,&token){
            return Err(io::Error::new(io::ErrorKind::AlreadyExists,"Original app.pkg already exists; no package was overwritten"));
        }
        receipt["phase"]=json!("restoring");receipt["restore_reservation"]=json!(token);save(dir,&receipt)?;
        if fs::symlink_metadata(original).is_err(){
            let mut f=fs::OpenOptions::new().write(true).create_new(true).open(original)?;
            let created=f.metadata()?;
            if let Err(e)=f.write_all(token.as_bytes()).and_then(|_|f.sync_all()){
                drop(f);
                let ours=fs::symlink_metadata(original).is_ok_and(|m|m.is_file()&&!m.file_type().is_symlink()&&m.ino()==created.ino()&&m.dev()==created.dev()&&m.len()<=token.len() as u64)
                    &&fs::read(original).is_ok_and(|b|token.as_bytes().starts_with(&b));
                if ours{let _=fs::remove_file(original);}return Err(e);
            }
        }
        let reservation=fs::symlink_metadata(original)?;
        if !is_reservation(original,&token){return Err(io::Error::other("Original package reservation changed; source kept"));}
        let rename=(||{
            if !same_reservation(original,&token,&reservation){return Err(io::Error::other("Original destination changed; source kept"));}
            match fs::rename(pkg,original){
                Ok(())=>Ok(()),
                Err(e)if e.raw_os_error()==Some(libc_exdev())||e.kind()==io::ErrorKind::Unsupported=>{
                    let a=alias.as_ref().ok_or_else(||io::Error::new(io::ErrorKind::CrossesDevices,"Original path cannot accept a same-filesystem rename; no package copy created"))?;
                    if !same_reservation(original,&token,&reservation){return Err(io::Error::other("Original destination changed; source kept"));}
                    fs::rename(a,original)
                },Err(e)=>Err(e)
            }
        })();
        if let Err(e)=rename{if same_reservation(original,&token,&reservation){let _=fs::remove_file(original);}return Err(e);}
        // rename changes the name, not the stored package bytes. Record completion only
        // after the new name identifies the source inode and the old name is absent.
        if !regular(original)||fs::metadata(original)?.ino()!=owned.ino()||fs::metadata(original)?.len()!=size||fs::symlink_metadata(pkg).is_ok(){
            return Err(io::Error::other("Restored package identity needs inspection; no further move attempted"));
        }
    }
    receipt["phase"]=json!("restored");receipt["restore_method"]=json!("reserved-destination-rename");save(dir,&receipt)?;
    Ok(original.to_owned())
}
fn is_reservation(path:&Path,token:&str)->bool{
    regular(path)&&fs::metadata(path).is_ok_and(|m|m.len()==token.len() as u64)&&fs::read(path).is_ok_and(|b|b==token.as_bytes())
}
fn same_reservation(path:&Path,token:&str,expected:&fs::Metadata)->bool{
    use std::os::unix::fs::MetadataExt;
    fs::symlink_metadata(path).is_ok_and(|m|m.ino()==expected.ino()&&m.dev()==expected.dev())&&is_reservation(path,token)
}

/// Restore only the small registration metadata needed for a missing installed title
/// directory. The caller first validates the actual package title. Existing folders stay intact.
pub fn restore_title_directory(apps:&Path,tid:&str,metadata:&Path)->io::Result<bool>{
    if !crate::scan::valid_tid(tid){return Err(io::Error::other("Invalid installed title"));}
    let dest=apps.join(tid);
    match fs::symlink_metadata(&dest){
        Ok(m)if m.is_dir()&&!m.file_type().is_symlink()=>return Ok(false),
        Ok(_)=>return Err(io::Error::other("Installed title path is not a regular directory")),
        Err(e)if e.kind()==io::ErrorKind::NotFound=>{},Err(e)=>return Err(e)
    }
    let param=metadata.join("param.json");let icon=metadata.join("icon0.png");
    if !regular(&param)||!regular(&icon){return Err(io::Error::other("Missing trusted title metadata; source kept"));}
    let v:Value=serde_json::from_slice(&fs::read(&param)?).map_err(io::Error::other)?;
    if v["titleId"].as_str().or_else(||v["title_id"].as_str())!=Some(tid){return Err(io::Error::other("Cached title metadata mismatch; source kept"));}
    let temp=apps.join(format!(".titan-title-{tid}-{}",std::process::id()));fs::create_dir(&temp)?;
    let result=(||{
        fs::create_dir(temp.join("sce_sys"))?;
        for (from,to) in [(&param,temp.join("sce_sys/param.json")),(&icon,temp.join("sce_sys/icon0.png")),(&icon,temp.join("icon0.png"))]{
            fs::copy(from,&to)?;fs::set_permissions(&to,fs::Permissions::from_mode(0o644))?;fs::File::options().write(true).open(to)?.sync_all()?;
        }
        if fs::symlink_metadata(&dest).is_ok(){return Err(io::Error::other("Installed directory appeared during restoration; kept"));}
        fs::rename(&temp,&dest)
    })();
    if result.is_err(){let _=fs::remove_dir_all(&temp);}result.map(|_|true)
}

#[cfg(test)]mod restore_tests{
    use super::*; use std::os::unix::fs::MetadataExt;
    fn setup(name:&str)->(PathBuf,PathBuf,PathBuf,PathBuf){
        let r=std::env::temp_dir().join(format!("titan-restore-{}-{name}",std::process::id()));
        let imports=r.join("imports");let apps=r.join("apps");let dir=imports.join("PPSA12345");
        fs::create_dir_all(&dir).unwrap();fs::create_dir_all(apps.join("PPSA12345")).unwrap();
        let pkg=dir.join("PPSA12345.pkg");fs::write(&pkg,b"package bytes").unwrap();
        save(&dir,&json!({"owner":"TITAN CORE","title_id":"PPSA12345","original":"/user/app/PPSA12345/app.pkg","size":13,"phase":"moved"})).unwrap();
        (r,imports,apps,pkg)
    }
    #[test]fn legacy_restore_keeps_one_inode_at_original_path_without_a_second_package(){
        let(r,imports,apps,pkg)=setup("normal");let inode=fs::metadata(&pkg).unwrap().ino();
        let results=restore_legacy(&imports,&apps,||false,|_,_|Ok(()));assert_eq!(results.len(),1);let path=results[0].1.as_ref().unwrap();
        assert_eq!(*path,apps.join("PPSA12345/app.pkg"));assert_eq!(fs::read(path).unwrap(),b"package bytes");
        assert_eq!(fs::metadata(path).unwrap().ino(),inode);assert_eq!(fs::metadata(path).unwrap().nlink(),1);assert!(!pkg.exists());
        assert!(restore_legacy(&imports,&apps,||false,|_,_|Ok(())).is_empty());fs::remove_dir_all(r).unwrap();
    }
    #[test]fn existing_different_system_package_is_never_overwritten(){
        let(r,imports,apps,pkg)=setup("collision");let original=apps.join("PPSA12345/app.pkg");fs::write(&original,b"new owner package").unwrap();
        assert!(restore_legacy(&imports,&apps,||false,|_,_|Ok(()))[0].1.is_err());assert_eq!(fs::read(original).unwrap(),b"new owner package");
        assert_eq!(fs::read(pkg).unwrap(),b"package bytes");fs::remove_dir_all(r).unwrap();
    }
    #[test]fn interrupted_link_is_recovered_without_duplicating_data(){
        let(r,imports,apps,pkg)=setup("resume");let original=apps.join("PPSA12345/app.pkg");fs::hard_link(&pkg,&original).unwrap();
        assert_eq!(fs::metadata(&original).unwrap().nlink(),2);assert!(restore_legacy(&imports,&apps,||false,|_,_|Ok(()))[0].1.is_ok());
        assert_eq!(fs::metadata(original).unwrap().nlink(),1);assert!(!pkg.exists());fs::remove_dir_all(r).unwrap();
    }
    #[test]fn invalid_size_or_receipt_cannot_restore_a_package(){
        let(r,imports,apps,pkg)=setup("invalid");fs::write(&pkg,b"changed").unwrap();assert!(restore_legacy(&imports,&apps,||false,|_,_|Ok(()))[0].1.is_err());
        assert!(!apps.join("PPSA12345/app.pkg").exists());
        let receipt=pkg.parent().unwrap().join("receipt.json");fs::write(receipt,b"{}").unwrap();assert!(restore_legacy(&imports,&apps,||false,|_,_|Ok(())).is_empty());
        assert_eq!(fs::read(pkg).unwrap(),b"changed");fs::remove_dir_all(r).unwrap();
    }
}
#[cfg(test)]mod restoration_guards{
    use super::*;
    #[test]fn running_game_defers_restoration(){
        let r=std::env::temp_dir().join(format!("titan-restore-paused-{}",std::process::id()));let imports=r.join("imports");let dir=imports.join("PPSA12345");let apps=r.join("apps");
        fs::create_dir_all(&dir).unwrap();fs::create_dir_all(apps.join("PPSA12345")).unwrap();let pkg=dir.join("PPSA12345.pkg");fs::write(&pkg,b"bytes").unwrap();
        save(&dir,&json!({"owner":"TITAN CORE","title_id":"PPSA12345","original":"/user/app/PPSA12345/app.pkg","size":5,"phase":"moved"})).unwrap();
        assert!(restore_legacy(&imports,&apps,||true,|_,_|Ok(())).is_empty());assert_eq!(fs::read(pkg).unwrap(),b"bytes");assert!(!apps.join("PPSA12345/app.pkg").exists());fs::remove_dir_all(r).unwrap();
    }
    #[cfg(target_os="linux")]
    #[test]fn cross_filesystem_restoration_never_falls_back_to_a_full_copy(){
        let r=std::env::temp_dir().join(format!("titan-restore-cross-{}",std::process::id()));let dir=r.join("imports/PPSA12345");
        let apps=PathBuf::from(format!("/dev/shm/titan-restore-apps-{}",std::process::id()));fs::create_dir_all(&dir).unwrap();fs::create_dir_all(apps.join("PPSA12345")).unwrap();
        let pkg=dir.join("PPSA12345.pkg");fs::write(&pkg,b"bytes").unwrap();save(&dir,&json!({"owner":"TITAN CORE","title_id":"PPSA12345","original":"/user/app/PPSA12345/app.pkg","size":5,"phase":"moved"})).unwrap();
        let results=restore_legacy(&r.join("imports"),&apps,||false,|_,_|Ok(()));assert_eq!(results[0].1.as_ref().unwrap_err().kind(),io::ErrorKind::CrossesDevices);
        assert_eq!(fs::read(pkg).unwrap(),b"bytes");assert!(!apps.join("PPSA12345/app.pkg").exists());fs::remove_dir_all(r).unwrap();fs::remove_dir_all(apps).unwrap();
    }
}
#[cfg(test)]mod validated_restore_test{
    use super::*;
    #[test]fn parser_rejection_keeps_the_legacy_source_untouched(){
        let r=std::env::temp_dir().join(format!("titan-restore-parse-{}",std::process::id()));let dir=r.join("imports/PPSA12345");let apps=r.join("apps");
        fs::create_dir_all(&dir).unwrap();fs::create_dir_all(apps.join("PPSA12345")).unwrap();let pkg=dir.join("PPSA12345.pkg");fs::write(&pkg,b"bytes").unwrap();
        save(&dir,&json!({"owner":"TITAN CORE","title_id":"PPSA12345","original":"/user/app/PPSA12345/app.pkg","size":5,"phase":"moved"})).unwrap();
        let rj=restore_legacy(&r.join("imports"),&apps,||false,|_,_|Err(io::Error::other("Title mismatch")));
        assert!(rj[0].1.is_err());assert_eq!(fs::read(pkg).unwrap(),b"bytes");assert!(!apps.join("PPSA12345/app.pkg").exists());fs::remove_dir_all(r).unwrap();
    }
}
#[cfg(test)]mod rename_recovery_tests{
    use super::*;use std::os::unix::fs::MetadataExt;
    fn setup(name:&str)->(PathBuf,PathBuf,PathBuf,PathBuf){
        let r=std::env::temp_dir().join(format!("titan-reservation-{}-{name}",std::process::id()));let imports=r.join("imports");let dir=imports.join("PPSA12345");let apps=r.join("apps");
        fs::create_dir_all(&dir).unwrap();fs::create_dir_all(apps.join("PPSA12345")).unwrap();let pkg=dir.join("PPSA12345.pkg");fs::write(&pkg,b"package bytes").unwrap();
        save(&dir,&json!({"owner":"TITAN CORE","title_id":"PPSA12345","original":"/user/app/PPSA12345/app.pkg","size":13,"phase":"restoring","restore_reservation":"TITAN CORE restore reservation\nfixture"})).unwrap();(r,imports,apps,pkg)
    }
    #[test]fn interrupted_destination_reservation_resumes_as_a_single_inode_rename(){
        let(r,imports,apps,pkg)=setup("resume");let ino=fs::metadata(&pkg).unwrap().ino();let original=apps.join("PPSA12345/app.pkg");
        fs::write(&original,b"TITAN CORE restore reservation\nfixture").unwrap();assert!(restore_legacy(&imports,&apps,||false,|_,_|Ok(()))[0].1.is_ok());
        assert_eq!(fs::read(&original).unwrap(),b"package bytes");assert_eq!(fs::metadata(original).unwrap().ino(),ino);assert!(!pkg.exists());fs::remove_dir_all(r).unwrap();
    }
    #[test]fn unrelated_file_cannot_be_treated_as_a_journaled_reservation(){
        let(r,imports,apps,pkg)=setup("collision");let original=apps.join("PPSA12345/app.pkg");fs::write(&original,b"another package").unwrap();
        assert!(restore_legacy(&imports,&apps,||false,|_,_|Ok(()))[0].1.is_err());assert_eq!(fs::read(original).unwrap(),b"another package");assert_eq!(fs::read(pkg).unwrap(),b"package bytes");fs::remove_dir_all(r).unwrap();
    }
    #[test]fn missing_title_folder_is_recreated_only_from_matching_metadata(){
        let(r,imports,apps,pkg)=setup("parent");fs::remove_dir(apps.join("PPSA12345")).unwrap();let meta=r.join("cache/sce_sys");fs::create_dir_all(&meta).unwrap();
        fs::write(meta.join("param.json"),br#"{"titleId":"PPSA12345"}"#).unwrap();fs::write(meta.join("icon0.png"),b"icon").unwrap();
        assert!(restore_title_directory(&apps,"PPSA12345",&meta).unwrap());assert!(apps.join("PPSA12345/sce_sys/param.json").is_file());
        assert!(restore_legacy(&imports,&apps,||false,|_,_|Ok(()))[0].1.is_ok());assert_eq!(fs::read(apps.join("PPSA12345/app.pkg")).unwrap(),b"package bytes");assert!(!pkg.exists());
        assert!(!restore_title_directory(&apps,"PPSA12345",&meta).unwrap());fs::remove_dir_all(r).unwrap();
    }
    #[test]fn mismatched_metadata_cannot_create_a_missing_system_title_directory(){
        let(r,_imports,apps,pkg)=setup("bad-parent");fs::remove_dir(apps.join("PPSA12345")).unwrap();let meta=r.join("cache/sce_sys");fs::create_dir_all(&meta).unwrap();
        fs::write(meta.join("param.json"),br#"{"titleId":"PPSA99999"}"#).unwrap();fs::write(meta.join("icon0.png"),b"icon").unwrap();
        assert!(restore_title_directory(&apps,"PPSA12345",&meta).is_err());assert!(!apps.join("PPSA12345").exists());assert_eq!(fs::read(pkg).unwrap(),b"package bytes");fs::remove_dir_all(r).unwrap();
    }
}
