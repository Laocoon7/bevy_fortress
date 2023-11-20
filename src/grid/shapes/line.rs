use std::fmt::{Debug, Display};

use serde::{Deserialize, Serialize};

use crate::{prelude::*, grid::shapes::iters::{LineTunnelHorizontalVerticalIter, LineTunnelVerticalHorizontalIter, LineBresenhamIter}};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
/// Represents a Line on a grid
pub struct Line {
    pub start: Coord,
    pub end: Coord,
}

// Constructors
impl Line {
    /// Creates a new Line
    pub fn new(start: Coord, end: Coord) -> Self {
        Self { start, end }
    }
}

// Implementation
impl Line {
    /// Get the total length of the line
    pub fn len(self) -> u32 {
        (self.end.x - self.start.x)
            .abs()
            .max((self.end.y - self.start.y).abs()) as u32
            + 1
    }

    /// Get the starting point of the line
    pub fn start(self) -> Coord {
        self.start
    }

    /// Get the ending point of the line
    pub fn end(self) -> Coord {
        self.end
    }
}

// Iterator
impl Line {
    /// Provides an iterator over a line horizontal and then vertical reaching the ending
    pub fn tunnel_horizontal_vertical_iter(self) -> LineTunnelHorizontalVerticalIter {
        LineTunnelHorizontalVerticalIter::new(self.start, self.end)
    }

    /// Provides an iterator over a line vertical and then horizontal reaching the ending
    pub fn tunnel_vertical_horizontal_iter(self) -> LineTunnelVerticalHorizontalIter {
        LineTunnelVerticalHorizontalIter::new(self.start, self.end)
    }

    /// Calls `f` for each Coord in the tunnel
    pub fn for_each_tunnel_horizontal_vertical<F: FnMut(Coord)>(self, mut f: F) {
        for coord in self.tunnel_horizontal_vertical_iter() {
            f(coord);
        }
    }

    /// Calls `f` for each Coord in the tunnel
    pub fn for_each_tunnel_vertical_horizontal<F: FnMut(Coord)>(self, mut f: F) {
        for coord in self.tunnel_vertical_horizontal_iter() {
            f(coord);
        }
    }
}

// Shape
impl Shape for Line {
    fn for_each<F: FnMut(Coord)>(self, mut f: F) {
        for coord in self {
            f(coord);
        }
    }

    fn aabb(self) -> Rectangle {
        Rectangle::from_corners(self.start, self.end)
    }
}

impl IntoIterator for Line {
    type IntoIter = LineBresenhamIter;
    type Item = Coord;
    fn into_iter(self) -> Self::IntoIter {
        LineBresenhamIter::new(self.start, self.end)
    }
}

impl Default for Line {
    fn default() -> Self {
        Self {
            start: Coord::new(0, 0),
            end: Coord::new(1, 0),
        }
    }
}

impl Debug for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Line {{ start: ({}, {}), end: ({}, {}) }}",
            self.start.x, self.start.y, self.end.x, self.end.y,
        )
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Line {{\n\tstart: ({}, {}),\n\tend: ({}, {}),\n}}",
            self.start.x, self.start.y, self.end.x, self.end.y,
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn start() -> Coord {
        Coord::new(0, 0)
    }

    fn end() -> Coord {
        Coord::new(2, 2)
    }

    fn len() -> u32 {
        3
    }

    fn line_new() -> Line {
        Line::new(start(), end())
    }

    #[test]
    fn test_new() {
        let line = line_new();
        assert_eq!(line.start(), start());
        assert_eq!(line.end(), end());
    }

    #[test]
    fn test_len() {
        let line = line_new();
        assert_eq!(line.len(), len())
    }

    #[test]
    fn test_for_each() {
        let line = line_new();
        let mut points = Vec::new();
        line.for_each(|point| points.push(point));
        assert_eq!(
            points,
            vec![Coord::new(0, 0), Coord::new(1, 1), Coord::new(2, 2)]
        );
    }

    #[test]
    fn test_into_iter() {
        let line = line_new();
        let points: Vec<Coord> = line.into_iter().collect();
        assert_eq!(
            points,
            vec![Coord::new(0, 0), Coord::new(1, 1), Coord::new(2, 2)]
        );
    }
}
