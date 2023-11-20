use crate::prelude::*;

pub trait ShapeGridExtensions<T> {
    /// Copy a shape off of the grid
    fn get_from_shape(&self, shape: impl Shape, unused_value: T) -> Self;
    /// Copy a shape onto the grid
    fn set_from_shape(&mut self, shape: impl Shape, value: T);
    /// Copy a shape off of the grid at an offset
    fn get_from_shape_offset(&self, shape: impl Shape, offset: Coord, unused_value: T) -> Self;
    /// Copy a shape onto the grid at an offset
    fn set_from_shape_offset(&mut self, shape: impl Shape, offset: Coord, value: T);

    /// Copy the border for a rectangle off of the grid
    fn get_from_rectangle_border(&self, rectangle: Rectangle, unused_value: T) -> Self;
    /// Copy the border for a rectangle onto the grid
    fn set_from_rectangle_border(&mut self, rectangle: Rectangle, value: T);
    /// Copy the border for a rectangle off of the grid at an offset
    fn get_from_rectangle_border_offset(
        &self,
        rectangle: Rectangle,
        offset: Coord,
        unused_value: T,
    ) -> Self;
    /// Copy the border for a rectangle onto the grid at an offset
    fn set_from_rectangle_border_offset(&mut self, rectangle: Rectangle, offset: Coord, value: T);

    /// Copy the circumference for a circle off of the grid
    fn get_from_circle_circumference(&self, circle: Circle, unused_value: T) -> Self;
    /// Copy the circumference for a circle onto the grid
    fn set_from_circle_circumference(&mut self, circle: Circle, value: T);
    /// Copy the circumference for a circle off of the grid at an offset
    fn get_from_circle_circumference_offset(
        &self,
        circle: Circle,
        offset: Coord,
        unused_value: T,
    ) -> Self;
    /// Copy the circumference for a circle onto the grid at an offset
    fn set_from_circle_circumference_offset(&mut self, circle: Circle, offset: Coord, value: T);

    /// Copy the line tunneling horizontal then vertical off of the grid
    fn get_from_line_tunnel_horizontal_vertical(&self, line: Line, unused_value: T) -> Self;
    /// Copy the line tunneling horizontal then vertical onto the grid
    fn set_from_line_tunnel_horizontal_vertical(&mut self, line: Line, value: T);
    /// Copy the line tunneling horizontal then vertical off of the grid at an offset
    fn get_from_line_tunnel_horizontal_vertical_offset(
        &self,
        line: Line,
        offset: Coord,
        unused_value: T,
    ) -> Self;
    /// Copy the line tunneling horizontal then vertical onto the grid at an offset
    fn set_from_line_tunnel_horizontal_vertical_offset(
        &mut self,
        line: Line,
        offset: Coord,
        value: T,
    );

    /// Copy the line tunneling vertical then horizontal off of the grid
    fn get_from_line_tunnel_vertical_horizontal(&self, line: Line, unused_value: T) -> Self;
    /// Copy the line tunneling vertical then horizontal onto the grid
    fn set_from_line_tunnel_vertical_horizontal(&mut self, line: Line, value: T);
    /// Copy the line tunneling vertical then horizontal off of the grid at an offset
    fn get_from_line_tunnel_vertical_horizontal_offset(
        &self,
        line: Line,
        offset: Coord,
        unused_value: T,
    ) -> Self;
    /// Copy the line tunneling vertical then horizontal onto the grid at an offset
    fn set_from_line_tunnel_vertical_horizontal_offset(
        &mut self,
        line: Line,
        offset: Coord,
        value: T,
    );
}

impl<T: Copy> ShapeGridExtensions<T> for Grid<T> {
    fn get_from_shape(&self, shape: impl Shape, unused_value: T) -> Self {
        let rectangle = shape.aabb();
        let mut grid = Grid::new_copy(rectangle.size, unused_value);

        shape.for_each(|coord| {
            let grid_position =
                Coord::new(coord.x - rectangle.left(), coord.y - rectangle.bottom());
            if let Some(grid_value) = self.get(coord) {
                *grid.get_checked_mut(grid_position) = *grid_value;
            }
        });

        grid
    }

