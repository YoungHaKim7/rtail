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


# `cargo test && cargo nextest run && rustc --version --verbose` 

```
$ rustc --version --verbose
rustc 1.82.0 (f6e511eec 2024-10-15)
binary: rustc
commit-hash: f6e511eec7342f59a25f7c0534f1dbea00d01b14
commit-date: 2024-10-15
host: aarch64-apple-darwin
release: 1.82.0
LLVM version: 19.1.1

$ cargo t

running 50 tests
test tests::notify::poll_watcher::poll_watcher_is_send_and_sync ... ok
test tests::test_free_argument_is_hyphen ... ok
test tests::test_long_to_short ... ok
test tests::test_optflag_missing ... ok
test tests::test_optflag ... ok
test tests::test_long_name_too_short - should panic ... ok
test tests::test_optflag_short_arg ... ok
test tests::test_optopt ... ok
test tests::test_reqopt ... ok
test tests::test_split_within ... ok
test tests::test_usage_description_multibyte_handling ... ok
test tests::test_usage_description_newline_handling ... ok
test tests::test_usage ... ok
test tests::unicode_tests::ambiguous_line_break ... ok
test tests::test_undefined_opt_present - should panic ... ok
test tests::test_usage_description_wrapping ... ok
test tests::test_usage_multiwidth ... ok
test tests::unicode_tests::test_arabic_lam_alef ... ok
test tests::unicode_tests::test_bad_devanagari ... ok
test tests::unicode_tests::test_ambiguous ... ok
test tests::unicode_tests::test_buginese_a_i_ya ... ok
test tests::unicode_tests::test_char ... ok
test tests::unicode_tests::test_char2 ... ok
test tests::unicode_tests::test_control_line_break ... ok
test tests::unicode_tests::test_default_ignorable ... ok
test tests::unicode_tests::test_devanagari_caret ... ok
test tests::unicode_tests::test_emoji ... ok
test tests::unicode_tests::test_emoji_modifier ... ok
test tests::unicode_tests::test_emoji_presentation ... ok
test tests::unicode_tests::test_gcb_prepend ... ok
test tests::unicode_tests::test_hebrew_alef_lamed ... ok
test tests::unicode_tests::test_emoji_zwj ... ok
test tests::unicode_tests::test_hieroglyph_format_controls ... ok
test tests::unicode_tests::test_interlinear_annotation_chars ... ok
test tests::unicode_tests::test_jamo ... ok
test tests::unicode_tests::test_khmer_qaa ... ok
test tests::unicode_tests::test_khmer_sign_beyyal ... ok
test tests::unicode_tests::test_lisu_tones ... ok
test tests::unicode_tests::test_marks ... ok
test tests::unicode_tests::test_old_turkic_ligature ... ok
test tests::unicode_tests::test_prepended_concatenation_marks ... ok
test tests::unicode_tests::test_str ... ok
test tests::unicode_tests::test_solidus_overlay ... ok
test tests::unicode_tests::test_text_presentation ... ok
test tests::unicode_tests::test_tifinagh_biconsonants ... ok
test tests::unicode_tests::unicode_12 ... ok
test tests::unicode_tests::emoji_test_file ... ok
test tests::unicode_tests::char_str_consistent ... ok
test tests::unicode_tests::test_khmer_coeng ... ok
test tests::notify::race_with_remove_dir::test_race_with_remove_dir ... ok

test result: ok. 50 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.01s

# Rust Version 🦀 v1.82.0

$ cargo nextest run
        PASS [   0.007s] rtail::bin/rtail tests::test_long_to_short
        PASS [   0.008s] rtail::bin/rtail tests::notify::poll_watcher::poll_watcher_is_send_and_sync
        PASS [   0.008s] rtail::bin/rtail tests::test_free_argument_is_hyphen
        PASS [   0.009s] rtail::bin/rtail tests::test_long_name_too_short
        PASS [   0.008s] rtail::bin/rtail tests::test_optflag_missing
        PASS [   0.008s] rtail::bin/rtail tests::test_optflag_short_arg
        PASS [   0.010s] rtail::bin/rtail tests::test_optflag
        PASS [   0.007s] rtail::bin/rtail tests::test_optopt
        PASS [   0.008s] rtail::bin/rtail tests::test_reqopt
        PASS [   0.007s] rtail::bin/rtail tests::test_usage
        PASS [   0.007s] rtail::bin/rtail tests::test_usage_description_multibyte_handling
        PASS [   0.009s] rtail::bin/rtail tests::test_split_within
        PASS [   0.009s] rtail::bin/rtail tests::test_undefined_opt_present
        PASS [   0.008s] rtail::bin/rtail tests::test_usage_description_newline_handling
        PASS [   0.007s] rtail::bin/rtail tests::test_usage_description_wrapping
        PASS [   0.008s] rtail::bin/rtail tests::test_usage_multiwidth
        PASS [   0.007s] rtail::bin/rtail tests::unicode_tests::ambiguous_line_break
        PASS [   0.007s] rtail::bin/rtail tests::unicode_tests::test_ambiguous
        PASS [   0.007s] rtail::bin/rtail tests::unicode_tests::test_arabic_lam_alef
        PASS [   0.006s] rtail::bin/rtail tests::unicode_tests::test_buginese_a_i_ya
        PASS [   0.009s] rtail::bin/rtail tests::unicode_tests::test_bad_devanagari
        PASS [   0.008s] rtail::bin/rtail tests::unicode_tests::test_char
        PASS [   0.008s] rtail::bin/rtail tests::unicode_tests::test_char2
        PASS [   0.009s] rtail::bin/rtail tests::unicode_tests::test_control_line_break
        PASS [   0.007s] rtail::bin/rtail tests::unicode_tests::test_default_ignorable
        PASS [   0.007s] rtail::bin/rtail tests::unicode_tests::test_devanagari_caret
        PASS [   0.008s] rtail::bin/rtail tests::unicode_tests::test_emoji
        PASS [   0.008s] rtail::bin/rtail tests::unicode_tests::test_emoji_modifier
        PASS [   0.007s] rtail::bin/rtail tests::unicode_tests::test_emoji_presentation
        PASS [   0.007s] rtail::bin/rtail tests::unicode_tests::test_emoji_zwj
        PASS [   0.007s] rtail::bin/rtail tests::unicode_tests::test_gcb_prepend
        PASS [   0.007s] rtail::bin/rtail tests::unicode_tests::test_hebrew_alef_lamed
        PASS [   0.007s] rtail::bin/rtail tests::unicode_tests::test_hieroglyph_format_controls
        PASS [   0.008s] rtail::bin/rtail tests::unicode_tests::test_jamo
        PASS [   0.009s] rtail::bin/rtail tests::unicode_tests::test_interlinear_annotation_chars
        PASS [   0.007s] rtail::bin/rtail tests::unicode_tests::test_khmer_qaa
        PASS [   0.007s] rtail::bin/rtail tests::unicode_tests::test_khmer_sign_beyyal
        PASS [   0.007s] rtail::bin/rtail tests::unicode_tests::test_marks
        PASS [   0.008s] rtail::bin/rtail tests::unicode_tests::test_lisu_tones
        PASS [   0.007s] rtail::bin/rtail tests::unicode_tests::test_old_turkic_ligature
        PASS [   0.007s] rtail::bin/rtail tests::unicode_tests::test_prepended_concatenation_marks
        PASS [   0.007s] rtail::bin/rtail tests::unicode_tests::test_str
        PASS [   0.008s] rtail::bin/rtail tests::unicode_tests::test_solidus_overlay
        PASS [   0.049s] rtail::bin/rtail tests::unicode_tests::emoji_test_file
        PASS [   0.007s] rtail::bin/rtail tests::unicode_tests::test_text_presentation
        PASS [   0.007s] rtail::bin/rtail tests::unicode_tests::test_tifinagh_biconsonants
        PASS [   0.006s] rtail::bin/rtail tests::unicode_tests::unicode_12
        PASS [   0.302s] rtail::bin/rtail tests::unicode_tests::char_str_consistent
        PASS [   0.550s] rtail::bin/rtail tests::unicode_tests::test_khmer_coeng
        PASS [   1.014s] rtail::bin/rtail tests::notify::race_with_remove_dir::test_race_with_remove_dir
------------
     Summary [   1.015s] 50 tests run: 50 passed, 0 skipped

```


# `cargo check --all-features --all-targets --all`

```bash
$ cargo check --all-features --all-targets --all
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
```