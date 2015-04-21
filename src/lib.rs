pub type Float = f32;
pub type Point = [Float; 3];

/// Generic grid type.
pub trait Grid {
    /// Cell coordinate on the grid.
    type Coordinate;
    /// Direciton on the grid.
    type Direction;
    /// Get the center point of a cell.
    fn get_cell_center(&self, Self::Coordinate) -> Point;
    /// Get the closest coordinate of a point.
    fn get_coordinate(&self, &Point) -> Self::Coordinate;
    /// Get the cell neighbor in a direction.
    fn get_neighbour(&self, Self::Coordinate, Self::Direction) -> Self::Coordinate;
    /// Fold over all neighbours.
    fn fold_neighbours<U, F: Fn(U, Self::Coordinate) -> U>(&self, Self::Coordinate, U) -> U;
    /// Fold over all cells in a radius.
    fn fold_in_radius<U, F: Fn(U, Self::Coordinate) -> U>(&self, Self::Coordinate, Float, U) -> U;
}

/// Position on the grid.
pub struct Position<G: Grid> {
    /// Coordinate.
    pub coord: G::Coordinate,
    /// Direction.
    pub dir: G::Direction,
}
