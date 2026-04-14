use crate::utils::git::commit::Commit;
use crate::utils::git::git_ref::GitRef;
use crate::utils::git::hash::Hash;
use crate::utils::git::ref_name::RefName;
use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct GitRepository {
    pub commits: HashMap<Hash, Commit>,
    pub refs: HashMap<RefName, GitRef>,
    pub head: Option<GitRef>,
}

impl GitRepository {
    pub fn new() -> Self {
        GitRepository::default()
    }
}
