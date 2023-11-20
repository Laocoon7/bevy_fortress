use std::fmt::{Debug, Display};

use serde::{Deserialize, Serialize};

use crate::{prelude::*, grid::shapes::iters::{RectangleBorderIter, RectangleIter}};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
/// Represents a Rectangle on a grid
pub struct Rectangle {
    pub position: Coord,
    pub size: Size,
}

// Constructors
impl Rectangle {
    /// Creates a new Rectangle with a known size
    pub fn from_size(position: Coord, size: Size) -> Self {
        Self { position, size }
    }

    /// Creates a new Rectangle with known corners
    pub fn from_corners(position0: Coord, position1: Coord) -> Self {
        Self::new(position0.x, position0.y, position1.x, position1.y)
    }

    /// Creates a new Rectangle
    pub fn new(x0: i32, y0: i32, x1: i32, y1: i32) -> Self {
        let min = Coord::new(x0.min(x1), y0.min(y1));
        let max = Coord::new(x0.max(x1), y0.max(y1));
        let size = Size::new(
            (max.x - min.x).abs() as u32 + 1,
            (max.y - min.y).abs() as u32 + 1,
        );
        Self::from_size(min, size)
    }
}

// Implementation
impl Rectangle {
    /// Get the size of the rectangle
    pub fn size(self) -> Size {
        self.size
    }

    /// Get the width of the rectangle
    pub fn width(self) -> u32 {
        self.size.width()
    }

    /// Get the height of the rectangle
    pub fn height(self) -> u32 {
        self.size.height()
    }

    /// Get the minimum Coord of the rectangle
    pub fn min(self) -> Coord {
        self.position
    }

    /// Get the maximum coord of the rectangle
    pub fn max(self) -> Coord {
        Coord::new(self.right(), self.top())
    }

    /// Get the left point of the rectangle
    pub fn left(self) -> i32 {
        self.position.x
    }

    /// Get the right point of the rectangle
    pub fn right(self) -> i32 {
        self.position.x + self.width() as i32 - 1
    }

    /// Get the top point of the rectangle
    pub fn top(self) -> i32 {
        self.position.y + self.height() as i32 - 1
    }

    /// Get the bottom point of the rectangle
    pub fn bottom(self) -> i32 {
        self.position.y
    }

    /// Get the center of the rectangle
    pub fn center(self) -> Coord {
        Coord::new(
            (self.right() + self.left()) / 2,
            (self.top() + self.bottom()) / 2,
        )
    }

    /// Determine if the rectangle is a square
    pub fn is_square(self) -> bool {
        self.width() == self.height()
    }

    /// Determine if one rectangle intersects another
    pub fn intersects(self, other: Self) -> bool {
        self.left() <= other.right()
            && self.right() >= other.left()
            && self.bottom() <= other.top()
            && self.top() >= other.bottom()
    }
}

// Iterators
impl Rectangle {
    /// Provides an iterator over the outer most border of the rectangle
    pub fn border_iter(self) -> RectangleBorderIter {
        RectangleBorderIter::new(self.position, self.size)
    }

    /// Calls `f` for each Coord in the border
    pub fn for_each_border<F: FnMut(Coord)>(self, mut f: F) {
        for coord in self.border_iter() {
            f(coord);
        }
    }
}

// Shape
impl Shape for Rectangle {
    fn for_each<F: FnMut(Coord)>(self, mut f: F) {
        for coord in self {
            f(coord);
        }
    }

    fn aabb(self) -> Rectangle {
        self
    }
}

impl IntoIterator for Rectangle {
    type IntoIter = RectangleIter;
    type Item = Coord;
    fn into_iter(self) -> Self::IntoIter {
        RectangleIter::new(self.position, self.size)
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        Self {
            position: Coord::new(0, 0),
            size: Size::new(1, 1),
        }
    }
}

impl Debug for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Rectangle {{ position: ({}, {}), size: ({}, {}) }}",
            self.position.x,
            self.position.y,
            self.size.width(),
            self.size.height()
        )
    }
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Rectangle {{\n\tposition: ({}, {}),\n\tsize: ({}, {})\n}}",
            self.position.x,
            self.position.y,
            self.size.width(),
            self.size.height()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn size() -> Size {
        Size::new(5, 10)
    }

    fn min() -> Coord {
        Coord::new(-2, 0)
    }

    fn max() -> Coord {
        Coord::new(2, 9)
    }

    fn rect_from_size() -> Rectangle {
        Rectangle::from_size(min(), size())
    }

    fn rect_from_corners() -> Rectangle {
        Rectangle::from_corners(min(), max())
    }

    fn rect_new() -> Rectangle {
        Rectangle::new(min().x, min().y, max().x, max().y)
    }

    #[test]
    fn test_from_size() {
        let rect = rect_from_size();
        assert_eq!(rect.right(), max().x);
        assert_eq!(rect.top(), max().y);
    }

    #[test]
    fn test_from_corners() {
        let rect = rect_from_corners();
        assert_eq!(rect.size(), size());
    }

    #[test]
    fn test_new() {
        let rect = rect_new();
        assert_eq!(rect.size(), size());
    }

    #[test]
    fn test_size() {
        let rect = rect_from_size();
        assert_eq!(rect.size(), size());
    }

    #[test]
    fn test_width() {
        let rect = rect_from_size();
        assert_eq!(rect.width(), size().width());
    }

    #[test]
    fn test_height() {
        let rect = rect_from_size();
        assert_eq!(rect.height(), size().height());
    }

    #[test]
    fn test_min() {
        let rect = rect_from_size();
        assert_eq!(rect.min(), min());
    }

    #[test]
    fn test_max() {
        let rect = rect_from_size();
        assert_eq!(rect.max(), max());
    }

    #[test]
    fn test_left() {
        let rect = rect_from_size();
        assert_eq!(rect.left(), min().x);
    }

    #[test]
    fn test_right() {
        let rect = rect_from_size();
        assert_eq!(rect.right(), max().x);
    }

    #[test]
    fn test_top() {
        let rect = rect_from_size();
        assert_eq!(rect.top(), max().y);
    }

    #[test]
    fn test_bottom() {
        let rect = rect_from_size();
        assert_eq!(rect.bottom(), min().y);
    }

    #[test]
    fn test_is_square() {
        let rect = Rectangle::new(0, 0, 10, 10);
        assert!(rect.is_square());

        let rect = Rectangle::new(0, 0, 10, 20);
        assert!(!rect.is_square());
    }

    #[test]
    fn test_intersects() {
        let rect1 = Rectangle::new(0, 0, 10, 10);
        let rect2 = Rectangle::new(5, 5, 15, 15);
        assert!(rect1.intersects(rect2));

        let rect1 = Rectangle::new(0, 0, 10, 10);
        let rect2 = Rectangle::new(11, 11, 20, 20);
        assert!(!rect1.intersects(rect2));
    }

    #[test]
    fn test_for_each() {
        let rect = Rectangle::new(0, 0, 1, 1);
        let mut points = Vec::new();
        rect.for_each(|point| points.push(point));
        assert_eq!(
            points,
            vec![
                Coord::new(0, 0),
                Coord::new(1, 0),
                Coord::new(0, 1),
                Coord::new(1, 1)
            ]
        );
    }

    #[test]
    fn test_into_iter() {
        let rect = Rectangle::new(0, 0, 1, 1);
        let points: Vec<Coord> = rect.into_iter().collect();
        assert_eq!(
            points,
            vec![
                Coord::new(0, 0),
                Coord::new(1, 0),
                Coord::new(0, 1),
                Coord::new(1, 1),
            ]
        )
    }
}
