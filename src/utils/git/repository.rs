use crate::utils::git::commit::Commit;
use crate::utils::git::git_ref::GitRef;
use crate::utils::git::hash::Hash;
use crate::utils::git::ref_name::RefName;
use crate::utils::git::ref_target::RefTarget;
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

    pub fn resolve_ref(&self, git_ref: GitRef) -> Option<Hash> {
        match git_ref.target {
            RefTarget::Direct(hash) => Some(hash),
            RefTarget::Symbolic(name) => self
                .refs
                .get(&name)
                .and_then(|next| self.resolve_ref(next.clone())),
        }
    }

    pub fn refs_by_hash(&self) -> HashMap<Hash, Vec<RefName>> {
        let mut result: HashMap<Hash, Vec<RefName>> = HashMap::new();

        for (name, git_ref) in self.refs.iter() {
            if let Some(hash) = self.resolve_ref(git_ref.clone()) {
                result.entry(hash).or_default().push(name.clone());
            }
        }

        if let Some(head) = &self.head {
            if let Some(hash) = self.resolve_ref(head.clone()) {
                result.entry(hash).or_default().push(head.name.clone());
            }
        }

        result
    }
}
