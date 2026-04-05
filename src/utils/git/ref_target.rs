use crate::utils::git::hash::Hash;
use crate::utils::git::ref_name::RefName;

#[derive(Debug,Clone)]
pub enum RefTarget {
    Direct(Hash),
    Symbolic(RefName),
}