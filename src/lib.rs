pub mod quad;

use std::fmt::Debug;

/// Scalar type to be used in `Point`.
pub type Float = f32;
/// A point in physical space.
pub type Point = [Float; 3];
/// Cubic area in physical space.
pub type Area = [Point; 2];

/// Generic grid type.
pub trait Grid {
    /// Cell coordinate on the grid.
    type Coordinate: Copy + Debug + PartialEq;
    /// Direciton on the grid.
    type Direction: Copy + Debug + PartialEq;
    /// Get the center point of a cell.
    fn get_cell_center(&self, Self::Coordinate) -> Point;
    /// Get the distance between two cell centers.
    fn get_distance(&self, a: Self::Coordinate, b: Self::Coordinate) -> Float {
        if a != b {
            let pa = self.get_cell_center(a);
            let pb = self.get_cell_center(b);
            (pa[0]*pb[0] + pa[1]*pb[1] + pa[2]*pb[2]).sqrt()
        }else { 0.0 }
    }
    /// Get the closest coordinate of a point.
    fn get_coordinate(&self, &Point) -> Self::Coordinate;
    /// Get the cell neighbor in a direction.
    fn get_neighbour(&self, Self::Coordinate, Self::Direction) -> Self::Coordinate;
    /// Get an edge in a given direction.
    fn get_edge(&self, Self::Coordinate, Self::Direction) -> [Point; 2];
    /// Fold over all neighbours.
    fn fold_neighbours<U, F: Fn(U, Self::Coordinate, Self::Direction) -> U>(&self, Self::Coordinate, U, F) -> U;
    /// Fold over all cells in a radius.
    fn fold_in_radius<U, F: Fn(U, Self::Coordinate) -> U>(&self, Self::Coordinate, Float, U, F) -> U;
    /// Fold over a cubic area.
    fn fold_in_area<U, F: Fn(U, Self::Coordinate) -> U>(&self, &Area, U, F) -> U;
}

/// Position on the grid.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position<G: Grid> {
    /// Coordinate.
    pub coord: G::Coordinate,
    /// Direction.
    pub dir: G::Direction,
}
