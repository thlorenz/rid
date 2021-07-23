use log::LevelFilter;
use simple_logger::SimpleLogger;

pub use log::{debug, error, info, trace, warn};
pub fn init() {
    SimpleLogger::new()
        .with_level(LevelFilter::Debug)
        .init()
        .unwrap();
}
