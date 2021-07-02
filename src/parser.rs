//! parser

use async_std::io::{prelude::*, BufRead};
use chrono::{DateTime, Local};
use log::debug;
use regex::Regex;
use thiserror::Error;

/// data-time segment in a line
pub(crate) struct CustomDateTime {
    date_time: DateTime<Local>,
}

/// a logical line segment
pub(crate) enum LineSegment {
    DateTime(CustomDateTime),
    Module(String),
    ProcessId(usize),
    ThreadId(usize),
    LogType(LogType),
}

pub(crate) enum LogType {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Custom(String),
}

/// represents a line in log
pub(crate) struct LogLine {}

/// log text reader
pub(crate) struct LogText {
    //reader: dyn BufRead,
}