    fn set_from_shape(&mut self, shape: impl Shape, value: T) {
        shape.for_each(|coord| {
            if let Some(grid_value) = self.get_mut(coord) {
                *grid_value = value;
            }
        });
    }

    fn get_from_shape_offset(&self, shape: impl Shape, offset: Coord, unused_value: T) -> Self {
        let rectangle = shape.aabb();
        let mut grid = Grid::new_copy(rectangle.size, unused_value);

        shape.for_each(|coord| {
            let grid_position =
                Coord::new(coord.x - rectangle.left(), coord.y - rectangle.bottom());
            let position = Coord::new(coord.x + offset.x, coord.y + offset.y);
            if let Some(grid_value) = self.get(position) {
                *grid.get_checked_mut(grid_position) = *grid_value;
            }
        });

        grid
    }

    fn set_from_shape_offset(&mut self, shape: impl Shape, offset: Coord, value: T) {
        shape.for_each(|coord| {
            let position = Coord::new(coord.x + offset.x, coord.y + offset.y);
            if let Some(grid_value) = self.get_mut(position) {
                *grid_value = value;
            }
        });
    }

    fn get_from_rectangle_border(&self, rectangle: Rectangle, unused_value: T) -> Self {
        let mut grid = Grid::new_copy(rectangle.size, unused_value);

        rectangle.for_each_border(|coord| {
            let grid_position =
                Coord::new(coord.x - rectangle.left(), coord.y - rectangle.bottom());
            if let Some(grid_value) = self.get(coord) {
                *grid.get_checked_mut(grid_position) = *grid_value;
            }
        });

        grid
    }

    fn set_from_rectangle_border(&mut self, rectangle: Rectangle, value: T) {
        rectangle.for_each_border(|coord| {
            if let Some(grid_value) = self.get_mut(coord) {
                *grid_value = value;
            }
        });
    }

    fn get_from_rectangle_border_offset(
        &self,
        rectangle: Rectangle,
        offset: Coord,
        unused_value: T,
    ) -> Self {
        let mut grid = Grid::new_copy(rectangle.size, unused_value);

        rectangle.for_each_border(|coord| {
            let grid_position =
                Coord::new(coord.x - rectangle.left(), coord.y - rectangle.bottom());
            let position = Coord::new(coord.x + offset.x, coord.y + offset.y);
            if let Some(grid_value) = self.get(position) {
                *grid.get_checked_mut(grid_position) = *grid_value;
            }
        });

        grid
    }

    fn set_from_rectangle_border_offset(&mut self, rectangle: Rectangle, offset: Coord, value: T) {
        rectangle.for_each_border(|coord| {
            let position = Coord::new(coord.x + offset.x, coord.y + offset.y);
            if let Some(grid_value) = self.get_mut(position) {
                *grid_value = value;
            }
        });
    }

    fn get_from_circle_circumference(&self, circle: Circle, unused_value: T) -> Self {
        let rectangle = circle.aabb();
        let mut grid = Grid::new_copy(rectangle.size, unused_value);

        circle.for_each_circumference(|coord| {
            let grid_position =
                Coord::new(coord.x - rectangle.left(), coord.y - rectangle.bottom());
            if let Some(grid_value) = self.get(coord) {
                *grid.get_checked_mut(grid_position) = *grid_value;
            }
        });

        grid
    }

    fn set_from_circle_circumference(&mut self, circle: Circle, value: T) {
        circle.for_each_circumference(|coord| {
            if let Some(grid_value) = self.get_mut(coord) {
                *grid_value = value;
            }
        });
    }

    fn get_from_circle_circumference_offset(
        &self,
        circle: Circle,
        offset: Coord,
        unused_value: T,
    ) -> Self {
        let rectangle = circle.aabb();
        let mut grid = Grid::new_copy(rectangle.size, unused_value);

        circle.for_each_circumference(|coord| {
            let grid_position =
                Coord::new(coord.x - rectangle.left(), coord.y - rectangle.bottom());
            let position = Coord::new(coord.x + offset.x, coord.y + offset.y);
            if let Some(grid_value) = self.get(position) {
                *grid.get_checked_mut(grid_position) = *grid_value;
            }
        });

        grid
    }

