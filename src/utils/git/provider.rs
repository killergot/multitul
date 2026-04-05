use std::path::{Path, PathBuf};
use std::fs;
use log::info;
use std::io::Read;

use super::commit::Commit;

use flate2::read::ZlibDecoder;

pub struct GitProvider {
    main_path: PathBuf,
    verbose: bool,
}

impl GitProvider {
    pub fn new<P: AsRef<Path>>(main_path: P) -> Self {
        GitProvider {
            main_path: main_path.as_ref().to_path_buf(),
            verbose: false,
        }
    }

    pub fn _get_commit_by_branch(&self, branch: PathBuf){
        let commit_uid = fs::read_to_string(&branch).unwrap();
        let commit_uid = commit_uid.trim();

        info!(target: "git", "Reading commit {}", &commit_uid);

        let dir_commit = self.main_path.join("objects").join(&commit_uid[0..2]);
        let compressed = fs::read(&dir_commit.join(&commit_uid[2..]))
            .expect("Failed to read commit file");

        let mut decoder = ZlibDecoder::new(&compressed[..]);
        let mut decoded = String::new();
        decoder
            .read_to_string(&mut decoded)
            .expect("Failed to decompress commit object");

        info!(target: "git", "Decompressed commit \n{}", decoded);
        let mut commit = Commit::new(commit_uid.to_string(), decoded);
        info!(target: "git", "Decompressed commit \n{:?}", commit);

    }


    pub fn get_all_branches(&self) -> Vec<PathBuf>{
        let mut branches: Vec<PathBuf> = Vec::new();
        let local_branch = self.main_path.join("refs/heads/");
        if self.verbose {
            info!("Listing local branches:");
        }
        self._get_all_branches(local_branch.as_path(),&mut branches);
        for branch in &branches{
            if self.verbose {
                info!("{}", branch.display());
            }
        }
        branches
    }

    fn _get_all_branches<'a>(&self, subpath: &'a Path, branches: &'a mut Vec<PathBuf>) {
        if subpath.is_dir(){
            if let Ok(entries) = fs::read_dir(subpath) {
                for entry in entries{
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        self._get_all_branches(&path, branches);
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