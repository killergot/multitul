use std::path::{Path, PathBuf};
use std::fs;

pub struct GitProvider {
    main_path: PathBuf,
    verbose: bool,
}

impl GitProvider {
    pub fn new<P: AsRef<Path>>(main_path: P) -> Self {
        GitProvider {
            main_path: main_path.as_ref().to_path_buf(),
            verbose: true,
        }
    }

    pub fn get_all_branches(&self){
        let mut branches: Vec<String> = Vec::new();
        let local_branch = self.main_path.join("refs/heads/");
        if self.verbose {
            println!("Listing local branches:");
        }
        self._get_all_branches(local_branch.as_path());
    }

    pub fn _get_all_branches(&self, subpath: &Path){
        if subpath.is_dir(){
            if let Ok(entries) = fs::read_dir(subpath) {
                for entry in entries{
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        self._get_all_branches(&path);
                    }
                }
            }
        }
        else if subpath.is_file(){
            if self.verbose{
                println!("{}", subpath.display());
            }
        }
    }
}