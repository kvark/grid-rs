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
}
