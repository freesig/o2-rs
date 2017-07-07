
const HEAD_SIZE: usize = 4;
const PORT_SIZE: usize = 2;
const HEADER: & [u8] = b"soc#";

mod finder;
mod beacon;

pub use finder::{Finder, finder, ParseError, Status};
pub use beacon::{Beacon, beacon};

