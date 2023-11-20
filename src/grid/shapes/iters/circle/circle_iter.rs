use std::collections::HashSet;

use crate::{prelude::*, grid::shapes::iters::{circle_iter_step::CircleIterStep, LineBresenhamIter}};

#[derive(Debug, Clone)]
pub struct CircleIter {
    center: Coord,
    discovered: HashSet<Coord>,
    d: i32,
    x: i32,
    y: i32,
    step: CircleIterStep,
    line: Option<LineBresenhamIter>,
}

impl CircleIter {
    pub fn new(center: Coord, radius: u32) -> Self {
        Self {
            center,
            discovered: HashSet::new(),
            d: (5 - (radius as i32 * 4)) / 4,
            x: 0,
            y: radius as i32,
            step: CircleIterStep::Line1,
            line: None,
        }
    }

    fn try_continue_line(&mut self) -> Option<Coord> {
        if let Some(line) = &mut self.line {
            // Does the line have a next point?
            if let Some(p) = line.next() {
                // Has this point been discovered?
                if self.discovered.insert(p) {
                    // It's new, return it
                    return Some(p);
                } else {
                    // Already seen this one, move on
                    return self.try_continue_line();
                }
            }
        }
        // No line / no more points
        None
    }

    fn try_next_line(&mut self) -> Option<LineBresenhamIter> {
        let line = match self.step {
            // Pick Line1
            CircleIterStep::Line1 => Some(
                Line::new(
                    Coord::new(self.center.x + self.x, self.center.y + self.y),
                    Coord::new(self.center.x + self.x, self.center.y - self.y),
                )
                .into_iter(),
            ),
            // Pick Line2
            CircleIterStep::Line2 => Some(
                Line::new(
                    Coord::new(self.center.x - self.x, self.center.y + self.y),
                    Coord::new(self.center.x - self.x, self.center.y - self.y),
                )
                .into_iter(),
            ),
            // Pick Line3
            CircleIterStep::Line3 => Some(
                Line::new(
                    Coord::new(self.center.x + self.y, self.center.y + self.x),
                    Coord::new(self.center.x + self.y, self.center.y - self.x),
                )
                .into_iter(),
            ),
            // Pick Line4
            CircleIterStep::Line4 => Some(
                Line::new(
                    Coord::new(self.center.x - self.y, self.center.y + self.x),
                    Coord::new(self.center.x - self.y, self.center.y - self.x),
                )
                .into_iter(),
            ),
            // Process for next lines
            CircleIterStep::Process => {
                if self.d < 0 {
                    self.d += (2 * self.x) + 1;
                } else {
                    self.d += (2 * (self.x - self.y)) + 1;
                    self.y -= 1;
                }
                self.x += 1;

                // Check if we are finished
                if self.x > self.y {
                    self.step = CircleIterStep::Finished;
                    return None;
                }

                // Go back to line1
                self.step = self.step.next();
                // try again
                return self.try_next_line();
            }
            // We shouldn't hit this ever
            CircleIterStep::Finished => None,
        };

        // Move to the next Line
        self.step = self.step.next();
        // Return this line
        line
    }
}

impl Iterator for CircleIter {
    type Item = Coord;
    fn next(&mut self) -> Option<Self::Item> {
        // Are we done?
        if self.step == CircleIterStep::Finished {
            return None;
        }

        // Are we in the middle of a line?
        if let Some(c) = self.try_continue_line() {
            return Some(c);
        }

        // Get the next line
        if let Some(line) = self.try_next_line() {
            // Set it as the active line
            self.line = Some(line);
            // try again
            return self.next();
        }

        // couldn't get the next line
        None
    }
}
