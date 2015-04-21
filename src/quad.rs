pub type Coordinate = [i32; 2];

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Grid {
    cell_size: [::Float; 2],
}

impl Grid {
    pub fn new(size: ::Float) -> Grid {
        Grid {
            cell_size: [size, size],
        }
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
    
    fn fold_neighbours<U, F: Fn(U, Coordinate, Direction) -> U>(&self, _: Coordinate, _: F, u: U) -> U {
        u //TODO
    }
    
    fn fold_in_radius<U, F: Fn(U, Coordinate) -> U>(&self, _: Coordinate, _: ::Float, _: F, u: U) -> U {
        u //TODO
    }
}
