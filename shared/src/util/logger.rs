use crate::util::types::BasicResult;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Logger, Root};
use log4rs::Config;
use std::sync::Mutex;

static HANDLE: Mutex<Option<log4rs::Handle>> = Mutex::new(None);

pub fn init_logger(level: log::LevelFilter) -> BasicResult<()> {
    let mut handle = HANDLE.lock().unwrap();

    let stdout = ConsoleAppender::builder().build();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .logger(Logger::builder().build("requests", level))
        .build(Root::builder().appender("stdout").build(level))
        .map_err(|e| e.to_string())?;

    if let Some(handle) = handle.as_ref() {
        handle.set_config(config);
    } else {
        handle.replace(log4rs::init_config(config).map_err(|e| e.to_string())?);
    }

    Ok(())
}
