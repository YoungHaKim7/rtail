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
    - Test code.
      - https://github.com/rust-lang/getopts/blob/master/src/tests/mod.rs 

  
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


# cargo test

```
cargo nextest run
   Compiling rtail v0.1.0 (/home/y/my_project/rust_lang/111111ru/9999999/rtail)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.37s
    Starting 17 tests across 1 binary
        FAIL [   4.038s] rtail::bin/rtail tests::test_combined

--- STDOUT:              rtail::bin/rtail tests::test_combined ---

running 1 test
test tests::test_combined ... FAILED

failures:

failures:
    tests::test_combined

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 16 filtered out; finished in 0.00s


--- STDERR:              rtail::bin/rtail tests::test_combined ---
thread 'tests::test_combined' panicked at src/tests/mod.rs:593:14:
explicit panic
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

   Canceling due to test failure: 11 tests still running
        PASS [   3.730s] rtail::bin/rtail tests::test_free_argument_is_hyphen
        PASS [   3.418s] rtail::bin/rtail tests::test_long_name_too_short
        PASS [   3.108s] rtail::bin/rtail tests::test_long_to_short
        PASS [   2.797s] rtail::bin/rtail tests::test_optflag
        PASS [   2.436s] rtail::bin/rtail tests::test_optflag_missing
        PASS [   2.075s] rtail::bin/rtail tests::test_optflag_short_arg
        PASS [   1.766s] rtail::bin/rtail tests::test_optopt
        FAIL [   1.405s] rtail::bin/rtail tests::test_optopt_missing

--- STDOUT:              rtail::bin/rtail tests::test_optopt_missing ---

running 1 test
test tests::test_optopt_missing ... FAILED

failures:

failures:
    tests::test_optopt_missing

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 16 filtered out; finished in 0.00s


--- STDERR:              rtail::bin/rtail tests::test_optopt_missing ---
thread 'tests::test_optopt_missing' panicked at src/tests/mod.rs:145:14:
explicit panic
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

        PASS [   1.045s] rtail::bin/rtail tests::test_reqopt
        PASS [   2.374s] rtail::bin/rtail tests::test_split_within
        PASS [   2.014s] rtail::bin/rtail tests::test_undefined_opt_present
        PASS [   1.703s] rtail::bin/rtail tests::test_usage
        PASS [   1.342s] rtail::bin/rtail tests::test_usage_description_multibyte_handling
        PASS [   1.032s] rtail::bin/rtail tests::test_usage_description_newline_handling
        PASS [   0.685s] rtail::bin/rtail tests::test_usage_description_wrapping
        PASS [   0.635s] rtail::bin/rtail tests::test_usage_multiwidth
------------
     Summary [   6.068s] 17 tests run: 15 passed, 2 failed, 0 skipped
        FAIL [   4.038s] rtail::bin/rtail tests::test_combined
        FAIL [   1.405s] rtail::bin/rtail tests::test_optopt_missing
error: test run failed
```