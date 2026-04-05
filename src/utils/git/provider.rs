use std::path::{Path, PathBuf};
use std::fs;
use log::info;

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

    pub fn get_all_branches(&self){
        let mut branches: Vec<PathBuf> = Vec::new();
        let local_branch = self.main_path.join("refs/heads/");
        if self.verbose {
            info!("Listing local branches:");
        }
        self._get_all_branches(local_branch.as_path(),&mut branches);
        for branch in branches{
            info!("{}", branch.display());
        }
    }

    pub fn _get_all_branches<'a>(&self, subpath: &'a Path, branches: &'a mut Vec<PathBuf>) {
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