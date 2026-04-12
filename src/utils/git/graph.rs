use std::collections::{HashMap, HashSet};
use crate::utils::git::commit::Commit;
use crate::utils::git::hash::Hash;
use crate::utils::git::ref_name::RefName;
use crate::utils::git::repository::Repository;


pub struct Node{
    id: Hash,
    parents: Vec<Hash>,
    children: Vec<Hash>,
}

impl Node {
    pub fn new(id: Hash) -> Node {
        Node{id, parents: Vec::new(), children: Vec::new()}
    }
}

pub struct GitGraph{
    nodes: HashMap<Hash, Node>,
    init_node: Hash,
}

impl GitGraph {
    pub fn new(commits: &HashMap<Hash, Commit>) -> Self {
        let mut checked_nodes = HashMap::<Hash,Node>::new();
        for (hash, commit) in commits {
            if !checked_nodes.contains_key(hash) {
                let mut new_node = Node::new(hash.clone());
                for i in commit.parent_hashes.iter() {
                    if !checked_nodes.contains_key(i) {
                        checked_nodes.insert(*i, Node::new(*i));
                    }
                    new_node.parents.push(*i);
                    checked_nodes.get_mut(i).expect("Не найдена нодка").children.push(hash.clone());
                }
                checked_nodes.insert(*hash, new_node);
            }
        }

        GitGraph{
            nodes:checked_nodes,
            init_node:
        }
    }
}