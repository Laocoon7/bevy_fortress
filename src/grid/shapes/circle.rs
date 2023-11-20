use std::fmt::{Debug, Display};

use serde::{Deserialize, Serialize};

use crate::{prelude::*, grid::shapes::iters::{CircleCircumferenceIter, CircleIter}};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
/// Represents a Circle on a grid
pub struct Circle {
    pub center: Coord,
    pub radius: u32,
}

// Constructors
impl Circle {
    /// Creates a new circle.
    pub fn new(center: Coord, radius: u32) -> Self {
        Self { center, radius }
    }
}

// Imnplementation
impl Circle {
    /// Get the center of the circle
    pub const fn center(self) -> Coord {
        self.center
    }

    /// Get the left point of the circle
    pub fn left(self) -> Coord {
        Coord::new(self.center.x - self.radius as i32, self.center.y)
    }

    /// Get the right point of the circle
    pub fn right(self) -> Coord {
        Coord::new(self.center.x + self.radius as i32, self.center.y)
    }

    /// Get the top point of the circle
    pub fn top(self) -> Coord {
        Coord::new(self.center.x, self.center.y + self.radius as i32)
    }

    /// Get the bottom point of the circle
    pub fn bottom(self) -> Coord {
        Coord::new(self.center.x, self.center.y - self.radius as i32)
    }

    /// Get the number of cells inside the circle
    pub fn get_count(self) -> u32 {
        self.into_iter().count() as u32
    }

    /// Determine if a position is inside the circle
    pub fn contains(self, position: Coord) -> bool {
        self.into_iter().find(|&c| c == position).is_some()
    }
}

// Iterators
impl Circle {
    /// Provides an iterator over the outer most ring of cells
    pub fn circumference_iter(self) -> CircleCircumferenceIter {
        CircleCircumferenceIter::new(self.center, self.radius)
    }

    /// Calls `f` for each Coord in the circumference
    pub fn for_each_circumference<F: FnMut(Coord)>(self, mut f: F) {
        for coord in self.circumference_iter() {
            f(coord);
        }
    }
}

// Shape
impl Shape for Circle {
    fn for_each<F: FnMut(Coord)>(self, mut f: F) {
        for coord in self {
            f(coord);
        }
    }

    fn aabb(self) -> Rectangle {
        Rectangle::new(
            self.center.x - self.radius as i32,
            self.center.y - self.radius as i32,
            self.center.x + self.radius as i32,
            self.center.y + self.radius as i32,
        )
    }
}

impl IntoIterator for Circle {
    type IntoIter = CircleIter;
    type Item = Coord;
    fn into_iter(self) -> Self::IntoIter {
        CircleIter::new(self.center, self.radius)
    }
}

impl Default for Circle {
    fn default() -> Self {
        Self {
            center: Coord::new(0, 0),
            radius: 1,
        }
    }
}

impl Debug for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Circle {{ center: ({}, {}), radius: {} }}",
            self.center.x, self.center.y, self.radius
        )
    }
}

impl Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Circle {{\n\tcenter: ({}, {}),\n\tradius: {},\n}}",
            self.center.x, self.center.y, self.radius
        )
    }
}
