/// Prints a success message in green, e.g. after applying/syncing an entry.
macro_rules! success {
    ($($arg:tt)*) => {
        println!("{}", console::style(format!($($arg)*)).green())
    };
}

/// Prints an informational message in cyan.
macro_rules! info {
    ($($arg:tt)*) => {
        println!("{}", console::style(format!($($arg)*)).cyan())
    };
}

/// Prints a warning to stderr in yellow, prefixed with "warning:".
macro_rules! warning {
    ($($arg:tt)*) => {
        eprintln!("{} {}", console::style("warning:").yellow().bold(), format!($($arg)*))
    };
}

/// Prints an error to stderr in red, prefixed with "error:".
macro_rules! error {
    ($($arg:tt)*) => {
        eprintln!("{} {}", console::style("error:").red().bold(), format!($($arg)*))
    };
}

pub(crate) use error;
pub(crate) use info;
pub(crate) use success;
pub(crate) use warning;
