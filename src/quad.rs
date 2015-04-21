pub type Coordinate = [i32; 2];

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Debug)]
pub struct Grid {
    cell_size: [::Float; 2],
}

impl Grid {
    /// Create a new grid.
    pub fn new(size: ::Float) -> Grid {
        Grid {
            cell_size: [size, size],
        }
    }
    
    /// Get all the corners of a given cell, starting with
    /// North-West and going clockwise
    pub fn get_corners(&self, c: Coordinate) -> [::Point; 4] {
        use ::Grid;
        let ext = [self.cell_size[0] * 0.5, self.cell_size[1] * 0.5];
        let center = self.get_cell_center(c);
        [[center[0]-ext[0], center[1]-ext[1], 0.0],
         [center[0]+ext[0], center[1]-ext[1], 0.0],
         [center[0]+ext[0], center[1]+ext[1], 0.0],
         [center[0]-ext[0], center[1]+ext[1], 0.0]]
    }

    /// Fold all of the edges in the area.
    pub fn fold_edges_in_area<U,
        F: Fn(U, ::Point, ::Point, Coordinate, Direction) -> U
    >(  &self, area: &::Area, u: U, fun: F) -> U
    {
        use ::Grid;
        self.fold_in_area(area, u, |mut u, c| {
            let corners = self.get_corners(c);
            u = fun(u, corners[0], corners[1], c, Direction::North);
            u = fun(u, corners[1], corners[2], c, Direction::East);
            u
        })
    }
}

impl ::Grid for Grid {
    type Coordinate = Coordinate;
    type Direction = Direction;
    
    fn get_cell_center(&self, c: Coordinate) -> ::Point {
        [(c[0] as f32 + 0.5) * self.cell_size[0],
         (c[1] as f32 + 0.5) * self.cell_size[1],
         0.0]
    }
    
    fn get_coordinate(&self, p: &::Point) -> Coordinate {
        [(p[0] / self.cell_size[0]) as i32,
         (p[1] / self.cell_size[1]) as i32]
    }
    
    fn get_neighbour(&self, c: Coordinate, d: Direction) -> Coordinate {
        match d {
            Direction::North => [c[0], c[1]+1],
            Direction::East  => [c[0]+1, c[1]],
            Direction::South => [c[0], c[1]-1],
            Direction::West  => [c[0]-1, c[1]],
        }
    }
    
    fn get_edge(&self, c: Coordinate, d: Direction) -> [::Point; 2] {
        let corners = self.get_corners(c);
        match d {
            Direction::North => [corners[0], corners[1]],
            Direction::East  => [corners[1], corners[2]],
            Direction::South => [corners[2], corners[3]],
            Direction::West  => [corners[3], corners[0]],
        }
    }
    
    fn fold_neighbours<U, F: Fn(U, Coordinate, Direction) -> U>(&self, coord: Coordinate, u: U, fun: F) -> U {
        [Direction::North, Direction::East, Direction::South, Direction::West]
            .iter().fold(u, |u, &d| fun(u, self.get_neighbour(coord, d), d))
    }
    
    fn fold_in_radius<U, F: Fn(U, Coordinate) -> U>(&self, coord: Coordinate, radius: ::Float, mut u: U, fun: F) -> U {
        let ext_x = (radius / self.cell_size[0]) as i32;
        let ext_y = (radius / self.cell_size[1]) as i32;
        for x in coord[0] - ext_x .. coord[0] + ext_x + 1 {
            for y in coord[1] - ext_y .. coord[1] + ext_y + 1 {
                let dist = self.get_distance(coord, [x, y]);
                if dist <= radius {
                    u = fun(u, [x, y]);
                }
            }
        }
        u
    }

    fn fold_in_area<U, F: Fn(U, Coordinate) -> U>(&self, area: &::Area, mut u: U, fun: F) -> U {
        let min_c = self.get_coordinate(&area[0]);
        let max_c = self.get_coordinate(&area[1]);
        for x in min_c[0] .. max_c[0]+1 {
            for y in min_c[1] .. max_c[1] + 1 {
                u = fun(u, [x,y]);
            }
        }
        u
    }
}
