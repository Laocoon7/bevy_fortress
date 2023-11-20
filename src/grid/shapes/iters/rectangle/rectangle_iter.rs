use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct RectangleIter {
    offset: Coord,
    max_offset: Coord,
    position: Coord,
}

impl RectangleIter {
    pub fn new(position: Coord, size: Size) -> Self {
        let max_offset = Coord::new(size.width() as i32, size.height() as i32);

        Self {
            offset: Coord::new(0, 0),
            max_offset,
            position,
        }
    }
}

impl Iterator for RectangleIter {
    type Item = Coord;
    fn next(&mut self) -> Option<Self::Item> {
        if self.offset.y >= self.max_offset.y {
            return None;
        }

        let p = self.offset;

        self.offset.x = self.offset.x + 1;

        if self.offset.x >= self.max_offset.x {
            self.offset.x = 0;
            self.offset.y = self.offset.y + 1;
        }

        Some(Coord::new(self.position.x + p.x, self.position.y + p.y))
    }
}
