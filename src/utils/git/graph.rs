use crate::Message;
use crate::utils::git::commit::Commit;
use crate::utils::git::hash::Hash;
use crate::utils::git::ref_name::RefName;
use crate::utils::git::repository::GitRepository;
use std::alloc::Layout;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct Node {
    id: Hash,
    parents: Vec<Hash>,
    children: Vec<Hash>,
}

impl Node {
    pub fn new(id: Hash) -> Node {
        Node {
            id,
            parents: Vec::new(),
            children: Vec::new(),
        }
    }
}

impl Node {
    pub fn id(&self) -> &Hash {
        &self.id
    }

    pub fn parents(&self) -> &Vec<Hash> {
        &self.parents
    }
    pub fn children(&self) -> &Vec<Hash> {
        &self.children
    }
}

#[derive(Debug, Clone)]
pub struct GitGraph {
    pub nodes: HashMap<Hash, Node>,
    pub root_nodes: Vec<Hash>,
}

impl GitGraph {
    pub fn new(commits: &HashMap<Hash, Commit>) -> Self {
        let mut checked_nodes = HashMap::<Hash, Node>::new();
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
        let init_node: Vec<Hash> = checked_nodes
            .iter()
            .filter(|(k, v)| v.parents.is_empty())
            .map(|(k, _)| k.clone())
            .collect();

        GitGraph {
            nodes: checked_nodes,
            root_nodes: init_node,
        }
    }

    pub fn topo_for_layout(&self, repo: &GitRepository) -> Vec<GraphNodeView> {
        let refs_map = repo.refs_by_hash();
        let start_hashes = start_hashes(self, repo);

        let mut visited = HashSet::<Hash>::new();
        let mut ordered_hashes = Vec::<Hash>::new();

        enum Frame {
            Enter(Hash),
            Exit(Hash),
        }

        let mut stack = Vec::<Frame>::new();

        for hash in start_hashes.iter().rev() {
            stack.push(Frame::Enter(hash.clone()));
        }

        while let Some(frame) = stack.pop() {
            match frame {
                Frame::Enter(hash) => {
                    if visited.contains(&hash) {
                        continue;
                    }

                    let node = match self.nodes.get(&hash) {
                        Some(node) => node,
                        None => continue,
                    };

                    visited.insert(hash.clone());

                    // Сначала кладём "выход" из вершины
                    stack.push(Frame::Exit(hash.clone()));

                    // Потом её соседей
                    for parent in node.parents.iter().rev() {
                        if !visited.contains(parent) {
                            stack.push(Frame::Enter(parent.clone()));
                        }
                    }
                }

                Frame::Exit(hash) => {
                    ordered_hashes.push(hash);
                }
            }
        }

        // Для topo через DFS обычно нужен reverse
        ordered_hashes.reverse();

        ordered_hashes
            .into_iter()
            .enumerate()
            .map(|(row, hash)| {
                let node = self.nodes.get(&hash).expect("node not found");

                GraphNodeView {
                    hash: hash.clone(),
                    row,
                    message: repo
                        .commits
                        .get(&hash)
                        .expect("not find commit")
                        .message
                        .clone(),
                    parents: node.parents.to_vec(),
                    refs: refs_map.get(&hash).cloned().unwrap_or_default(),
                }
            })
            .collect()
    }


    pub fn dfs_for_layout(&self, repo: &GitRepository) -> Vec<GraphNodeView> {
        let refs_map = repo.refs_by_hash();
        let start_hashes = start_hashes(&self, repo);

        let mut visited = HashSet::<Hash>::new();
        let mut ordered = Vec::<GraphNodeView>::new();
        let mut count = 0;

        let mut stack = Vec::new();

        for hash in start_hashes.iter().rev() {
            stack.push(hash.clone());
        }

        while let Some(hash) = stack.pop() {
            if !visited.insert(hash.clone()) {
                continue;
            };

            let node = match self.nodes.get(&hash) {
                Some(node) => node,
                None => continue,
            };
            ordered.push(GraphNodeView {
                hash: hash.clone(),
                row: count,
                message: repo
                    .commits
                    .get(&hash)
                    .expect("not find commit")
                    .message
                    .clone(),
                parents: node.parents.to_vec(),
                refs: refs_map.get(&hash).cloned().unwrap_or_default(),
            });
            count += 1;
            for parent in node.parents.iter().rev() {
                stack.push(parent.clone());
            }
        }
        ordered
    }

    pub fn bfs_for_layout(&self, repo: &GitRepository) -> Vec<GraphNodeView> {
        let mut ordered = Vec::<GraphNodeView>::new();

        ordered
    }
}

fn start_hashes(graph: &GitGraph, repo: &GitRepository) -> Vec<Hash> {
    let mut starts: Vec<Hash> = repo.refs_by_hash().into_keys().collect();

    // Вдруг нет refs, то по графу берем узлы без детей
    if starts.is_empty() {
        starts = graph
            .nodes
            .values()
            .filter(|node| node.children().is_empty())
            .map(|node| node.id().clone())
            .collect();
    }

    starts.sort_by(|a, b| a.0.cmp(&b.0));
    starts.dedup();
    starts
}

#[derive(Debug, Clone)]
pub struct GraphNodeView {
    pub hash: Hash,
    pub row: usize,
    pub parents: Vec<Hash>,
    pub message: String,
    pub refs: Vec<RefName>,
}
