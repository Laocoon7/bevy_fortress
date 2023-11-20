use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct RectangleBorderIter {
    offset: Coord,
    max_offset: Coord,
    position: Coord,
}

impl RectangleBorderIter {
    pub fn new(position: Coord, size: Size) -> Self {
        let max_offset = Coord::new(
            position.x + size.width() as i32,
            position.y + size.height() as i32,
        );

        Self {
            offset: Coord::new(0, 0),
            max_offset,
            position,
        }
    }
}

impl Iterator for RectangleBorderIter {
    type Item = Coord;
    fn next(&mut self) -> Option<Self::Item> {
        if self.offset.y >= self.max_offset.y {
            return None;
        }

        let p = self.offset;

        // Fill in top and bottom lines
        if self.offset.y == 0 || self.offset.y == self.max_offset.y - 1 {
            self.offset.x = self.offset.x + 1;

            if self.offset.x >= self.max_offset.x {
                self.offset.x = 0;
                self.offset.y = self.offset.y + 1;
            }
        } else {
            if self.offset.x == 0 {
                self.offset.x = self.max_offset.x - 1;
            } else {
                self.offset.x = 0;
                self.offset.y = self.offset.y + 1;
            }
        }

        Some(Coord::new(self.position.x + p.x, self.position.y + p.y))
    }
}
