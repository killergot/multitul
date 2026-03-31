#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Mark {
    Absent,
    Present,
    Correct,
    #[default]
    Unknown,
}

