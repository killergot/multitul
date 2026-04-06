use std::path::{Path, PathBuf};
use std::fs;
use log::info;
use std::io::Read;

use super::commit::Commit;

use flate2::read::ZlibDecoder;
use crate::utils::git::git_error::GitError;
use crate::utils::git::git_ref::GitRef;
use crate::utils::git::hash::Hash;
use crate::utils::git::ref_name::RefName;
use crate::utils::git::ref_target::RefTarget;
use crate::utils::git::repository::{Repository};

pub struct GitProvider {
    main_path: PathBuf,
    repository: Repository,
    verbose: bool,
}

impl GitProvider {
    pub fn new<P: AsRef<Path>>(main_path: P) -> Self {
        GitProvider {
            main_path: main_path.as_ref().to_path_buf(),
            repository: Repository::new(),
            verbose: false,
        }
    }

    pub fn scan_repository(&mut self){
    }
}