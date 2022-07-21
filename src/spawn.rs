use std::collections::HashMap;
use std::env;
use std::ffi::OsStr;
use std::io::Write;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::process::{Command, Output, Stdio};

use serde::Serialize;

#[derive(Serialize)]
pub struct Info {
    program: String,
    args: Vec<String>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    env: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stdin: Option<String>,
}

fn describe_program(cmd: &OsStr) -> String {
    let filename = Path::new(cmd).file_name().unwrap();
    let name = filename.to_string_lossy();
    let mut name = &name as &str;
    if !env::consts::EXE_SUFFIX.is_empty() {
        name = name.strip_suffix(env::consts::EXE_SUFFIX).unwrap_or(name);
    }
    name.into()
}

impl Info {
    fn from_std_command(cmd: &Command) -> Info {
        Info {
            program: describe_program(cmd.get_program()),
            args: cmd
                .get_args()
                .map(|x| x.to_string_lossy().into_owned())
                .collect(),
            env: cmd
                .get_envs()
                .map(|(k, v)| {
                    (
                        k.to_string_lossy().into_owned(),
                        v.unwrap_or(OsStr::new("")).to_string_lossy().into_owned(),
                    )
                })
                .collect(),
            stdin: None,
        }
    }
}

/// Implemented by different types that can be spawned and snapshotted.
pub trait Spawn {
    #[doc(hidden)]
    fn spawn_with_info(&mut self) -> (Info, Output);
}

impl Spawn for Command {
    fn spawn_with_info(&mut self) -> (Info, Output) {
        let info = Info::from_std_command(self);
        let output = self.output().unwrap();
        (info, output)
    }
}

impl<'a> Spawn for &'a mut Command {
    fn spawn_with_info(&mut self) -> (Info, Output) {
        <Command as Spawn>::spawn_with_info(self)
    }
}

/// Like [`Command`] but sends some input to stdin.
pub struct StdinCommand {
    command: Command,
    stdin: Vec<u8>,
}

impl StdinCommand {
    /// Creates a new command that also gets some input value fed to stdin.
    pub fn new<S: AsRef<OsStr>, I: Into<Vec<u8>>>(program: S, stdin: I) -> StdinCommand {
        let mut command = Command::new(program);
        command.stdin(Stdio::piped());
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());
        StdinCommand {
            command,
            stdin: stdin.into(),
        }
    }
}

impl Deref for StdinCommand {
    type Target = Command;

    fn deref(&self) -> &Self::Target {
        &self.command
    }
}

impl DerefMut for StdinCommand {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.command
    }
}

impl Spawn for StdinCommand {
    fn spawn_with_info(&mut self) -> (Info, Output) {
        let mut info = Info::from_std_command(&self.command);
        let mut child = self.command.spawn().unwrap();
        let mut stdin = child.stdin.take().expect("Failed to open stdin");
        let to_write = mem::take(&mut self.stdin);
        info.stdin = Some(String::from_utf8_lossy(&to_write).into());
        std::thread::spawn(move || {
            stdin
                .write_all(&to_write)
                .expect("Failed to write to stdin");
        });
        let output = child.wait_with_output().unwrap();
        (info, output)
    }
}

impl<'a, T: AsRef<OsStr>> Spawn for &'a [T] {
    fn spawn_with_info(&mut self) -> (Info, Output) {
        let mut cmd = Command::new(self.get(0).expect("expected program name as first item"));
        for arg in &self[1..] {
            cmd.arg(arg);
        }
        cmd.spawn_with_info()
    }
}

impl<T: AsRef<OsStr>, const N: usize> Spawn for [T; N] {
    fn spawn_with_info(&mut self) -> (Info, Output) {
        (&self[..]).spawn_with_info()
    }
}

impl<T: AsRef<OsStr>> Spawn for Vec<T> {
    fn spawn_with_info(&mut self) -> (Info, Output) {
        (&self[..]).spawn_with_info()
    }
}
