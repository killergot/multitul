use crate::utils::git::hash::Hash;
use crate::utils::git::ref_name::RefName;
use crate::utils::git::ref_target::RefTarget;

#[derive(Debug, Clone)]
pub struct GitRef {
    pub name: RefName,
    pub target: RefTarget,
}

impl GitRef {
    pub fn new(name: impl Into<RefName>, target: impl Into<RefTarget>) -> GitRef {
        GitRef {
            name: name.into(),
            target: target.into(),
        }
    }
}
