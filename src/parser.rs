//! parser

use async_std::{
    fs::File,
    future::Future,
    io::{self, prelude::*, BufReader},
    path::PathBuf,
};
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
    Ignore(String),
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
pub struct LogParser {
    log_paths: Vec<PathBuf>,
    // reader: BufReader<File>,
    parsed: Vec<LogLine>,
}

/// associated functions
impl LogParser {
    pub fn new(log_paths: Vec<PathBuf>) -> Self {
        Self {
            log_paths,
            // reader,
            parsed: vec![],
        }
    }
}

/// public methods
// impl LogParser {
//     fn
// }

/// private methods
impl LogParser {
    async fn check_if_all_files_exist(&self) -> bool {
        self.log_paths.iter().all(|f| async { f.exists().await })
    }
}

#[derive(Error, Debug)]
pub enum LogParserError {
    #[error("file access issue")]
    FileAccess(#[from] io::Error),
    #[error("unknown error")]
    Unknown,
}
