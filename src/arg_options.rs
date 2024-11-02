// Copyright 2012-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
// ignore-lexer-test FIXME #15677

//! Simple getopt alternative.
//!
//! Construct instance of `Options` and configure it by using  `reqopt()`,
//! `optopt()` and other methods that add option configuration. Then call
//! `parse()` method and pass into it a vector of actual arguments (not
//! including `argv[0]`).
//!
//! You'll either get a failure code back, or a match. You'll have to verify
//! whether the amount of 'free' arguments in the match is what you expect. Use
//! `opt_*` accessors to get argument values out of the matches object.
//!
//! Single-character options are expected to appear on the command line with a
//! single preceding dash; multiple-character options are expected to be
//! proceeded by two dashes. Options that expect an argument accept their
//! argument following either a space or an equals sign. Single-character
//! options don't require the space. Everything after double-dash "--"  argument
//! is considered to be a 'free' argument, even if it starts with dash.
//!
//! original code <https://github.com/rust-lang/getopts/blob/master/src/lib.rs>
//! # Usage
//!
//! This crate is [on crates.io](https://crates.io/crates/getopts) and can be
//! used by adding `getopts` to the dependencies in your project's `Cargo.toml`.
//!
//! ```toml
//! [dependencies]
//! getopts = "0.2"
//! ``
//!
//! and this to your crate root:
//!
//! ```rust
//! extern crate getopts;
//! ```
//!
//! # Example
//!
//! The following example shows simple command line parsing for an application
//! that requires an input file to be specified, accepts an optional output file
//! name following `-o`, and accepts both `-h` and `--help` as optional flags.
//!
//! ```{.rust}
//! extern crate getopts;
//! use getopts::Options;
//! use std::env;
//!
//! fn do_work(inp: &str, out: Option<String>) {
//!     println!("{}", inp);
//!     match out {
//!         Some(x) => println!("{}", x),
//!         None => println!("No Output"),
//!     }
//! }
//!
//! fn print_usage(program: &str, opts: Options) {
//!     let brief = format!("Usage: {} FILE [options]", program);
//!     print!("{}", opts.usage(&brief));
//! }
//!
//! fn main() {
//!     let args: Vec<String> = env::args().collect();
//!     let program = args[0].clone();
//!
//!     let mut opts = Options::new();
//!     opts.optopt("o", "", "set output file name", "NAME");
//!     opts.optflag("h", "help", "print this help menu");
//!     let matches = match opts.parse(&args[1..]) {
//!         Ok(m) => { m }
//!         Err(f) => { panic!("{}", f.to_string()) }
//!     };
//!     if matches.opt_present("h") {
//!         print_usage(&program, opts);
//!         return;
//!     }
//!     let output = matches.opt_str("o");
//!     let input = if !matches.free.is_empty() {
//!         matches.free[0].clone()
//!     } else {
//!         print_usage(&program, opts);
//!         return;
//!     };
//!     do_work(&input, output);
//! }
//! ```

use result_error::{Fail, Matches, Opt, Optval};
use std::{ffi::OsStr, iter::repeat};
// use unicode_width::UnicodeWidthStr;
use unicode_width::UnicodeWidthStr;

use crate::{
    global_fn::{self, each_split_within},
    optgroup::{HasArg, Name, Occur, OptGroup, ParsingStyle},
    result_error::{self, Result},
};

use global_fn::{find_opt, is_arg, validate_names};

#[derive(Debug, PartialEq, Eq)]
pub struct Options {
    pub grps: Vec<OptGroup>,
    parsing_style: ParsingStyle,
    long_only: bool,
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

