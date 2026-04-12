use crate::Message;
use crate::utils::git::GitGraph;
use crate::utils::git::provider::GitProvider;
use iced::Element;
use iced::widget::text;

pub fn git_widget<'a>() -> Element<'a, Message> {
    let mut provider = GitProvider::new();
    match provider.scan_repository() {
        Err(e) => panic!("{}", e),
        _ => {}
    }

    let graph = GitGraph::new(&provider.repository.commits);

    iced::widget::column![text(format!("Init commit: {}", graph.init_node.0))].into()
}
