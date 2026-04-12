use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

use iced::mouse;
use iced::widget::canvas::{self, Frame, Geometry, Path, Program, Stroke, Text};
use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Color, Element, Length, Pixels, Point, Rectangle, Renderer, Theme};

use crate::Message;
use crate::Screen;
use crate::utils::git::commit::Commit;
use crate::utils::git::{GitGraph, GitProvider, GitRef, Hash, RefName, RefTarget, Repository};

const LANE_SPACING: f32 = 28.0;
const ROW_HEIGHT: f32 = 30.0;
const TOP_PADDING: f32 = 20.0;
const LEFT_PADDING: f32 = 24.0;
const TEXT_OFFSET: f32 = 36.0;
const DOT_RADIUS: f32 = 5.0;

#[derive(Debug, Clone)]
pub struct GitGraphView {
    state: GitGraphState,
}

#[derive(Debug, Clone)]
enum GitGraphState {
    Ready(GraphLayout),
    Error(String),
}

#[derive(Debug, Clone)]
struct GraphLayout {
    rows: Vec<GraphRow>,
    lane_count: usize,
}

#[derive(Debug, Clone)]
struct GraphRow {
    title: String,
    lane: usize,
    before: Vec<Hash>,
    after: Vec<Hash>,
    children: Vec<Hash>,
    labels: Vec<String>,
}

impl GitGraphView {
    pub fn new() -> Self {
        let mut provider = GitProvider::new();
        let state = match provider.scan_repository() {
            Ok(()) => GraphLayout::from_repository(&provider.repository)
                .map(GitGraphState::Ready)
                .unwrap_or_else(GitGraphState::Error),
            Err(error) => GitGraphState::Error(format!("Git scan error: {error:?}")),
        };

        Self { state }
    }

    pub fn view(&self) -> Element<'_, Message> {
        match &self.state {
            GitGraphState::Ready(layout) => {
                let canvas_height = (layout.rows.len() as f32 * ROW_HEIGHT + TOP_PADDING * 2.0).max(240.0);
                let graph = canvas::Canvas::new(GraphCanvas {
                    layout: layout.clone(),
                })
                .width(Length::Fill)
                .height(canvas_height);

                let content = column![
                    row![
                        text("Git graph").size(28),
                        button("Go home").on_press(Message::SwitchTo(Screen::Main)),
                    ]
                    .spacing(16),
                    container(graph)
                        .width(Length::Fill)
                        .padding(12),
                ]
                .spacing(16)
                .padding(20);

                scrollable(content).into()
            }
            GitGraphState::Error(error) => column![
                text("Git graph").size(28),
                text(error),
                button("Go home").on_press(Message::SwitchTo(Screen::Main)),
            ]
            .spacing(16)
            .padding(20)
            .into(),
        }
    }
}

impl GraphLayout {
    fn from_repository(repository: &Repository) -> Result<Self, String> {
        if repository.commits.is_empty() {
            return Err("Repository has no commits".into());
        }

        let graph = GitGraph::new(&repository.commits);
        let refs_by_hash = build_refs_by_hash(repository);
        let (topo_order, generations) = topo_from_init(repository, &graph);
        let priority = build_priority_map(repository, &graph, &topo_order, &refs_by_hash);
        let mut rows = Vec::with_capacity(repository.commits.len());
        let mut active_lanes = vec![graph.init_node.clone()];
        let mut processed = HashSet::new();
        let mut lane_count = 1;

        while processed.len() < repository.commits.len() {
            let (lane, hash) = next_ready_lane(&active_lanes, &processed, repository, &generations)
                .or_else(|| inject_ready_commit(&mut active_lanes, &processed, repository, &generations, &priority))
                .ok_or_else(|| "Unable to find next commit from init_node".to_string())?;

            let before = active_lanes.clone();
            let commit = repository
                .commits
                .get(&hash)
                .ok_or_else(|| format!("Commit {} is missing in repository", hash.as_str()))?;
            let children = sorted_children(&graph, &hash, &priority);
            let mut after = before.clone();

            match children.as_slice() {
                [] => {
                    after.remove(lane);
                }
                [first, rest @ ..] => {
                    after[lane] = first.clone();
                    for (offset, child) in rest.iter().enumerate() {
                        after.insert(lane + 1 + offset, child.clone());
                    }
                }
            }

            dedup_hashes(&mut after);
            processed.insert(hash.clone());
            lane_count = lane_count.max(before.len()).max(after.len()).max(lane + 1);

            rows.push(GraphRow {
                title: format_commit_line(commit),
                lane,
                before,
                after: after.clone(),
                children,
                labels: refs_by_hash.get(&hash).cloned().unwrap_or_default(),
            });

            active_lanes = after;
        }

        Ok(Self { rows, lane_count })
    }
}

#[derive(Debug, Clone)]
struct GraphCanvas {
    layout: GraphLayout,
}

impl<Message> Program<Message> for GraphCanvas {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        let palette = palette();

