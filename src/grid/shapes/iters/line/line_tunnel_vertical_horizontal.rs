use std::ops::RangeInclusive;

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct LineTunnelVerticalHorizontalIter {
    start: Coord,
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>,
    end: Coord,
}

impl LineTunnelVerticalHorizontalIter {
    pub fn new(start: Coord, end: Coord) -> Self {
        Self {
            start,
            x: (start.x..=end.x).into_iter(),
            y: (start.y..=end.y).into_iter(),
            end,
        }
    }
}

impl Iterator for LineTunnelVerticalHorizontalIter {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(y) = self.y.next() {
            Some(Coord::new(self.start.x, y))
        } else if let Some(x) = self.x.next() {
            Some(Coord::new(x, self.end.y))
        } else {
            None
        }
    }
}
