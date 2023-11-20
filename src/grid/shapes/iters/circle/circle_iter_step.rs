#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircleIterStep {
    Line1,
    Line2,
    Line3,
    Line4,
    Process,
    Finished,
}

impl CircleIterStep {
    pub fn next(self) -> Self {
        match self {
            CircleIterStep::Line1 => CircleIterStep::Line2,
            CircleIterStep::Line2 => CircleIterStep::Line3,
            CircleIterStep::Line3 => CircleIterStep::Line4,
            CircleIterStep::Line4 => CircleIterStep::Process,
            CircleIterStep::Process => CircleIterStep::Line1,
            CircleIterStep::Finished => CircleIterStep::Finished,
        }
    }
}
