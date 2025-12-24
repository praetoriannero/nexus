mod bit_serde;
mod bit_width;

/// Most significant bit orientation
pub enum Msb {
    Left,
    Right,
}

pub mod prelude {
    pub use crate::bit_serde::*;
    pub use crate::bit_width::*;
}
