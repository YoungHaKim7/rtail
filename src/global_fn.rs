use std::process;
use unicode_width::UnicodeWidthStr;

use crate::{arg_options::Options, optgroup::Name, result_error::Opt};

pub fn validate_names(short_name: &str, long_name: &str) {
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

pub fn is_arg(arg: &str) -> bool {
    arg.as_bytes().get(0) == Some(&b'-') && arg.len() > 1
}

pub fn find_opt(opts: &[Opt], nm: &Name) -> Option<usize> {
    // Search main options.
    let pos = opts.iter().position(|opt| &opt.name == nm);
    if pos.is_some() {
        return pos;
    }

    // Search in aliases.
    for candidate in opts.iter() {
        if candidate.aliases.iter().any(|opt| &opt.name == nm) {
            return opts.iter().position(|opt| opt.name == candidate.name);
        }
    }

    None
}

pub fn print_usage(program: &str, options: &Options) {
    let brief = format!("Usage: {} [options] FILE", program);
    print!("{}", options.usage(&brief));
    process::exit(0);
}

pub fn each_split_within(desc: &str, lim: usize) -> Vec<String> {
    let mut rows = Vec::new();
    for line in desc.trim().lines() {
        let line_chars = line.chars().chain(Some(' '));
        let words = line_chars
            .fold((Vec::new(), 0, 0), |(mut words, a, z), c| {
                let idx = z + c.len_utf8(); // Get the current byte offset

                // If the char is whitespace, advance the word start and maybe push a word
                if c.is_whitespace() {
                    if a != z {
                        words.push(&line[a..z]);
                    }
                    (words, idx, idx)
                }
                // If the char is not whitespace, continue, retaining the current
                else {
                    (words, a, idx)
                }
            })
            .0;

        let mut row = String::new();
        for word in words.iter() {
            let sep = if !row.is_empty() { Some(" ") } else { None };
            let width = row.width() + word.width() + sep.map(UnicodeWidthStr::width).unwrap_or(0);

            if width <= lim {
                if let Some(sep) = sep {
                    row.push_str(sep)
                }
                row.push_str(word);
                continue;
            }
            if !row.is_empty() {
                rows.push(row.clone());
                row.clear();
            }
            row.push_str(word);
        }
        if !row.is_empty() {
            rows.push(row);
        }
    }
    rows
}
