#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Error {
    InvalidHand,
    InvalidCardNotation,
    InvalidHandNotation,
}
