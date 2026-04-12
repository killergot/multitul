use crate::utils::git::commit::Commit;
use crate::utils::git::git_ref::GitRef;
use crate::utils::git::hash::Hash;
use crate::utils::git::ref_name::RefName;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Repository {
    pub commits: HashMap<Hash, Commit>,
    pub refs: HashMap<RefName, GitRef>,
    pub head: Option<GitRef>,
}

impl Repository {
    pub fn new() -> Self {
        Repository::default()
    }
}
