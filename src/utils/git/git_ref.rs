use crate::utils::git::ref_name::RefName;
use crate::utils::git::ref_target::RefTarget;

#[derive(Debug,Clone)]
pub struct GitRef {
    pub name: RefName,
    pub target: RefTarget,
}