# Rust Version 1.82

<hr />

# Install

```
$ git clone https://github.com/YoungHaKim7/rtail.git

$ cd rtail
 
$ cargo install --path .
```

# rust_tail
- fork
  - original code https://crates.io/crates/rtail
  - github https://github.com/17g/rtail
  
- https://github.com/17g/rtail/blob/master/src/main.rs
- https://github.com/rust-lang/getopts/blob/master/src/lib.rs#L136

- 라이브러리 2개 씀(crates.io)
  - https://github.com/unicode-rs/unicode-width
  - https://github.com/notify-rs/notify


# Test Result

```bash
$ cargo r -- -n 10 src/global_fn.rs 

    for n in start_line..end_line {
        result += &line_strs[n as usize][..];
        result += "\n";
    }
    print_result(result);
}

fn print_result(disp_str: String) {
    print!("{}", disp_str);
}

```

# rtail

- This is tail command implemented Rust lang.

```bash
Usage: rtail [options] FILE

Options:
    -n NUMS             number of lines
    -f, ---follow       output appended data as the file grows
    -h                  print help
```
