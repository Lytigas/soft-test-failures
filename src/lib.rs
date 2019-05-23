//! This crate trackes assertions during the execution of a test, delaying panicks until the end of execution.
//! This ensures that if two assertions were to fail, the first does not clobber the second. This is most useful
//! when writing large tests with many assertions that may begin to fail simultaneously, as you can pinpoint exactly
//! which ones failed.
//!
//! # Usage
//!
//! Replace your normal `assert!()` assertions with `expect!()` from this crate. At the end of your test, call `let_fail!()`
//!
//! ```rust
//! #[test]
//! fn expect_failures() {
//!     let x = 4;
//!     let y = "is not";
//!     let z = 5;
//!     expect!(2 + 2 == 5, "{} surely {} {}", x, y, z);
//!     expect!(1 + 1 == 2);
//!     expect!(3 - 7 == -4);
//!     expect!(3 - 7 == -3);
//!     let_fail!();
//! }
//! ```

use std::cell::RefCell;
use std::thread_local;

thread_local! {
    /// Implementation detail; don't use directly
    #[doc(hidden)]
    pub static FAILURES: RefCell<Vec<String>> = RefCell::new(Vec::new());
}

/// Tracks a failed condition like an assertion, but allows test execution to continue.
#[macro_export]
macro_rules! expect {
    ($cond:expr) => {{
        expect!($cond, "Expected {}", stringify!($cond));
    }};
    ($cond:expr, $($arg:tt)+) => {{
        if !($cond) {
            $crate::FAILURES.with(|cell| cell.borrow_mut().push(format!($($arg)*)));
        }
    }};
}

/// Panics if at least one `expect!()` failed, thus failing the test.
#[macro_export]
macro_rules! let_fail {
    () => {
        use std::fmt::Write;
        $crate::FAILURES.with(|cell| {
            let f = cell.borrow_mut();
            if f.len() > 0 {
                let mut msg = format!("`expect` test failed with {} failed assertions:\n", f.len());
                f.iter()
                    .enumerate()
                    .for_each(|x| writeln!(msg, "{}: {}", x.0 + 1, x.1).unwrap()); // <String as Write> never panics
                panic!(msg);
            } else {
                println!("`expect` test passed.");
            }
        });
    };
}

#[test]
#[should_panic]
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

#[test]
fn expect_pass() {
    let x = 4;
    let y = "is not";
    let z = 5;
    expect!(2 + 2 == 4, "{} surely {} {}", x, y, z);
    expect!(1 + 1 == 2);
    expect!(3 - 7 == -4);
    expect!(3 - 7 == -4);
    let_fail!();
}
