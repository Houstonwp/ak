use jiff::{Span, civil::Date};

#[derive(Debug, Clone)]
pub struct Schedule {
    pub start: Date,
    pub end: Date,
    pub frequency: Span,
}

impl Schedule {
    pub fn new(start: Date, end: Date, frequency: Span) -> Self {
        Schedule {
            start,
            end,
            frequency,
        }
    }
    pub fn from(mut self, start: Date) -> Self {
        self.start = start;
        self
    }
    pub fn to(mut self, end: Date) -> Self {
        self.end = end;
        self
    }
    pub fn by(mut self, frequency: Span) -> Self {
        self.frequency = frequency;
        self
    }
}
