#![allow(dead_code)]
macro_rules! log { ($($arg:tt)*) => { let _ = format!($($arg)*); }; }
#[path = "../../../src/stack_policy.rs"] mod stack_policy;
#[path = "../../../src/stack.rs"] mod stack;
mod web { pub struct State { pub stack: std::sync::Arc<crate::stack::Stack> } impl State { pub fn wake(&self) {} } }
mod scan {
    pub fn valid_tid(value: &str) -> bool {
        value.len() == 9 && value.as_bytes()[..4].iter().all(u8::is_ascii_uppercase) && value.as_bytes()[4..].iter().all(u8::is_ascii_digit)
    }
}

#[path = "../../../src/telemetry_math.rs"] mod telemetry_math;

#[path = "../../../src/library.rs"] mod library;

#[path = "../../../src/migration.rs"] mod migration;

#[path = "../../../src/installed.rs"] mod installed;
#[path = "../../../src/backup.rs"] mod backup;

#[path = "../../../src/ampr_index.rs"] mod ampr_index;
#[cfg(test)]mod stream_tests{use super::ampr_index::*;
#[test]fn incompatible_block_grids_and_sparse_chunks_require_loose_cache(){let normal=Chunk{pkg_off:65536,stored_len:262144,raw_len:262144,kraken:None};assert!(streamable("data.bin",262144,&[normal],1048576));let small=Chunk{pkg_off:65536,stored_len:131072,raw_len:131072,kraken:None};assert!(!streamable("data.bin",262144,&[small],1048576));let zero=Chunk{pkg_off:0,stored_len:0,raw_len:262144,kraken:Some(Kraken{even_len:0,mode:0})};assert!(!streamable("zero.bin",262144,&[zero],1048576));}}

#[path = "../../../src/relocation.rs"] mod relocation;
#[path = "../../../src/config.rs"] mod config;

#[path = "../../../src/installer_io.rs"] mod installer_io;

#[path = "../../../src/background.rs"] mod background;

#[path = "../../../src/installed_access.rs"] mod installed_access;
