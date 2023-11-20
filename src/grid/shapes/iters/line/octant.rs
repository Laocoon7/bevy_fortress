use crate::prelude::*;

/// An octant
#[derive(Debug, Clone)]
pub struct Octant(pub u8);

impl Octant {
    /// adapted from <http://codereview.stackexchange.com/a/95551>
    /// converts a `Coord` into a coordinate relative `Octant(0)` offset

    #[inline]
    pub fn to_offset(&self, position: Coord) -> (i32, i32) {
        match self.0 {
            0 => (position.x, position.y),
            1 => (position.y, position.x),
            2 => (position.y, -position.x),
            3 => (-position.x, position.y),
            4 => (-position.x, -position.y),
            5 => (-position.y, -position.x),
            6 => (-position.y, position.x),
            7 => (position.x, -position.y),
            _ => unreachable!(),
        }
    }

    /// converts from a `Octant(0)` relative coordinate into a `Coord`
    #[inline]
    #[allow(clippy::wrong_self_convention)]
    pub fn from_offset(&self, offset: (i32, i32)) -> Coord {
        let p = match self.0 {
            0 => (offset.0, offset.1),
            1 => (offset.1, offset.0),
            2 => (-offset.1, offset.0),
            3 => (-offset.0, offset.1),
            4 => (-offset.0, -offset.1),
            5 => (-offset.1, -offset.0),
            6 => (offset.1, -offset.0),
            7 => (offset.0, -offset.1),
            _ => unreachable!(),
        };
        Coord::new(p.0, p.1)
    }

    /// Creates a new `Octant` from two points
    #[inline(always)]
    pub fn new(position: Coord, other: Coord) -> Self {
        // adapted from <http://codereview.stackexchange.com/a/95551>
        let start = position;
        let end = other;

        let mut dx = end.x - start.x;
        let mut dy = end.y - start.y;
        let mut octant = 0;
        if dy < 0 {
            dx = -dx;
            dy = -dy;
            octant += 4;
        }
        if dx < 0 {
            let tmp = dx;
            dx = dy;
            dy = -tmp;
            octant += 2;
        }
        if dx < dy {
            octant += 1;
        }

        Self(octant)
    }
}
