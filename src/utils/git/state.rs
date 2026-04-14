use crate::utils::git::GitGraph;
use crate::utils::git::provider::GitProvider;
use crate::utils::git::repository::GitRepository;

#[derive(Debug, Clone)]
pub struct GitState {
    pub graph: GitGraph,
    pub repo: GitRepository,
}
