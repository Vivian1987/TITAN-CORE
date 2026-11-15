#![allow(dead_code)]
macro_rules! log { ($($arg:tt)*) => { let _ = format!($($arg)*); }; }
#[path="../../../src/scan.rs"] mod scan;
#[path="../../../src/library.rs"] mod library;
mod storage {
    pub fn drive_of(path:&std::path::Path)->String {
        if path.starts_with("/mnt/ext0") { "/mnt/ext0".into() }
        else if path.starts_with("/tmp/titan-discovery/usb0") { "/tmp/titan-discovery/usb0".into() }
        else { "/data".into() }
    }
}
// The installer suite tests real receipt validation. Here inject its validated-root result
// to exercise the production scanner independently of console storage APIs.
mod installer {
    use std::cell::RefCell;
    thread_local! {pub static ROOTS:RefCell<Vec<String>>=const {RefCell::new(Vec::new())};}
    pub fn roots()->Vec<String>{ROOTS.with(|r|r.borrow().clone())}
}
mod cache {
    use std::{path::{Path,PathBuf},sync::atomic::{AtomicUsize,Ordering}};
    pub const CACHES:&str="/tmp/titan-discovery/cache";
    pub const DONE:&str=".done";
    pub static READS:AtomicUsize=AtomicUsize::new(0);
    pub fn dir(tid:&str)->PathBuf {Path::new(CACHES).join(tid)}
    pub fn peek_pkg(path:&Path)->std::io::Result<(String,String,String)> {
        READS.fetch_add(1,Ordering::SeqCst);
        crate::scan::parse_param(&std::fs::read(path)?)
    }
}
#[cfg(test)] mod discovery {
    use super::*;
    use std::{fs,path::{Path,PathBuf},sync::atomic::Ordering};
    const TID:&str="PPSA12345";
    fn setup()->PathBuf {
        let base=PathBuf::from("/tmp/titan-discovery");
        let _=fs::remove_dir_all(&base);
        fs::create_dir_all(base.join("usb0/Archive")).unwrap();
        fs::create_dir_all(base.join("internal")).unwrap();
        cache::READS.store(0,Ordering::SeqCst);
        installer::ROOTS.with(|r|r.borrow_mut().clear());base
    }
    fn pkg(path:&Path){fs::create_dir_all(path.parent().unwrap()).unwrap();fs::write(path,br#"{"titleId":"PPSA12345","contentVersion":"01.000.000"}"#).unwrap();}
    fn roots(paths:&[PathBuf])->Vec<String>{paths.iter().map(|p|p.to_string_lossy().into_owned()).collect()}
    #[test]fn uninstalled_external_archive_is_not_parsed_or_made_playable(){
        let b=setup();let path=b.join("usb0/Archive/Game.PKG");pkg(&path);
        let r=scan::scan(&roots(&[b.join("usb0/Archive")]),false);
        assert!(r.games.is_empty()&&r.duplicates.is_empty()&&r.errors.is_empty());
        assert_eq!(cache::READS.load(Ordering::SeqCst),0);assert!(path.is_file());
    }
    #[test]fn archive_cannot_shadow_installed_internal_pkg_even_when_scanned_first(){
        let b=setup();pkg(&b.join("usb0/Archive/Game.pkg"));pkg(&b.join("internal/app.pkg"));
        let r=scan::scan(&roots(&[b.join("usb0/Archive"),b.join("internal")]),false);
        assert_eq!(r.games.len(),1);assert_eq!(r.games[0].path,b.join("internal/app.pkg"));
        assert_eq!(r.games[0].title_id,TID);assert!(r.duplicates.is_empty());
    }
    #[test]fn validated_external_install_is_selected_instead_of_archive(){
        let b=setup();let dir=b.join("usb0/TITAN-CORE/Games").join(TID);let installed=dir.join(format!("{TID}.pkg"));pkg(&installed);pkg(&b.join("usb0/Archive/Game.pkg"));
        installer::ROOTS.with(|r|r.borrow_mut().push(dir.to_string_lossy().into_owned()));
        let r=scan::scan(&roots(&[b.join("usb0/Archive"),dir]),false);
        assert_eq!(r.games.len(),1);assert_eq!(r.games[0].path,installed);assert!(r.duplicates.is_empty());
    }
    #[test]fn installation_folder_name_alone_and_extra_pkg_in_owned_folder_are_not_authorization(){
        let b=setup();let dir=b.join("usb0/TITAN-CORE/Games").join(TID);pkg(&dir.join(format!("{TID}.pkg")));pkg(&dir.join("Other.pkg"));
        assert!(scan::scan(&roots(&[dir.clone()]),false).games.is_empty());
        installer::ROOTS.with(|r|r.borrow_mut().push(dir.to_string_lossy().into_owned()));
        let r=scan::scan(&roots(&[dir]),false);assert_eq!(r.games.len(),1);assert!(r.duplicates.is_empty());
    }
    #[test]fn external_system_installed_app_pkg_remains_supported(){
        let b=setup();let dir=PathBuf::from("/mnt/ext0/user/app/PPSA12345");fs::create_dir_all(&dir).unwrap();pkg(&dir.join("app.pkg"));pkg(&b.join("usb0/Archive/Game.pkg"));
        let r=scan::scan(&roots(&[b.join("usb0/Archive"),dir.clone()]),false);
        assert_eq!(r.games.len(),1);assert_eq!(r.games[0].path,dir.join("app.pkg"));assert!(r.duplicates.is_empty());
        fs::remove_dir_all(dir).unwrap();
    }
    #[test]fn external_extracted_folders_keep_plain_zstd_and_lz4_behavior(){
        let b=setup();for (index,packing) in [(None,scan::Packing::Plain),(Some("ampr_zstd.index"),scan::Packing::Zstd),(Some("ampr_assets.index"),scan::Packing::Lz4)]{
            let root=b.join(format!("usb0/{packing:?}"));let game=root.join("Game");fs::create_dir_all(game.join("sce_sys")).unwrap();pkg(&game.join("sce_sys/param.json"));fs::write(game.join("eboot.bin"),b"fixture").unwrap();if let Some(index)=index{fs::write(game.join(index),b"fixture").unwrap();}
            let r=scan::scan(&roots(&[root]),false);assert_eq!(r.games.len(),1);assert_eq!(r.games[0].kind,scan::Kind::Folder(packing));
        }
    }
    #[test]fn custom_homebrew_root_and_invalid_download_cannot_bypass_archive_policy(){
        let b=setup();let root=b.join("usb0/homebrew");fs::create_dir_all(&root).unwrap();fs::write(root.join("Broken.pkg"),b"not a package").unwrap();pkg(&root.join("Game.pkg"));
        let r=scan::scan(&roots(&[root]),true);assert!(r.games.is_empty()&&r.errors.is_empty());assert_eq!(cache::READS.load(Ordering::SeqCst),0);
    }
}