        for (row_index, row) in self.layout.rows.iter().enumerate() {
            let y = TOP_PADDING + row_index as f32 * ROW_HEIGHT;

            for hash in row.before.iter().filter(|hash| lane_position(&row.after, hash).is_some()) {
                if let (Some(from_lane), Some(to_lane)) =
                    (lane_position(&row.before, hash), lane_position(&row.after, hash))
                {
                    let color = if from_lane == row.lane { palette.active } else { palette.branch };
                    draw_segment(&mut frame, from_lane, y, to_lane, y + ROW_HEIGHT, color);
                }
            }

            for child in &row.children {
                if let Some(child_lane) = lane_position(&row.after, child) {
                    draw_segment(&mut frame, row.lane, y, child_lane, y + ROW_HEIGHT, palette.active);
                }
            }
            draw_dot(&mut frame, row.lane, y, palette.commit);

            let label = if row.labels.is_empty() {
                row.title.clone()
            } else {
                format!("[{}] {}", row.labels.join(", "), row.title)
            };

            frame.fill_text(Text {
                content: label,
                position: Point::new(lane_x(self.layout.lane_count) + TEXT_OFFSET, y + 5.0),
                color: palette.text,
                size: Pixels(16.0),
                ..Text::default()
            });
        }

        vec![frame.into_geometry()]
    }
}

fn draw_segment(frame: &mut Frame, from_lane: usize, from_y: f32, to_lane: usize, to_y: f32, color: Color) {
    let path = Path::new(|builder| {
        builder.move_to(Point::new(lane_x(from_lane), from_y));
        builder.line_to(Point::new(lane_x(to_lane), to_y));
    });

    frame.stroke(
        &path,
        Stroke::default().with_color(color).with_width(2.0),
    );
}

fn draw_dot(frame: &mut Frame, lane: usize, y: f32, color: Color) {
    let dot = Path::circle(Point::new(lane_x(lane), y), DOT_RADIUS);
    frame.fill(&dot, color);
}

fn lane_x(lane: usize) -> f32 {
    LEFT_PADDING + lane as f32 * LANE_SPACING
}

fn format_commit_line(commit: &Commit) -> String {
    let short_hash: String = commit.hash.as_str().chars().take(7).collect();
    let message = commit.message.lines().next().unwrap_or("(no message)").trim();
    format!("{short_hash}  {message}")
}

fn build_refs_by_hash(repository: &Repository) -> HashMap<Hash, Vec<String>> {
    let mut refs_by_hash: HashMap<Hash, Vec<String>> = HashMap::new();

    for git_ref in repository.refs.values() {
        if let Some(hash) = resolve_ref_hash(git_ref, repository) {
            refs_by_hash
                .entry(hash)
                .or_default()
                .push(short_ref_name(git_ref.name.as_str()));
        }
    }

    if let Some(head) = &repository.head {
        if let Some(hash) = resolve_ref_hash(head, repository) {
            refs_by_hash.entry(hash).or_default().push("HEAD".into());
        }
    }

    refs_by_hash
}

fn short_ref_name(name: &str) -> String {
    name.rsplit('/').next().unwrap_or(name).to_string()
}

fn lane_position(lanes: &[Hash], target: &Hash) -> Option<usize> {
    lanes.iter().position(|hash| hash == target)
}

fn dedup_hashes(hashes: &mut Vec<Hash>) {
    let mut seen = HashSet::new();
    hashes.retain(|hash| seen.insert(hash.clone()));
}

fn next_ready_lane(
    active_lanes: &[Hash],
    processed: &HashSet<Hash>,
    repository: &Repository,
    generations: &HashMap<Hash, usize>,
) -> Option<(usize, Hash)> {
    active_lanes
        .iter()
        .enumerate()
        .filter(|(_, hash)| !processed.contains(*hash) && is_ready(hash, processed, repository))
        .min_by(|(left_lane, left_hash), (right_lane, right_hash)| {
            compare_generation(left_hash, right_hash, generations).then_with(|| left_lane.cmp(right_lane))
        })
        .map(|(lane, hash)| (lane, hash.clone()))
}

fn inject_ready_commit(
    active_lanes: &mut Vec<Hash>,
    processed: &HashSet<Hash>,
    repository: &Repository,
    generations: &HashMap<Hash, usize>,
    priority: &HashMap<Hash, i64>,
) -> Option<(usize, Hash)> {
    let mut candidates: Vec<Hash> = repository
        .commits
        .keys()
        .filter(|hash| !processed.contains(*hash) && is_ready(hash, processed, repository))
        .cloned()
        .collect();
    candidates.sort_by(|left, right| {
        compare_generation(left, right, generations).then_with(|| compare_priority(left, right, priority))
    });

    let hash = candidates.into_iter().next()?;
    active_lanes.push(hash.clone());
    Some((active_lanes.len() - 1, hash))
}

fn is_ready(hash: &Hash, processed: &HashSet<Hash>, repository: &Repository) -> bool {
    repository
        .commits
        .get(hash)
        .map(|commit| commit.parent_hashes.iter().all(|parent| processed.contains(parent)))
        .unwrap_or(false)
}

fn sorted_children(graph: &GitGraph, hash: &Hash, priority: &HashMap<Hash, i64>) -> Vec<Hash> {
    let mut children = graph.children_of(hash).to_vec();
    children.sort_by(|left, right| compare_priority(left, right, priority));
    children
}

