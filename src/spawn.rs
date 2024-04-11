use std::collections::BTreeMap;
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
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    env: BTreeMap<String, String>,
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
    fn from_std_command(cmd: &Command, stdin: Option<&[u8]>) -> Info {
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
            stdin: stdin.as_ref().map(|x| String::from_utf8_lossy(x).into()),
        }
    }
}

/// Implemented by different types that can be spawned and snapshotted.
pub trait Spawn {
    #[doc(hidden)]
    fn spawn_with_info(&mut self, stdin: Option<Vec<u8>>) -> (Info, Output);
}

/// Utility methods for spawning.
pub trait SpawnExt {
    /// This passes the given input to stdin before spawning the command.
    fn pass_stdin(&mut self, stdin: impl Into<Vec<u8>>) -> SpawnWithStdin<'_>;
}

impl<T: Spawn> SpawnExt for T {
    fn pass_stdin(&mut self, stdin: impl Into<Vec<u8>>) -> SpawnWithStdin<'_> {
        SpawnWithStdin {
            spawn: self,
            stdin: stdin.into(),
        }
    }
}

pub struct SpawnWithStdin<'a> {
    spawn: &'a mut dyn Spawn,
    stdin: Vec<u8>,
}

impl<'a> Spawn for SpawnWithStdin<'a> {
    fn spawn_with_info(&mut self, stdin: Option<Vec<u8>>) -> (Info, Output) {
        self.spawn
            .spawn_with_info(Some(stdin.unwrap_or(mem::take(&mut self.stdin))))
    }
}

impl Spawn for Command {
    fn spawn_with_info(&mut self, stdin: Option<Vec<u8>>) -> (Info, Output) {
        let info = Info::from_std_command(self, stdin.as_deref());
        let output = if let Some(stdin) = stdin {
            self.stdin(Stdio::piped());
            self.stdout(Stdio::piped());
            self.stderr(Stdio::piped());
            let mut child = self.spawn().unwrap();
            let mut child_stdin = child.stdin.take().expect("Failed to open stdin");
            std::thread::spawn(move || {
                child_stdin
                    .write_all(&stdin)
                    .expect("Failed to write to stdin");
            });
            child.wait_with_output().unwrap()
        } else {
            self.output().unwrap()
        };
        (info, output)
    }
}

impl<'a> Spawn for &'a mut Command {
    fn spawn_with_info(&mut self, stdin: Option<Vec<u8>>) -> (Info, Output) {
        <Command as Spawn>::spawn_with_info(self, stdin)
    }
}

/// Like [`Command`] but sends some input to stdin.
#[deprecated = "Use .pass_stdin(...) instead"]
pub struct StdinCommand {
    command: Command,
    stdin: Vec<u8>,
}

#[allow(deprecated)]
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

#[allow(deprecated)]
impl Deref for StdinCommand {
    type Target = Command;

    fn deref(&self) -> &Self::Target {
        &self.command
    }
}

#[allow(deprecated)]
impl DerefMut for StdinCommand {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.command
    }
}

#[allow(deprecated)]
impl Spawn for StdinCommand {
    fn spawn_with_info(&mut self, stdin: Option<Vec<u8>>) -> (Info, Output) {
        Command::spawn_with_info(
            &mut self.command,
            Some(stdin.unwrap_or(mem::take(&mut self.stdin))),
        )
    }
}

impl<'a, T: AsRef<OsStr>> Spawn for &'a [T] {
    fn spawn_with_info(&mut self, stdin: Option<Vec<u8>>) -> (Info, Output) {
        let mut cmd = Command::new(self.first().expect("expected program name as first item"));
        for arg in &self[1..] {
            cmd.arg(arg);
        }
        cmd.spawn_with_info(stdin)
    }
}

impl<T: AsRef<OsStr>, const N: usize> Spawn for [T; N] {
    fn spawn_with_info(&mut self, stdin: Option<Vec<u8>>) -> (Info, Output) {
        (&self[..]).spawn_with_info(stdin)
    }
}

impl<T: AsRef<OsStr>> Spawn for Vec<T> {
    fn spawn_with_info(&mut self, stdin: Option<Vec<u8>>) -> (Info, Output) {
        (&self[..]).spawn_with_info(stdin)
    }
}
