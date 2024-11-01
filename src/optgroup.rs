use crate::result_error::Opt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptGroup {
    pub short_name: String,
    pub long_name: String,
    pub hint: String,
    pub desc: String,
    pub hasarg: HasArg,
    pub occur: Occur,
}

#[allow(unused)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HasArg {
    Yes,
    No,
    Maybe,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Occur {
    Req,
    Optional,
    Multi,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Name {
    Long(String),
    Short(char),
}

#[allow(unused)]
#[derive(Debug, PartialEq, Eq)]
pub enum ParsingStyle {
    FloatingFrees,
    StopAtFirstFree,
}
impl ParsingStyle {
    pub fn clone(&self) -> ParsingStyle {
        todo!()
    }
}

#[allow(unused)]
impl Name {
    pub fn from_str(nm: &str) -> Name {
        if nm.len() == 1 {
            Name::Short(nm.as_bytes()[0] as char)
        } else {
            Name::Long(nm.to_string())
        }
    }

    pub fn to_string(&self) -> String {
        match *self {
            Name::Short(ch) => ch.to_string(),
            Name::Long(ref s) => s.to_string(),
        }
    }

    fn clone(&self) -> Name {
        todo!()
    }
}

impl OptGroup {
    pub fn long_to_short(&self) -> Opt {
        let OptGroup {
            short_name,
            long_name,
            hasarg,
            occur,
            ..
        } = (*self).clone();

        match (short_name.len(), long_name.len()) {
            (0, 0) => panic!("this long-format option was given no name"),
            (0, _) => Opt {
                name: Name::Long(long_name),
                hasarg,
                occur,
                aliases: Vec::new(),
            },
            (1, 0) => Opt {
                name: Name::Short(short_name.as_bytes()[0] as char),
                hasarg,
                occur,
                aliases: Vec::new(),
            },
            (1, _) => Opt {
                name: Name::Long(long_name),
                hasarg: hasarg.clone(),
                occur: occur.clone(),
                aliases: vec![Opt {
                    name: Name::Short(short_name.as_bytes()[0] as char),
                    hasarg: hasarg.clone(),
                    occur: occur.clone(),
                    aliases: Vec::new(),
                }],
            },
            (_, _) => panic!("something is wrong with the long-form opt"),
        }
    }

    fn clone(&self) -> Self {
        OptGroup {
            short_name: self.short_name.clone(),
            long_name: self.long_name.clone(),
            hint: self.hint.clone(),
            desc: self.desc.clone(),
            hasarg: self.hasarg.clone(),
            occur: self.occur.clone(),
        }
    }
}

impl Clone for Name {
    fn clone(&self) -> Self {
        match *self {
            Name::Short(ch) => Name::Short(ch),
            Name::Long(ref s) => Name::Long(s.clone()),
        }
    }
}