fn compare_generation(left: &Hash, right: &Hash, generations: &HashMap<Hash, usize>) -> Ordering {
    generations
        .get(left)
        .copied()
        .unwrap_or(usize::MAX)
        .cmp(&generations.get(right).copied().unwrap_or(usize::MAX))
        .then_with(|| left.as_str().cmp(right.as_str()))
}

fn compare_priority(left: &Hash, right: &Hash, priority: &HashMap<Hash, i64>) -> Ordering {
    priority
        .get(right)
        .copied()
        .unwrap_or_default()
        .cmp(&priority.get(left).copied().unwrap_or_default())
        .then_with(|| left.as_str().cmp(right.as_str()))
}

fn topo_from_init(repository: &Repository, graph: &GitGraph) -> (Vec<Hash>, HashMap<Hash, usize>) {
    let mut indegree: HashMap<Hash, usize> = repository
        .commits
        .keys()
        .cloned()
        .map(|hash| {
            let parents = graph
                .parents_of(&hash)
                .iter()
                .filter(|parent| repository.commits.contains_key(*parent))
                .count();
            (hash, parents)
        })
        .collect();
    let mut ready = vec![graph.init_node.clone()];
    let mut ordered = Vec::with_capacity(repository.commits.len());
    let mut seen = HashSet::new();
    let mut generations = HashMap::new();
    generations.insert(graph.init_node.clone(), 0);

    while !ready.is_empty() {
        ready.sort_by(|left, right| compare_generation(left, right, &generations));
        let hash = ready.remove(0);
        if !seen.insert(hash.clone()) {
            continue;
        }

        ordered.push(hash.clone());
        let current_generation = generations.get(&hash).copied().unwrap_or_default();

        for child in graph.children_of(&hash) {
            let next_generation = current_generation + 1;
            generations
                .entry(child.clone())
                .and_modify(|generation| *generation = (*generation).max(next_generation))
                .or_insert(next_generation);

            if let Some(count) = indegree.get_mut(child) {
                *count = count.saturating_sub(1);
                if *count == 0 {
                    ready.push(child.clone());
                }
            }
        }
    }

    for hash in repository.commits.keys() {
        if seen.insert(hash.clone()) {
            generations.entry(hash.clone()).or_insert(usize::MAX / 2);
            ordered.push(hash.clone());
        }
    }

    (ordered, generations)
}

fn build_priority_map(
    repository: &Repository,
    graph: &GitGraph,
    topo_order: &[Hash],
    refs_by_hash: &HashMap<Hash, Vec<String>>,
) -> HashMap<Hash, i64> {
    let head_hash = repository.head.as_ref().and_then(|head| resolve_ref_hash(head, repository));
    let head_ancestors = head_hash
        .as_ref()
        .map(|hash| collect_ancestors(hash, repository))
        .unwrap_or_default();
    let mut scores = HashMap::new();

    for hash in topo_order.iter().rev() {
        let children_score = graph
            .children_of(hash)
            .iter()
            .map(|child| 1 + scores.get(child).copied().unwrap_or_default())
            .sum::<i64>();
        let refs_score = refs_by_hash
            .get(hash)
            .map(|labels| labels.len() as i64)
            .unwrap_or_default()
            * 10_000;
        let head_score = if head_ancestors.contains(hash) { 1_000_000 } else { 0 };
        scores.insert(hash.clone(), head_score + refs_score + children_score);
    }

    scores
}

fn collect_ancestors(start: &Hash, repository: &Repository) -> HashSet<Hash> {
    let mut seen = HashSet::new();
    let mut stack = vec![start.clone()];

    while let Some(hash) = stack.pop() {
        if !seen.insert(hash.clone()) {
            continue;
        }
        if let Some(commit) = repository.commits.get(&hash) {
            for parent in &commit.parent_hashes {
                stack.push(parent.clone());
            }
        }
    }

    seen
}

fn resolve_ref_hash(git_ref: &GitRef, repository: &Repository) -> Option<Hash> {
    match &git_ref.target {
        RefTarget::Direct(hash) => Some(hash.clone()),
        RefTarget::Symbolic(name) => resolve_ref_by_name(name.clone(), repository),
    }
}

fn resolve_ref_by_name(name: RefName, repository: &Repository) -> Option<Hash> {
    repository.refs.get(&name).and_then(|git_ref| match &git_ref.target {
        RefTarget::Direct(hash) => Some(hash.clone()),
        RefTarget::Symbolic(next) => resolve_ref_by_name(next.clone(), repository),
    })
}

struct Palette {
    active: Color,
    branch: Color,
    commit: Color,
    text: Color,
}

fn palette() -> Palette {
    Palette {
        active: Color::from_rgb8(0x6E, 0xC2, 0x7E),
        branch: Color::from_rgb8(0x7A, 0x84, 0x92),
        commit: Color::from_rgb8(0xF5, 0xB9, 0x42),
        text: Color::from_rgb8(0xE7, 0xEC, 0xF3),
    }
}
