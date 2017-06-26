// private sub-module defined in other files
mod max_filter;
mod mean_filter;
mod min_filter;
mod total_filter;

// exports identifiers from private sub-modules in the current module namespace
pub use self::max_filter::MaximumFilter;
pub use self::mean_filter::MeanFilter;
pub use self::min_filter::MinimumFilter;
pub use self::total_filter::TotalFilter;