use std::collections::HashMap;
use crate::Message;

use iced::{mouse, Element, Length, Rectangle, Renderer, Theme, Point, Color, Pixels};
use iced::alignment::Vertical;
use iced::widget::canvas::{Cache, Geometry, Image, Path, Program, Stroke};
use iced::widget::{canvas, text, Canvas};
use crate::utils::git::graph_layout::{GraphLayout, LayoutNode};
use crate::utils::git::hash::Hash;

pub fn git_widget<'a>(layout: &'a GraphLayout) -> Element<'a, Message> {
    Canvas::new(GitGraphCanvas::new(layout))
        .width(Length::Fill)
        .height(Length::Fixed(TOP_PAD * 2.0 + layout.nodes.len() as f32 * ROW_H))
        .into()
}


#[derive(Debug)]
struct GitGraphCanvas<'a> {
    layout: &'a GraphLayout,
    edge_cache: Cache,
    node_cache: Cache,
}

const LEFT_PAD: f32 = 12.0;
const TOP_PAD: f32 = 12.0;
const LANE_W: f32 = 18.0;
const ROW_H: f32 = 26.0;
const NODE_R: f32 = 4.0;
const LABEL_GAP: f32 = 18.0;

const EDGE_COLOR : Color = Color::from_rgb8(110, 120, 140);
const NODE_COLOR : Color = Color::from_rgb8(235, 235, 235);
const TEXT_COLOR : Color = Color::from_rgb8(210, 210, 210);


impl<'a> GitGraphCanvas<'a> {
    fn new(layout: &'a GraphLayout) -> Self {
        Self {
            layout,
            edge_cache: Cache::new(),
            node_cache: Cache::new(),
        }
    }

    fn point(&self, row: usize, lane: usize) -> Point {
        Point::new(
            LEFT_PAD + lane as f32 * LANE_W,
            TOP_PAD + row as f32 * ROW_H,
        )
    }

    fn node_point(&self, node: &LayoutNode) -> Point {
        self.point(node.row, node.lane)
    }

}

impl <'a,Message> Program<Message> for GitGraphCanvas<'a> {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor
    ) -> Vec<Geometry> {
        let hash_map_nodes : HashMap<Hash, &LayoutNode> =
            self.layout.nodes.iter().map(|node| (node.hash.clone(), node)).collect();

        let label_x = LEFT_PAD + self.layout.lane_count as f32 * LANE_W + LABEL_GAP;

        let edges = self.edge_cache.draw(renderer, bounds.size(), |frame| {
            for edge in &self.layout.edges {
                let Some(from_node) = hash_map_nodes.get(&edge.from) else { continue; };
                let Some(to_node) = hash_map_nodes.get(&edge.to) else { continue; };

                let from = self.point(from_node.row, edge.from_lane);
                let to = self.point(to_node.row, edge.to_lane);

                let path = if edge.from_lane == edge.to_lane {
                    Path::line(from, to)
                } else {
                    let mid_y = (from.y + to.y) * 0.5;
                    Path::new(|b| {
                        b.move_to(from);
                        b.line_to(Point::new(from.x, mid_y));
                        b.line_to(Point::new(to.x, mid_y));
                        b.line_to(to);
                    })
                };

                frame.stroke(&path, Stroke::default().with_width(2.0).with_color(EDGE_COLOR));
            }
        });

        let nodes = self.node_cache.draw(renderer, bounds.size(), |frame| {
            for node in &self.layout.nodes {
                let p = self.node_point(node);

                frame.fill(&Path::circle(p, NODE_R), NODE_COLOR);

                let refs = if node.refs.is_empty() {
                    String::new()
                } else {
                    format!(
                        " [{}]",
                        node.refs.iter().map(|r| r.as_str()).collect::<Vec<_>>().join(", ")
                    )
                };

                frame.fill_text(canvas::Text {
                    content: format!("{}{}", node.message, refs),
                    position: Point::new(label_x, p.y),
                    color: TEXT_COLOR,
                    size: Pixels(14.0),
                    align_y: Vertical::Center,
                    ..canvas::Text::default()
                });
            }
        });
        vec![edges, nodes]
    }
}

