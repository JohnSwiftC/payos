use std::io;
use std::process::{Child, Command, Stdio};

pub fn open(url: &str) -> io::Result<Child> {
    Command::new("xdg-open")
        .arg(url)
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
}
