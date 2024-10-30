use std::ffi::OsStr;

use result_error::{Fail, Matches, Opt, Optval};

use crate::{
    global_fn,
    optgroup::{HasArg, Name, Occur, OptGroup, ParsingStyle},
    result_error::{self, Result},
};

use global_fn::{find_opt, is_arg, validate_names};

#[derive(Debug)]
pub struct Options {
    grps: Vec<OptGroup>,
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

    fn clone(&self) -> Self {
        Options {
            grps: self.grps.clone(),
            parsing_style: self.parsing_style.clone(),
            long_only: self.long_only,
        }
    }
}
