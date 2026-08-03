use halcyon::log::Category;
use rustest::test;

/// Both macro overloads (with and without a category) must compile and run;
/// the convenience overload defaults to `Category::Application`.
#[test]
fn log_macro_overloads() {
    halcyon::log_trace!(Category::Audio, "trace {}", 1);
    halcyon::log_trace!("trace {}", 2);
    halcyon::log_verbose!(Category::Audio, "verbose {}", 1);
    halcyon::log_verbose!("verbose {}", 2);
    halcyon::log_debug!(Category::Audio, "debug {}", 1);
    halcyon::log_debug!("debug {}", 2);
    halcyon::log_info!(Category::Audio, "info {}", 1);
    halcyon::log_info!("info {}", 2);
    halcyon::log_warn!(Category::Audio, "warn {}", 1);
    halcyon::log_warn!("warn {}", 2);
    halcyon::log_error!(Category::Audio, "error {}", 1);
    halcyon::log_error!("error {}", 2);
    halcyon::log_critical!(Category::Audio, "critical {}", 1);
    halcyon::log_critical!("critical {}", 2);

    // A format string without arguments, without a category.
    halcyon::log_info!("bare format string");
}
