use std::collections::HashSet;

use crate::{prelude::*, grid::shapes::iters::circle_circumference_iter_step::CircleCircumferenceIterStep};

#[derive(Debug, Clone)]
pub struct CircleCircumferenceIter {
    center: Coord,
    discovered: HashSet<Coord>,
    d: i32,
    x: i32,
    y: i32,
    step: CircleCircumferenceIterStep,
}

impl CircleCircumferenceIter {
    pub fn new(center: Coord, radius: u32) -> Self {
        Self {
            center,
            discovered: HashSet::new(),
            d: (5 - (radius as i32 * 4)) / 4,
            x: 0,
            y: radius as i32,
            step: CircleCircumferenceIterStep::Line1,
        }
    }

    fn try_next_point(&mut self) -> Option<Coord> {
        let c = match self.step {
            CircleCircumferenceIterStep::Line1 => {
                Some(Coord::new(self.center.x + self.x, self.center.y + self.y))
            }
            CircleCircumferenceIterStep::Line2 => {
                Some(Coord::new(self.center.x + self.x, self.center.y - self.y))
            }
            CircleCircumferenceIterStep::Line3 => {
                Some(Coord::new(self.center.x - self.x, self.center.y + self.y))
            }
            CircleCircumferenceIterStep::Line4 => {
                Some(Coord::new(self.center.x - self.x, self.center.y - self.y))
            }
            CircleCircumferenceIterStep::Line5 => {
                Some(Coord::new(self.center.x + self.y, self.center.y + self.x))
            }
            CircleCircumferenceIterStep::Line6 => {
                Some(Coord::new(self.center.x + self.y, self.center.y - self.x))
            }
            CircleCircumferenceIterStep::Line7 => {
                Some(Coord::new(self.center.x - self.y, self.center.y + self.x))
            }
            CircleCircumferenceIterStep::Line8 => {
                Some(Coord::new(self.center.x - self.y, self.center.y - self.x))
            }
            CircleCircumferenceIterStep::Process => {
                if self.d < 0 {
                    self.d += (2 * self.x) + 1;
                } else {
                    self.d += (2 * (self.x - self.y)) + 1;
                    self.y -= 1;
                }

                self.x += 1;

                if self.x > self.y {
                    self.step = CircleCircumferenceIterStep::Finished;
                    return None;
                }

                self.step = self.step.next();
                return self.try_next_point();
            }
            CircleCircumferenceIterStep::Finished => None,
        };

        self.step = self.step.next();

        c
    }
}

impl Iterator for CircleCircumferenceIter {
    type Item = Coord;
    fn next(&mut self) -> Option<Self::Item> {
        // Are we done?
        if self.step == CircleCircumferenceIterStep::Finished {
            return None;
        }

        // Get the next point
        if let Some(c) = self.try_next_point() {
            // Make sure it's unique
            if self.discovered.insert(c) {
                // It's new return it
                return Some(c);
            } else {
                // It's old, try the next one
                return self.next();
            }
        }

        // Couldn't get the next point
        None
    }
}
