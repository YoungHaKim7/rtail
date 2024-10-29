use std::{error::Error, fmt, result};

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

impl Error for Fail {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }

    // fn provide<'a>(&'a self, request: &mut std::error::Request<'a>) {}
}

impl fmt::Display for Fail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Fail::*;
        match *self {
            ArgumentMissing(ref nm) => write!(f, "Argument to option '{}' missing", *nm),
            UnrecognizedOption(ref nm) => write!(f, "Unrecognized option: '{}'", *nm),
            OptionMissing(ref nm) => write!(f, "Required option '{}' missing", *nm),
            OptionDuplicated(ref nm) => write!(f, "Option '{}' given more than once", *nm),
            UnexpectedArgument(ref nm) => write!(f, "Option '{}' does not take an argument", *nm),
        }
    }
}