    pub fn optopt(
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

    pub fn optflag(&mut self, short_name: &str, long_name: &str, desc: &str) -> &mut Options {
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

    // pub fn opt_strs(&self, name: &str) -> Vec<String> {
    //     self.opt_vals(name)
    //         .into_iter()
    //         .filter_map(|(_, v)| match v {
    //             Optval::Val(s) => Some(s),
    //             _ => None,
    //         })
    //         .collect()
    // }

    // fn opt_vals(&self, nm: &str) -> Vec<(usize, Optval)> {
    //     match find_opt(&self.opts, &Name::from_str(nm)) {
    //         Some(id) => self.vals[id].clone(),
    //         None => panic!("No option '{}' defined", nm),
    //     }
    // }

    pub fn parse<C>(&self, args: C) -> Result
    where
        C: IntoIterator,
        C::Item: AsRef<OsStr>,
    {
        let opts: Vec<Opt> = self.grps.iter().map(|x| x.long_to_short()).collect();

        let mut vals = (0..opts.len())
            .map(|_| Vec::new())
            .collect::<Vec<Vec<(usize, Optval)>>>();
        let mut free: Vec<String> = Vec::new();
        let mut args_end = None;

        let args = args
            .into_iter()
            .map(|i| {
                i.as_ref()
                    .to_str()
                    .ok_or_else(|| Fail::UnrecognizedOption(format!("{:?}", i.as_ref())))
                    .map(|s| s.to_owned())
            })
            .collect::<::std::result::Result<Vec<_>, _>>()?;
        let mut args = args.into_iter().peekable();
        let mut arg_pos = 0;
        while let Some(cur) = args.next() {
            if !is_arg(&cur) {
                free.push(cur);
                match self.parsing_style {
                    ParsingStyle::FloatingFrees => {}
                    ParsingStyle::StopAtFirstFree => {
                        free.extend(args);
                        break;
                    }
                }
            } else if cur == "--" {
                args_end = Some(free.len());
                free.extend(args);
                break;
            } else {
                let mut name = None;
                let mut i_arg = None;
                let mut was_long = true;
                if cur.as_bytes()[1] == b'-' || self.long_only {
                    let tail = if cur.as_bytes()[1] == b'-' {
                        &cur[2..]
                    } else {
                        assert!(self.long_only);
                        &cur[1..]
                    };
                    let mut parts = tail.splitn(2, '=');
                    name = Some(Name::from_str(parts.next().unwrap()));
                    if let Some(rest) = parts.next() {
                        i_arg = Some(rest.to_string());
                    }
                } else {
                    was_long = false;
                    for (j, ch) in cur.char_indices().skip(1) {
                        let opt = Name::Short(ch);

                        let opt_id = match find_opt(&opts, &opt) {
                            Some(id) => id,
                            None => return Err(Fail::UnrecognizedOption(opt.to_string())),
                        };

                        // In a series of potential options (eg. -aheJ), if we
                        // see one which takes an argument, we assume all
                        // subsequent characters make up the argument. This
                        // allows options such as -L/usr/local/lib/foo to be
                        // interpreted correctly
                        let arg_follows = match opts[opt_id].hasarg {
                            HasArg::Yes | HasArg::Maybe => true,
                            HasArg::No => false,
                        };

                        if arg_follows {
                            name = Some(opt);
                            let next = j + ch.len_utf8();
                            if next < cur.len() {
                                i_arg = Some(cur[next..].to_string());
                                break;
                            }
                        } else {
                            vals[opt_id].push((arg_pos, Optval::Given));
                        }
                    }
                }
                if let Some(nm) = name {
                    let opt_id = match find_opt(&opts, &nm) {
                        Some(id) => id,
                        None => return Err(Fail::UnrecognizedOption(nm.to_string())),
                    };
                    match opts[opt_id].hasarg {
                        HasArg::No => {
                            if i_arg.is_some() {
                                return Err(Fail::UnexpectedArgument(nm.to_string()));
                            }
                            vals[opt_id].push((arg_pos, Optval::Given));
                        }
                        HasArg::Maybe => {
                            // Note that here we do not handle `--arg value`.
                            // This matches GNU getopt behavior; but also
                            // makes sense, because if this were accepted,
                            // then users could only write a "Maybe" long
                            // option at the end of the arguments when
                            // FloatingFrees is in use.
                            if let Some(i_arg) = i_arg.take() {
                                vals[opt_id].push((arg_pos, Optval::Val(i_arg)));
                            } else if was_long || args.peek().map_or(true, |n| is_arg(&n)) {
                                vals[opt_id].push((arg_pos, Optval::Given));
                            } else {
                                vals[opt_id].push((arg_pos, Optval::Val(args.next().unwrap())));
                            }
                        }
                        HasArg::Yes => {
                            if let Some(i_arg) = i_arg.take() {
                                vals[opt_id].push((arg_pos, Optval::Val(i_arg)));
                            } else if let Some(n) = args.next() {
                                vals[opt_id].push((arg_pos, Optval::Val(n)));
                            } else {
                                return Err(Fail::ArgumentMissing(nm.to_string()));
                            }
                        }
                    }
                }
            }
            arg_pos += 1;
        }
        debug_assert_eq!(vals.len(), opts.len());
        for (vals, opt) in vals.iter().zip(opts.iter()) {
            if opt.occur == Occur::Req && vals.is_empty() {
                return Err(Fail::OptionMissing(opt.name.to_string()));
            }
            if opt.occur != Occur::Multi && vals.len() > 1 {
                return Err(Fail::OptionDuplicated(opt.name.to_string()));
            }
        }

        // Note that if "--" is last argument on command line, then index stored
        // in option does not exist in `free` and must be replaced with `None`
        args_end = args_end.filter(|pos| pos != &free.len());

        Ok(Matches {
            opts,
            vals,
            free,
            args_end,
        })
    }

    #[allow(unused)]
    fn clone(&self) -> Self {
        Options {
            grps: self.grps.clone(),
            parsing_style: self.parsing_style.clone(),
            long_only: self.long_only,
        }
    }
    /// Create a long option that is required and takes an argument.
    ///
    /// * `short_name` - e.g. `"h"` for a `-h` option, or `""` for none
    /// * `long_name` - e.g. `"help"` for a `--help` option, or `""` for none
    /// * `desc` - Description for usage help
    /// * `hint` - Hint that is used in place of the argument in the usage help,
    ///   e.g. `"FILE"` for a `-o FILE` option
    ///
    /// # Example
    ///
    /// ```
    /// # use getopts::Options;
    /// # use getopts::Fail;
    /// let mut opts = Options::new();
    /// opts.optopt("o", "optional", "optional text option", "TEXT");
    /// opts.reqopt("m", "mandatory", "madatory text option", "TEXT");
    ///
    /// let result = opts.parse(&["--mandatory", "foo"]);
    /// assert!(result.is_ok());
    ///
    /// let result = opts.parse(&["--optional", "foo"]);
    /// assert!(result.is_err());
    /// assert_eq!(Fail::OptionMissing("mandatory".to_owned()), result.unwrap_err());
    /// ```
    #[allow(unused)]
    pub fn reqopt(
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
    /// Derive a formatted message from a set of options.
    pub fn usage(&self, brief: &str) -> String {
        self.usage_with_format(|opts| {
            format!(
                "{}\n\nOptions:\n{}\n",
                brief,
                opts.collect::<Vec<String>>().join("\n")
            )
        })
    }
    /// Derive a custom formatted message from a set of options. The formatted options provided to
    /// a closure as an iterator.
    pub fn usage_with_format<F>(&self, mut formatter: F) -> String
    where
        F: FnMut(&mut dyn Iterator<Item = String>) -> String,
    {
        formatter(&mut self.usage_items())
    }

    /// Derive usage items from a set of options.
    fn usage_items<'a>(&'a self) -> Box<dyn Iterator<Item = String> + 'a> {
        let desc_sep = format!("\n{}", repeat(" ").take(24).collect::<String>());

        let any_short = self.grps.iter().any(|optref| !optref.short_name.is_empty());

        let rows = self.grps.iter().map(move |optref| {
            let OptGroup {
                short_name,
                long_name,
                hint,
                desc,
                hasarg,
                ..
            } = (*optref).clone();

            let mut row = "    ".to_string();

            // short option
            match short_name.width() {
                0 => {
                    if any_short {
                        row.push_str("    ");
                    }
                }
                1 => {
                    row.push('-');
                    row.push_str(&short_name);
                    if long_name.width() > 0 {
                        row.push_str(", ");
                    } else {
                        // Only a single space here, so that any
                        // argument is printed in the correct spot.
                        row.push(' ');
                    }
                }
                // FIXME: refer issue #7.
                _ => panic!("the short name should only be 1 ascii char long"),
            }

            // long option
            match long_name.width() {
                0 => {}
                _ => {
                    row.push_str(if self.long_only { "-" } else { "--" });
                    row.push_str(&long_name);
                    row.push(' ');
                }
            }

            // arg
            match hasarg {
                HasArg::No => {}
                HasArg::Yes => row.push_str(&hint),
                HasArg::Maybe => {
                    row.push('[');
                    row.push_str(&hint);
                    row.push(']');
                }
            }

            let rowlen = row.width();
            if rowlen < 24 {
                for _ in 0..24 - rowlen {
                    row.push(' ');
                }
            } else {
                row.push_str(&desc_sep)
            }

            let desc_rows = each_split_within(&desc, 54);
            row.push_str(&desc_rows.join(&desc_sep));

            row
        });

        Box::new(rows)
    }
}
