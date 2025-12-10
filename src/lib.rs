//! Split by difficulty

#[allow(unused_macros)]
macro_rules! dbg {
    ($expr:expr) => {{
        let value = $expr;
        eprintln!("{} = {:?}", stringify!($expr), value);
        value
    }};
}

pub mod diff_5_kyu;
pub mod diff_6_kyu;
pub mod diff_7_kyu;
pub mod diff_8_kyu;
pub mod utils;
