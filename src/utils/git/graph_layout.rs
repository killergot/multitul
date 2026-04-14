use crate::utils::git::graph::GraphNodeView;
use crate::utils::git::hash::Hash;
use crate::utils::git::ref_name::RefName;
use iced::widget::text::base;
use std::cmp::PartialEq;
use std::collections::HashMap;

pub struct GraphLayout {
    pub nodes: Vec<LayoutNode>,
    pub edges: Vec<LayoutEdge>,
    pub lane_count: usize,
}

impl GraphLayout {
    pub fn new(nodes_input: &[GraphNodeView]) -> Self {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        let mut lane_count = 0;

        let mut active_lanes = Vec::<Option<Hash>>::new();

        for node in nodes_input.iter() {
            let mut first_free_lane = None;
            let mut base_lane = None;

            for (lane, hash) in active_lanes.iter().enumerate() {
                match hash {
                    Some(hash) => {
                        if hash == &node.hash {
                            base_lane = Some(lane)
                        };
                    }
                    None => {
                        first_free_lane = Some(lane);
                    }
                }
            }

            match base_lane {
                Some(lane) => {
                    active_lanes[lane] = None;
                }
                None => match first_free_lane {
                    Some(lane) => {
                        base_lane = Some(lane);
                        first_free_lane = None;
                    }
                    None => {
                        active_lanes.push(None);
                        base_lane = Some(active_lanes.len() - 1);
                    }
                },
            }

            nodes.push(LayoutNode {
                hash: node.hash.clone(),
                row: node.row,
                lane: base_lane.unwrap(),
                message: node.message.clone(),
                refs: node.refs.clone(),
            });

            if let Some(parent) = node.parents.first() {
                active_lanes[base_lane.unwrap()] = Some(parent.clone());
                edges.push(LayoutEdge {
                    from: parent.clone(),
                    to: node.hash.clone(),
                    from_lane: base_lane.unwrap(),
                    to_lane: base_lane.unwrap(),
                })
            }

            let mut new_base_lane = None;
            for parent in node.parents.iter().skip(1) {
                if let Some(lane) = first_free_lane {
                    new_base_lane = Some(lane);
                    active_lanes[lane] = Some(parent.clone());
                } else {
                    if let Some(i) = active_lanes.iter().position(Option::is_none) {
                        new_base_lane = Some(i);
                        active_lanes[i] = Some(parent.clone());
                    } else {
                        new_base_lane = Some(active_lanes.len());
                        active_lanes.push(Some(parent.clone()));
                    }
                }
                edges.push(LayoutEdge {
                    from: parent.clone(),
                    to: node.hash.clone(),
                    from_lane: new_base_lane.unwrap(),
                    to_lane: new_base_lane.unwrap(),
                })
            }
        }

        Self {
            nodes,
            edges,
            lane_count,
        }
    }
}

pub struct LayoutNode {
    pub hash: Hash,
    pub row: usize,
    pub lane: usize,
    pub message: String,
    pub refs: Vec<RefName>,
}

pub struct LayoutEdge {
    pub from: Hash,
    pub to: Hash,
    pub from_lane: usize,
    pub to_lane: usize,
}
