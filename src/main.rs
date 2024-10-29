use std::env;

#[derive(Debug)]
struct Options {
    grps: Vec<OptGroup>,
    parsing_style: ParsingStyle,
    long_only: bool,
}

#[derive(Debug)]
struct OptGroup {
    short_name: String,
    long_name: String,
    hint: String,
    desc: String,
    hasarg: HasArg,
    occur: Occur,
}

#[derive(Debug)]
enum HasArg {
    Yes,
    No,
    Maybe,
}

#[derive(Debug)]
enum Occur {
    Req,
    Optional,
    Multi,
}

#[derive(Debug)]
enum ParsingStyle {
    FloatingFrees,
    StopAtFirstFree,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            grps: Vec::new(),
            parsing_style: ParsingStyle::FloatingFrees,
            long_only: false,
        }
    }
}

impl Options {
    /// Creates a new [`Options`].
    pub fn new() -> Options {
        Self::new()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut options = Options::new();
}
