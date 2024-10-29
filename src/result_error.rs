use std::result;

use crate::{HasArg, Name};

pub type Result = result::Result<Matches, Fail>;

pub struct Matches {
    opts: Vec<Opt>,
    val: Vec<Vec<(usize, Optval)>>,
}

struct Opt {
    name: Name,
    hasarg: HasArg,
}

#[derive(Debug)]
pub enum Fail {
    ArgumentMissing(String),
    UnrecognizedOption(String),
    OptionMissing(String),
    OptionDuplicated(String),
    UnexpectedArgument(String),
}

enum Optval {
    Val(String),
    Given,
}
