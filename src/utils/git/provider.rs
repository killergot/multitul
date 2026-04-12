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

    fn _scan_commit_chain(&mut self, hash: Hash) {
        // Если коммит уже есть, то и его подцепочка, тоже должна быть
        if !self.repository.commits.contains_key(&hash) {
            if let Ok(commit_raw) = self.storage.read_commit_by_hash(hash.as_str()){
                let commit = Commit::new(hash.clone(),commit_raw);
                for i in commit.parent_hashes.iter() {
                    self._scan_commit_chain(i.clone());
                }
                self.repository.commits
                    .entry(hash)
                    .or_insert(commit);
            };
        }
    }

    pub fn scan_repository(&mut self) -> Result<(), GitError> {
        let mut refs = self.storage.get_all_refs();
        for path in refs.iter_mut() {
            let commit_uid = self.storage.read_ref(path)?;
            let ref_name = RefName::from(
                path.strip_prefix(".git")
                    .unwrap_or(path)
                    .to_string_lossy()
                    .replace('\\', "/"),
            );
            let hash = Hash(commit_uid);
            self.repository.refs
                .entry(ref_name.clone())
                .or_insert(GitRef::new(ref_name,hash.clone()));
            self._scan_commit_chain(hash)
        }
        let head = self.storage.read_head()?;
        self.repository.head = if let Some(rest) = head.strip_prefix("ref: ") {
            Some(GitRef::new("HEAD", RefName::from(rest.trim())))
        } else {
            Some(GitRef::new("HEAD", Hash::from(head.trim())))
        };
        Ok(())
    }
}