
/// A position on a board of 19x19 lines
#[derive(Copy, Hash, Eq, PartialEq, Clone, Debug)]
pub struct Position19x19 {
    pub x: usize,
    pub y: usize,
}
