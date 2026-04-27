use std::{ffi::OsStr, process::Command};

use crate::Alert;

pub fn run_command<I, S>(command: &str, args: I) -> Result<String, Alert>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let output = Command::new(command).args(args).output()?;
    let stdout = String::from_utf8(output.stdout)?;
    let stderr = String::from_utf8(output.stderr)?;
    let status = output.status.code().ok_or(&stderr)?;
    if status == 0 {
        println!("StdOut from {}:\n{}", command, stdout);
        return Ok(stdout);
    }
    Err(Alert::from(stderr))
}
