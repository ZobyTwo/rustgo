/// A stone
///
/// Either black, white or empty. 
#[derive(Copy, PartialEq, Clone, Eq, Hash, Debug)]
pub enum Stone {
    Black,
    White,
    Empty
}