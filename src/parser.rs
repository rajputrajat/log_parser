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
    LogType_(LogType),
    Message(String),
}

/// possible Log types
pub(crate) enum LogType {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Custom(String),
}

/// represents a line in log
pub(crate) struct LogLine {
    segments: Vec<LineSegment>,
}

/// log text reader
pub(crate) struct LogParser {
    reader: Box<dyn BufRead>,
    parsed: Vec<LogLine>,
}

impl LogParser {
    fn new(reader: Box<dyn BufRead>) -> Self {
        Self {
            reader,
            parsed: vec![],
        }
    }
}
