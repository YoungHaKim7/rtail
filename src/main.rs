use std::env;

use arg_options::Options;
use global_fn::print_usage;

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
}
