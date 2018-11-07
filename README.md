# Soft-Test-Failures

Multiple test failures, any one of which will result in a failed test. Like gtest's `EXPECT_*` rather than `ASSERT_*`.

```rust
#[test]
fn expect_failures() {
    let x = 4;
    let y = "is not";
    let z = 5;
    expect!(2 + 2 == 5, "{} surely {} {}", x, y, z);
    expect!(1 + 1 == 2);
    expect!(3 - 7 == -4);
    expect!(3 - 7 == -3);
    let_fail!();
}

```

```txt
running 1 test
test expect_failures ... FAILED

failures:

---- expect_failures stdout ----
thread 'expect_failures' panicked at '`expect` test failed with 2 failed assertions:
1: 4 surely is not 5
2: Expected 3 - 7 == -3
', src/lib.rs:48:5
note: Run with `RUST_BACKTRACE=1` for a backtrace.

```
