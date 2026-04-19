use crate::Message;
use crate::utils::git::GitGraph;
use crate::utils::git::provider::GitProvider;

use iced::Element;
use iced::widget::text;
use crate::utils::git::graph_layout::GraphLayout;

pub fn git_widget<'a>(graph: &GraphLayout) -> Element<'a, Message> {
    iced::widget::column![text(format!("Init commit: {}", graph.nodes.last().unwrap().hash.0))].into()
}
