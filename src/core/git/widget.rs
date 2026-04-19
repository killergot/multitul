use crate::Message;

use iced::{mouse, Element, Length, Rectangle, Renderer, Theme};
use iced::widget::canvas::{Cache, Geometry, Image, Program};
use iced::widget::{text, Canvas};
use crate::utils::git::graph_layout::GraphLayout;

pub fn git_widget<'a>(layout: &GraphLayout) -> Element<'a, Message> {
    Canvas::new(GitGraphCanvas::new(layout))
        .width(Length::Fixed(300.0))
        .height(Length::Fixed(180.0))
        .into()
}


#[derive(Debug)]
struct GitGraphCanvas<'a> {
    layout: &'a GraphLayout,
    edge_cache: Cache,
    node_cache: Cache,
}

impl<'a> GitGraphCanvas<'a> {
    fn new(layout: &'a GraphLayout) -> Self {
        Self {
            layout,
            edge_cache: Cache::new(),
            node_cache: Cache::new(),
        }
    }
}

impl <'a,Message> Program<Message> for GitGraphCanvas<'a> {
    type State = ();

    fn draw(
        &self,
        state: &Self::State,
        rendered: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        cursor: mouse::Cursor) -> Vec<Geometry<Message>> {

    }
}

