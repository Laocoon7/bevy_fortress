use crate::{grid::shapes::iters::octant::Octant, prelude::*};

#[derive(Debug, Clone)]
pub struct LineBresenhamIter {
    abs_x: i32,
    abs_y: i32,
    end_x: i32,
    delta_step: i32,
    delta_x: i32,
    delta_y: i32,
    octant: Octant,
}

impl LineBresenhamIter {
    pub fn new(start: Coord, end: Coord) -> Self {
        let octant = Octant::new(start, end);

        let start_offset = octant.to_offset(start);
        let end_offset = octant.to_offset(end);

        let delta_x = end_offset.0 - start_offset.0;
        let delta_y = end_offset.1 - start_offset.1;

        Self {
            abs_x: start_offset.0,
            abs_y: start_offset.1,
            end_x: end_offset.0,
            delta_step: delta_y - delta_x,
            delta_x,
            delta_y,
            octant,
        }
    }

    pub fn advance(&mut self) -> Coord {
        let current_point = (self.abs_x, self.abs_y);
        if self.delta_step >= 0 {
            self.abs_y += 1;
            self.delta_step -= self.delta_x;
        }

        self.delta_step += self.delta_y;

        self.abs_x += 1;

        self.octant.from_offset(current_point)
    }
}

impl Iterator for LineBresenhamIter {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        if self.abs_x > self.end_x {
            None
        } else {
            Some(self.advance())
        }
    }
}
