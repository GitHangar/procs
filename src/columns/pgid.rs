use crate::process::ProcessInfo;
use crate::{column_default, Column};
use std::cmp;
use std::collections::HashMap;

pub struct Pgid {
    header: String,
    unit: String,
    fmt_contents: HashMap<i32, String>,
    raw_contents: HashMap<i32, i32>,
    width: usize,
}

impl Pgid {
    pub fn new(header: Option<String>) -> Self {
        let header = header.unwrap_or_else(|| String::from("PGID"));
        let unit = String::from("");
        Pgid {
            fmt_contents: HashMap::new(),
            raw_contents: HashMap::new(),
            width: 0,
            header,
            unit,
        }
    }
}

#[cfg(target_os = "linux")]
impl Column for Pgid {
    fn add(&mut self, proc: &ProcessInfo) {
        let raw_content = proc.pgid;
        let fmt_content = match proc.curr_proc {
            crate::process::ProcessTask::Process(_) => format!("{}", raw_content),
            _ => format!("[{}]", raw_content),
        };

        self.fmt_contents.insert(proc.pid, fmt_content);
        self.raw_contents.insert(proc.pid, raw_content);
    }

    column_default!(i32);
}

#[cfg(not(target_os = "linux"))]
impl Column for Pgid {
    fn add(&mut self, proc: &ProcessInfo) {
        let raw_content = proc.pgid;
        let fmt_content = format!("{}", raw_content);

        self.fmt_contents.insert(proc.pid, fmt_content);
        self.raw_contents.insert(proc.pid, raw_content);
    }

    column_default!(i32);
}
