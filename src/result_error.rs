use std::{error::Error, fmt, result};

use crate::{HasArg, Name, Occur};

pub type Result = result::Result<Matches, Fail>;

pub struct Matches {
    /// Options that matched
    pub opts: Vec<Opt>,
    /// Values of the Options that matched and their positions
    pub vals: Vec<Vec<(usize, Optval)>>,

    /// Free string fragments
    pub free: Vec<String>,

    /// Index of first free fragment after "--" separator
    pub args_end: Option<usize>,
}

#[derive(Clone, Debug)]
pub struct Opt {
    /// Name of the option
    pub name: Name,
    /// Whether it has an argument
    pub hasarg: HasArg,
    /// How often it can occur
    pub occur: Occur,
    /// Which options it aliases
    pub aliases: Vec<Opt>,
}

// impl Clone for Opt {
//     fn clone(&self) -> Self {
//         Opt {
//             name: self.name.clone(),
//             hasarg: self.hasarg.clone(),
//             occur: self.occur.clone(),
//             aliases: self.aliases.clone(),
//         }
//     }
// }

#[derive(Debug)]
pub enum Fail {
    ArgumentMissing(String),
    UnrecognizedOption(String),
    OptionMissing(String),
    OptionDuplicated(String),
    UnexpectedArgument(String),
}

pub enum Optval {
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
