use crate::Message;
use crate::core::git::consts::*;
use crate::utils::git::graph_layout::{GraphLayout, LayoutNode};
use crate::utils::git::hash::Hash;
use iced::alignment::Vertical;
use iced::widget::Canvas;
use iced::widget::canvas::{self, Cache, Geometry, LineCap, LineJoin, Path, Program, Stroke};
use iced::{Color, Element, Length, Pixels, Point, Rectangle, Renderer, Theme, mouse};
use std::collections::HashMap;

pub fn git_widget<'a>(layout: &'a GraphLayout) -> Element<'a, Message> {
    Canvas::new(GitGraphCanvas::new(layout))
        .width(Length::Fill)
        .height(Length::Fixed(
            TOP_PAD * 2.0 + layout.nodes.len() as f32 * ROW_H,
        ))
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

    fn point(&self, row: usize, lane: usize) -> Point {
        Point::new(
            LEFT_PAD + lane as f32 * LANE_W,
            TOP_PAD + row as f32 * ROW_H,
        )
    }

    fn node_point(&self, node: &LayoutNode) -> Point {
        self.point(node.row, node.lane)
    }

    fn edge_path(
        &self,
        from: Point,
        to: Point,
        source_lane: usize,
        from_lane: usize,
        to_lane: usize,
        target_lane: usize,
    ) -> Path {
        let source_track_x = LEFT_PAD + from_lane as f32 * LANE_W;
        let target_track_x = LEFT_PAD + to_lane as f32 * LANE_W;

        if source_lane == from_lane && from_lane == to_lane && to_lane == target_lane {
            return Path::line(from, to);
        }

        let vertical_direction = (to.y - from.y).signum();
        let exit_y = from.y + vertical_direction * (ROW_H * MERGE_APPROACH_FACTOR);
        let approach_y = to.y - vertical_direction * (ROW_H * MERGE_APPROACH_FACTOR);

        Path::new(|builder| {
            builder.move_to(from);

            if (from.y - exit_y).abs() > f32::EPSILON {
                builder.line_to(Point::new(from.x, exit_y));
            }

            if (from.x - source_track_x).abs() > f32::EPSILON {
                builder.line_to(Point::new(source_track_x, exit_y));
            }

            if (exit_y - approach_y).abs() > f32::EPSILON {
                builder.line_to(Point::new(source_track_x, approach_y));
            }

            if (source_track_x - target_track_x).abs() > f32::EPSILON {
                builder.line_to(Point::new(target_track_x, approach_y));
            }

            if (to.x - target_track_x).abs() > f32::EPSILON {
                builder.line_to(Point::new(target_track_x, to.y));
            }

            builder.line_to(to);
        })
    }
}

impl<'a, Message> Program<Message> for GitGraphCanvas<'a> {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let hash_map_nodes: HashMap<Hash, &LayoutNode> = self
            .layout
            .nodes
            .iter()
            .map(|node| (node.hash.clone(), node))
            .collect();

        let label_x = LEFT_PAD + self.layout.lane_count as f32 * LANE_W + LABEL_GAP;

        let edges = self.edge_cache.draw(renderer, bounds.size(), |frame| {
            for edge in &self.layout.edges {
                let Some(from_node) = hash_map_nodes.get(&edge.from) else {
                    continue;
                };
                let Some(to_node) = hash_map_nodes.get(&edge.to) else {
                    continue;
                };

                let from = self.node_point(from_node);
                let to = self.node_point(to_node);
                let path = self.edge_path(
                    from,
                    to,
                    from_node.lane,
                    edge.from_lane,
                    edge.to_lane,
                    to_node.lane,
                );

                frame.stroke(
                    &path,
                    Stroke::default()
                        .with_width(EDGE_WIDTH)
                        .with_color(EDGE_COLOR)
                        .with_line_cap(LineCap::Round)
                        .with_line_join(LineJoin::Round),
                );
            }
        });

        let nodes = self.node_cache.draw(renderer, bounds.size(), |frame| {
            for node in &self.layout.nodes {
                let p = self.node_point(node);

                frame.fill(&Path::circle(p, NODE_OUTLINE_R), NODE_OUTLINE_COLOR);
                frame.fill(&Path::circle(p, NODE_R), NODE_COLOR);

                let refs = if node.refs.is_empty() {
                    String::new()
                } else {
                    format!(
                        " [{}]",
                        node.refs
                            .iter()
                            .map(|r| r.as_str())
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                };

                frame.fill_text(canvas::Text {
                    content: format!(
                        "{}{}",
                        node.message.chars().take(20).collect::<String>(),
                        refs
                    ),
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
