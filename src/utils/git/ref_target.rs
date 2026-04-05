use crate::utils::git::hash::Hash;
use crate::utils::git::ref_name::RefName;

#[derive(Debug,Clone)]
pub enum RefTarget {
    Direct(Hash),
    Symbolic(RefName),
}

impl From<Hash> for RefTarget{
    fn from(hash: Hash)->Self{RefTarget::Direct(hash)}
}

impl From<RefName> for RefTarget{
    fn from(name: RefName)->Self{RefTarget::Symbolic(name)}
}
