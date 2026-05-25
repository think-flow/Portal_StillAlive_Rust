mod data;
mod player;
pub mod stage;

#[macro_export]
macro_rules! typed {
    (true, $($arg:tt)*) => {{
        println!($($arg)*);
    }};

    (false, $($arg:tt)*) => {{
        typed!($($arg)*);
    }};

    (if $cond:expr, $($arg:tt)*) => {{
        if $cond {
            println!($($arg)*);
        } else {
            typed!($($arg)*);
        }
    }};

    ($($arg:tt)*) => {{
        print!($($arg)*);
        std::io::Write::flush(&mut std::io::stdout()).expect("Failed to flush stdout");
    }};
}
