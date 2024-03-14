pub use self::grid::SkewedGridLocation;
pub use self::match_capstones::find_groupings;

pub mod grid;
pub mod match_capstones;

/// A simple point in (some) space
#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
