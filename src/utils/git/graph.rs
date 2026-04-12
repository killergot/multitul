use std::collections::HashMap;

use crate::utils::git::commit::Commit;
use crate::utils::git::hash::Hash;

#[derive(Debug,Clone)]
pub struct Node{
    id: Hash,
    parents: Vec<Hash>,
    children: Vec<Hash>,
}

impl Node {
    pub fn new(id: Hash) -> Node {
        Node{id, parents: Vec::new(), children: Vec::new()}
    }

    pub fn parents(&self) -> &[Hash] {
        &self.parents
    }

    pub fn children(&self) -> &[Hash] {
        &self.children
    }
}

#[derive(Debug,Clone)]
pub struct GitGraph{
    pub nodes: HashMap<Hash, Node>,
    pub init_node: Hash,
}

impl GitGraph {
    pub fn new(commits: &HashMap<Hash, Commit>) -> Self {
        let mut checked_nodes = HashMap::<Hash,Node>::new();
        for (hash, commit) in commits {
            checked_nodes
                .entry(hash.clone())
                //.or_insert(Node::new(hash.clone())) не подходит, ибо она всегда вычисляет новую ноду
                // а .or_insert_with только при надобности
                .or_insert_with(|| Node::new(hash.clone()))
                .parents = commit.parent_hashes.clone();
            for parent in commit.parent_hashes.iter() {
                checked_nodes
                    .entry(parent.clone())
                    .or_insert_with(|| Node::new(parent.clone()))
                    .children
                    .push(hash.clone());
            }
        }


        // Search initional commit for buils started graph
        // Here can use FIRST and SINGLE node, because we have a DAG graph
        let init_node = checked_nodes
            .iter()
            .find(|(k,v)| v.parents.is_empty())
            .map(|(k,_)| k.clone())
            .expect("Не найден первый коммит");

        GitGraph{
            nodes: checked_nodes,
            init_node: init_node
        }
    }

    pub fn node(&self, hash: &Hash) -> Option<&Node> {
        self.nodes.get(hash)
    }

    pub fn children_of(&self, hash: &Hash) -> &[Hash] {
        self.node(hash).map(Node::children).unwrap_or(&[])
    }

    pub fn parents_of(&self, hash: &Hash) -> &[Hash] {
        self.node(hash).map(Node::parents).unwrap_or(&[])
    }
}
