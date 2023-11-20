#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircleCircumferenceIterStep {
    Line1,
    Line2,
    Line3,
    Line4,
    Line5,
    Line6,
    Line7,
    Line8,
    Process,
    Finished,
}

impl CircleCircumferenceIterStep {
    pub fn next(self) -> Self {
        match self {
            CircleCircumferenceIterStep::Line1 => CircleCircumferenceIterStep::Line2,
            CircleCircumferenceIterStep::Line2 => CircleCircumferenceIterStep::Line3,
            CircleCircumferenceIterStep::Line3 => CircleCircumferenceIterStep::Line4,
            CircleCircumferenceIterStep::Line4 => CircleCircumferenceIterStep::Line5,
            CircleCircumferenceIterStep::Line5 => CircleCircumferenceIterStep::Line6,
            CircleCircumferenceIterStep::Line6 => CircleCircumferenceIterStep::Line7,
            CircleCircumferenceIterStep::Line7 => CircleCircumferenceIterStep::Line8,
            CircleCircumferenceIterStep::Line8 => CircleCircumferenceIterStep::Process,
            CircleCircumferenceIterStep::Process => CircleCircumferenceIterStep::Line1,
            CircleCircumferenceIterStep::Finished => CircleCircumferenceIterStep::Finished,
        }
    }
}
