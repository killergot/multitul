use std::collections::HashMap;
use crate::utils::git::commit::Commit;
use crate::utils::git::git_ref::GitRef;

pub type Hash = String;
pub type RefName = String;

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