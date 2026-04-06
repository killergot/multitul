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

pub struct GitStorage {
    main_path: PathBuf,
    verbose: bool,
}

impl GitStorage {
    pub fn new<P: AsRef<Path>>(main_path: P) -> Self {
        GitStorage {
            main_path: main_path.as_ref().to_path_buf(),
            verbose: false,
        }
    }

    pub fn read_head(&self) -> GitRef {
        let content = fs::read_to_string(self.main_path.join("HEAD")).unwrap();
        let content = content.trim();

        if let Some(rest) = content.strip_prefix("ref: ") {
            GitRef::new("HEAD", RefName::from(rest))
        } else {
            GitRef::new("HEAD", Hash::from(content))
        }
    }


    pub fn get_commit_by_hash(&self, hash: &str) -> Option<String> {
        let dir_commit = self.main_path.join("objects").join(&hash[0..2]);
        let compressed = fs::read(&dir_commit.join(&hash[2..]))
            .expect("Failed to read commit file");

        let mut decoder = ZlibDecoder::new(&compressed[..]);
        let mut decoded = String::new();
        decoder
            .read_to_string(&mut decoded)
            .expect("Failed to decompress commit object");
        Some(decoded)
    }

    pub fn get_commit_by_refs(&self, refs: &Path) -> Result<Commit, String> {
        let commit_uid = fs::read_to_string(&refs).unwrap();
        let commit_uid = commit_uid.trim();

        info!(target: "git", "Reading commit {}", &commit_uid);

        if let Some(raw_commit) = self.get_commit_by_hash(commit_uid){
            let mut commit = Commit::new(commit_uid.to_string(), raw_commit);
            info!(target: "git", "Decoding commit {:?}", &commit);
            Ok(commit)
        } else{
            Err(format!("Failed to find commit {}", commit_uid))
        }
    }


    pub fn get_all_refs(&self) -> Vec<PathBuf>{
        let mut branches: Vec<PathBuf> = Vec::new();
        let local_branch = self.main_path.join("refs/heads/");
        if self.verbose {
            info!("Listing local branches:");
        }
        self._get_all_refs(local_branch.as_path(),&mut branches);
        for branch in &branches{
            if self.verbose {
                info!("{}", branch.display());
            }
        }
        branches
    }

    fn _get_all_refs<'a>(&self, subpath: &'a Path, branches: &'a mut Vec<PathBuf>) {
        if subpath.is_dir(){
            if let Ok(entries) = fs::read_dir(subpath) {
                for entry in entries{
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        self._get_all_refs(&path, branches);
                    }
                }
            }
        }
        else if subpath.is_file(){
            if self.verbose{
                info!("{}", subpath.display());
            }
            branches.push(subpath.to_path_buf());
        }
    }
}