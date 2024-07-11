use crate::rule::Rule;

// Evaluation engine for A-5 lowball (played in Razz, low in PLO8, low in Stud8)

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Lowball {}

impl Rule for Lowball {}
