use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::{stdin, BufRead, BufReader, Read, Seek, SeekFrom},
    path::Path,
    process,
    sync::mpsc::channel,
    time::Duration,
};
use unicode_width::UnicodeWidthStr;

use crate::{arg_options::Options, optgroup::Name, result_error::Opt};

const BUF_SIZE: usize = 1024;

pub fn tail_file(path: &String, count: u64, fflag: bool) {
    //let file = match File::open(path){
    let file = match OpenOptions::new().read(true).open(path) {
        Err(why) => panic!(
            "Cannot open file! file:{} cause:{:?}",
            path,
            Error::source(&why)
        ),
        Ok(file) => file,
    };
    let f_metadata = match file.metadata() {
        Err(why) => panic!("Cannot read file metadata :{:?}", Error::source(&why)),
        Ok(data) => data,
    };
    let f_size = f_metadata.len();
    //println!("file size is {} bytes", f_size);
    if f_size == 0 {
        process::exit(0);
    }
    let mut reader = BufReader::new(file);

    let mut line_count = 0;
    // minus 2 byte for skip eof null byte.
    let mut current_pos = f_size - 2;
    let mut read_start = if (f_size - 2) > BUF_SIZE as u64 {
        f_size - 2 - BUF_SIZE as u64
    } else {
        0
    };
    let mut buf = [0; BUF_SIZE];
    'outer: loop {
        match reader.seek(SeekFrom::Start(read_start)) {
            Err(why) => panic!(
                "Cannot move offset! offset:{} cause:{:?}",
                current_pos,
                Error::source(&why)
            ),
            Ok(_) => current_pos,
        };
        let b = match reader.read(&mut buf) {
            Err(why) => panic!(
                "Cannot read offset byte! offset:{} cause:{:?}",
                current_pos,
                Error::source(&why)
            ),
            Ok(b) => b,
        };
        for i in 0..b {
            if buf[b - (i + 1)] == 0xA {
                line_count += 1;
            }
            // println!("{}, {}", line_count, i);
            if line_count == count {
                break 'outer;
            }
            current_pos -= 1;
            //println!("{}", current_pos);
            if current_pos <= 0 {
                current_pos = 0;
                break 'outer;
            }
        }
        read_start = if read_start > BUF_SIZE as u64 {
            read_start - BUF_SIZE as u64
        } else {
            0
        }
    }
    //println!("last pos :{}", current_pos);
    match reader.seek(SeekFrom::Start(current_pos)) {
        Err(why) => panic!(
            "Cannot read offset byte! offset:{} cause:{:?}",
            current_pos,
            Error::source(&why)
        ),
        Ok(_) => current_pos,
    };
    let mut buf_str = String::new();
    match reader.read_to_string(&mut buf_str) {
        Err(why) => panic!(
            "Cannot read offset byte! offset:{} cause:{:?}",
            current_pos,
            Error::source(&why)
        ),
        Ok(_) => current_pos,
    };
    print_result(buf_str);
    if fflag {
        if cfg!(target_os = "windows") {
            println!("");
        }
        if let Err(why) = tail_file_follow(&mut reader, path, f_size) {
            panic!(
                "Cannot follow file! file:{:?} cause:{:?}",
                reader.by_ref(),
                Error::source(&why)
            )
        }
    }
}

fn tail_file_follow(
    reader: &mut BufReader<File>,
    spath: &String,
    file_size: u64,
) -> notify::Result<()> {
    let config = Config::default()
        .with_poll_interval(Duration::from_secs(2))
        .with_compare_contents(true);

    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = Watcher::new(tx, config)?;
    let path = Path::new(spath);
    watcher.watch(path, RecursiveMode::NonRecursive)?;

    let mut start_byte = file_size;
    let mut buf_str = String::new();
    loop {
        match rx.recv() {
            Err(e) => println!("watch error: {:?}", e),
            Ok(_) => {
                match reader.seek(SeekFrom::Start(start_byte)) {
                    Err(why) => panic!(
                        "Cannot move offset! offset:{} cause:{:?}",
                        start_byte,
                        Error::source(&why)
                    ),
                    Ok(_) => start_byte,
                };
                let read_byte = match reader.read_to_string(&mut buf_str) {
                    Err(why) => panic!(
                        "Cannot read offset byte! offset:{} cause:{:?}",
                        start_byte,
                        Error::source(&why)
                    ),
                    Ok(b) => b,
                };
                start_byte += read_byte as u64;
                print_result(buf_str.clone());
                buf_str.clear();
            }
        }
    }
}

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

pub fn tail_stdin(count: u64) {
    let stdin = stdin();
    let mut line_strs: Vec<String> = Vec::new();
    for line in stdin.lock().lines() {
        line_strs.push(match line {
            Err(why) => panic!("Cannot read strin! cause:{:?}", Error::source(&why)),
            Ok(l) => l,
        });
    }
    let mut result = String::new();
    let end_line = line_strs.len() as u64;
    let start_line = if (end_line) > count {
        end_line - count
    } else {
        0
    };
    for n in start_line..end_line {
        result += &line_strs[n as usize][..];
        result += "\n";
    }
    print_result(result);
}

fn print_result(disp_str: String) {
    print!("{}", disp_str);
}