    fn set_from_circle_circumference_offset(&mut self, circle: Circle, offset: Coord, value: T) {
        circle.for_each_circumference(|coord| {
            let position = Coord::new(coord.x + offset.x, coord.y + offset.y);
            if let Some(grid_value) = self.get_mut(position) {
                *grid_value = value;
            }
        });
    }

    fn get_from_line_tunnel_horizontal_vertical(&self, line: Line, unused_value: T) -> Self {
        let rectangle = line.aabb();
        let mut grid = Grid::new_copy(rectangle.size, unused_value);

        line.for_each_tunnel_horizontal_vertical(|coord| {
            let grid_position =
                Coord::new(coord.x - rectangle.left(), coord.y - rectangle.bottom());
            if let Some(grid_value) = self.get(coord) {
                *grid.get_checked_mut(grid_position) = *grid_value;
            }
        });

        grid
    }

    fn set_from_line_tunnel_horizontal_vertical(&mut self, line: Line, value: T) {
        line.for_each_tunnel_horizontal_vertical(|coord| {
            if let Some(grid_value) = self.get_mut(coord) {
                *grid_value = value;
            }
        });
    }

    fn get_from_line_tunnel_horizontal_vertical_offset(
        &self,
        line: Line,
        offset: Coord,
        unused_value: T,
    ) -> Self {
        let rectangle = line.aabb();
        let mut grid = Grid::new_copy(rectangle.size, unused_value);

        line.for_each_tunnel_horizontal_vertical(|coord| {
            let grid_position =
                Coord::new(coord.x - rectangle.left(), coord.y - rectangle.bottom());
            let position = Coord::new(coord.x + offset.x, coord.y + offset.y);
            if let Some(grid_value) = self.get(position) {
                *grid.get_checked_mut(grid_position) = *grid_value;
            }
        });

        grid
    }

    fn set_from_line_tunnel_horizontal_vertical_offset(
        &mut self,
        line: Line,
        offset: Coord,
        value: T,
    ) {
        line.for_each_tunnel_horizontal_vertical(|coord| {
            let position = Coord::new(coord.x + offset.x, coord.y + offset.y);
            if let Some(grid_value) = self.get_mut(position) {
                *grid_value = value;
            }
        });
    }

    fn get_from_line_tunnel_vertical_horizontal(&self, line: Line, unused_value: T) -> Self {
        let rectangle = line.aabb();
        let mut grid = Grid::new_copy(rectangle.size, unused_value);

        line.for_each_tunnel_vertical_horizontal(|coord| {
            let grid_position =
                Coord::new(coord.x - rectangle.left(), coord.y - rectangle.bottom());
            if let Some(grid_value) = self.get(coord) {
                *grid.get_checked_mut(grid_position) = *grid_value;
            }
        });

        grid
    }

    fn set_from_line_tunnel_vertical_horizontal(&mut self, line: Line, value: T) {
        line.for_each_tunnel_vertical_horizontal(|coord| {
            if let Some(grid_value) = self.get_mut(coord) {
                *grid_value = value;
            }
        });
    }

    fn get_from_line_tunnel_vertical_horizontal_offset(
        &self,
        line: Line,
        offset: Coord,
        unused_value: T,
    ) -> Self {
        let rectangle = line.aabb();
        let mut grid = Grid::new_copy(rectangle.size, unused_value);

        line.for_each_tunnel_vertical_horizontal(|coord| {
            let grid_position =
                Coord::new(coord.x - rectangle.left(), coord.y - rectangle.bottom());
            let position = Coord::new(coord.x + offset.x, coord.y + offset.y);
            if let Some(grid_value) = self.get(position) {
                *grid.get_checked_mut(grid_position) = *grid_value;
            }
        });

        grid
    }

    fn set_from_line_tunnel_vertical_horizontal_offset(
        &mut self,
        line: Line,
        offset: Coord,
        value: T,
    ) {
        line.for_each_tunnel_vertical_horizontal(|coord| {
            let position = Coord::new(coord.x + offset.x, coord.y + offset.y);
            if let Some(grid_value) = self.get_mut(position) {
                *grid_value = value;
            }
        });
    }
}
