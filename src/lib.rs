use std::cell::RefCell;
use std::thread_local;

thread_local! {
    /// Implementation detail; don't use directly
    #[doc(hidden)]
    pub static FAILURES: RefCell<Vec<String>> = RefCell::new(Vec::new());
}

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
