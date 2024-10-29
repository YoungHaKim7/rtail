use std::{env, f32::consts::LOG10_2};

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
    fn default() -> Options {
        Self::new()
    }
}

impl Options {
    /// Creates a new [`Options`].
    pub fn new() -> Options {
        Options {
            grps: Vec::new(),
            parsing_style: ParsingStyle::FloatingFrees,
            long_only: false,
        }
    }

    fn optopt(
        &mut self,
        short_name: &str,
        long_name: &str,
        desc: &str,
        hint: &str,
    ) -> &mut Options {
        validate_names(short_name, long_name);
        self.grps.push(OptGroup {
            short_name: short_name.to_string(),
            long_name: long_name.to_string(),
            hint: hint.to_string(),
            desc: desc.to_string(),
            hasarg: HasArg::Yes,
            occur: Occur::Req,
        });
        self
    }

    fn optflag(&mut self, short_name: &str, long_name: &str, desc: &str) -> &mut Options {
        validate_names(short_name, long_name);
        self.grps.push(OptGroup {
            short_name: short_name.to_string(),
            long_name: long_name.to_string(),
            hint: "".to_string(),
            desc: desc.to_string(),
            hasarg: HasArg::No,
            occur: Occur::Optional,
        });
        self
    }
}

fn validate_names(short_name: &str, long_name: &str) {
    let len = short_name.len();
    assert!(
        len == 1 || len == 0,
        "the short_name (first argument) should be a single character, \
        or an empty string for none"
    );
    let len = long_name.len();
    assert!(
        len == 0 || len > 1,
        "the long_name (second argument) should be longer than a single \
         character, or an empty string for none"
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut options = Options::new();
    options.optopt("n", "", "number of lines", "NUMS");
    options.optflag("f", "-follow", "output appended data as the file grows");
}
