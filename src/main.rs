use std::env;

use arg_options::Options;
use global_fn::{print_usage, tail_file, tail_stdin};

mod arg_options;
mod global_fn;
mod optgroup;
mod result_error;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut options = Options::new();
    options.optopt("n", "", "number of lines", "NUMS");
    options.optflag("f", "-follow", "output appended data as the file grows");
    options.optflag("h", "", "print help");
    let cmd_args = match options.parse(&args[1..]) {
        Err(e) => panic!("Cannot parse command args : {:?}", e),
        Ok(ok) => ok,
    };
    if cmd_args.opt_present("h") {
        print_usage(&program, &options);
    }
    let line_number = if cmd_args.opt_present("n") {
        let str_num = match cmd_args.opt_str("n") {
            None => panic!("specify line number!"),
            Some(num) => num,
        };
        match str_num.trim().parse() {
            Err(_) => panic!("specify line number!"),
            Ok(num) => num,
        }
    } else {
        10
    };
    let fflag = cmd_args.opt_present("f");
    if cmd_args.free.is_empty() {
        tail_stdin(line_number);
    } else {
        let file = cmd_args.free[0].clone();
        tail_file(&file, line_number, fflag);
    }
}
