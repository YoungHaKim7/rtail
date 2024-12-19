use std::env;

use arg_options::Options;
use global_fn::{print_usage, tail_file, tail_stdin};
mod arg_options;
mod global_fn;
mod optgroup;
mod result_error;

#[allow(unused)]
mod tests;

#[allow(unused)]
fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut options = Options::new();
    options.optopt("n", "", "number of lines", "NUMS");
    options.optflag("f", "-follow", "output appended data as the file grows");
    options.optflag("h", "", "print help");

    let cmd_args = match options.parse(&args[1..]) {
        Ok(ok) => ok,
        Err(e) => panic!("Cannot parse command args : {:?}", e),
    };

    if cmd_args.opt_present("h") {
        print_usage(&program, &options);
        return;
    }

    let line_number = if let Some(str_num) = cmd_args.opt_str("n") {
        match str_num.trim().parse() {
            Ok(num) => num,
            Err(_) => panic!("specify line number!"),
        }
    } else {
        10
    };

    let fflag = cmd_args.opt_present("f");

    if let Some(file) = cmd_args.free.first() {
        tail_file(file, line_number, fflag);
    } else {
        tail_stdin(line_number);
    }
}
