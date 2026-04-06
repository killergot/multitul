use std::path::{Path, PathBuf};
use crate::utils::git::commit::Commit;
use crate::utils::git::git_error::GitError;
use crate::utils::git::git_ref::GitRef;
use crate::utils::git::GitStorage;
use crate::utils::git::hash::Hash;
use crate::utils::git::ref_name::RefName;
use crate::utils::git::repository::{Repository};

pub struct GitProvider {
    pub repository: Repository,
    storage: GitStorage,
    verbose: bool,
}

impl GitProvider {
    pub fn new() -> Self {
        GitProvider {
            repository: Repository::new(),
            storage: GitStorage::new(".git"),
            verbose: false,
        }
    }

    pub fn scan_repository(&mut self) -> Result<(), GitError> {
        let mut refs = self.storage.get_all_refs();
        for path in refs.iter_mut() {
            let row_commit = self.storage.read_commit_by_ref(path)?;
            let ref_name = RefName::from(
                path.strip_prefix(".git")
                    .unwrap_or(path)
                    .to_string_lossy()
                    .replace('\\', "/"),
            );
            self.repository.refs
                .entry(ref_name.clone())
                .or_insert(GitRef::new(ref_name,Hash(row_commit.0.clone())));
            self.repository.commits
                .entry(row_commit.0.clone().into())
                .or_insert(Commit::new(row_commit.0,row_commit.1));
        }
        let head = self.storage.read_head()?;
        let head = if let Some(rest) = head.strip_prefix("ref: ") {
            GitRef::new("HEAD", RefName::from(rest))
        } else {
            GitRef::new("HEAD", Hash::from(head))
        };
        Ok(())
    }
}