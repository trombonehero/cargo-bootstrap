use std;
use std::fmt;
use std::error::Error;

///
/// An error type that unifies failure of the OS to execute a command
/// with a command that executes but returns an error code. Either way,
/// from our perspective, the command did not succeed.
///
pub enum CmdError {
    //Exit { status: std::process::ExitStatus, stdout: String, stderr: String },
    Exit(std::process::Output),
    IoError(std::io::Error),
}

fn write_vec(f: &mut fmt::Formatter, name: &str, v: &[u8]) -> std::fmt::Result {
    write![f, "{}:\n{}\n", name, String::from_utf8_lossy(v)]
}

impl fmt::Display for CmdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &CmdError::Exit(ref result) => {
                match result.status.code() {
                    Some(code) => write![f, "exit code {}\n", code],
                    None => write![f, "no exit code\n"],
                }
                .and_then(|_:()| write_vec(f, "Stdout", &result.stdout))
                .and_then(|_:()| write_vec(f, "Stderr", &result.stderr))
            },

            &CmdError::IoError(ref err) => write![f, "{}", err],
        }
    }
}

impl std::convert::From<std::io::Error> for CmdError {
    fn from(err: std::io::Error) -> CmdError {
        CmdError::IoError(err)
    }
}
