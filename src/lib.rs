pub mod quad;

use std::fmt::Debug;

/// Scalar type to be used in `Point`.
pub type Float = f32;
/// A point in physical space.
pub type Point = [Float; 3];

/// Generic grid type.
pub trait Grid {
    /// Cell coordinate on the grid.
    type Coordinate: Copy + Debug + PartialEq;
    /// Direciton on the grid.
    type Direction: Copy + Debug + PartialEq;
    /// Get the center point of a cell.
    fn get_cell_center(&self, Self::Coordinate) -> Point;
    /// Get the closest coordinate of a point.
    fn get_coordinate(&self, &Point) -> Self::Coordinate;
    /// Get the cell neighbor in a direction.
    fn get_neighbour(&self, Self::Coordinate, Self::Direction) -> Self::Coordinate;
    /// Fold over all neighbours.
    fn fold_neighbours<U, F: Fn(U, Self::Coordinate, Self::Direction) -> U>(&self, Self::Coordinate, F, U) -> U;
    /// Fold over all cells in a radius.
    fn fold_in_radius<U, F: Fn(U, Self::Coordinate) -> U>(&self, Self::Coordinate, Float, F, U) -> U;
}

/// Position on the grid.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position<G: Grid> {
    /// Coordinate.
    pub coord: G::Coordinate,
    /// Direction.
    pub dir: G::Direction,
}
