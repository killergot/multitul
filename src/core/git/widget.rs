use crate::Message;
use crate::utils::git::GitGraph;
use crate::utils::git::provider::GitProvider;

use iced::Element;
use iced::widget::{text};

pub fn git_widget<'a>(graph: &GitGraph) -> Element<'a, Message> {
    iced::widget::column![text(format!("Init commit: {}", graph.init_node.0))].into()
}
