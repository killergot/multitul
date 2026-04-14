use crate::Message;
use crate::utils::git::GitGraph;
use crate::utils::git::provider::GitProvider;

use iced::Element;
use iced::widget::{text};

pub fn git_widget<'a>(graph: &GitGraph) -> Element<'a, Message> {
    iced::widget::column![text(format!("Init commit: {}", graph.root_nodesgit[0].0))].into()
}
