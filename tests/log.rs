use halcyon::log::Category;
use rustest::{main, test};

// Tests that all log priority macro overloads work.
// Only tests `log_trace!` since every other priority macro works exactly the same.
// This also avoids extraneous logs in tests.
#[test]
fn macro_overloads() {
    halcyon::log_trace!("without any arguments");
    halcyon::log_trace!("with an argument {}", 1);
    halcyon::log_trace!(Category::Audio, "with a category");
    halcyon::log_trace!(Category::Audio, "with a category and an argument {}", 2);
}

#[main]
fn main() {}
