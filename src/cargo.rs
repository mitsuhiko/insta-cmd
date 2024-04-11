use std::env;
use std::path::PathBuf;
use std::process::Command;

use serde::Deserialize;

#[derive(Deserialize)]
struct MetaData {
    target_directory: PathBuf,
}

fn add_profile_dir(mut target_dir: PathBuf) -> PathBuf {
    if cfg!(debug_assertions) {
        target_dir.push("debug");
    } else {
        target_dir.push("release");
    }
    target_dir
}

fn primary_target_dir() -> PathBuf {
    // the target dir is explicitly set and exists
    if let Some(target_dir) = env::var_os("CARGO_TARGET_DIR") {
        let target_dir = add_profile_dir(PathBuf::from(target_dir));
        if target_dir.is_dir() {
            return target_dir;
        };
    }

    // try to guess it from the current exe
    env::current_exe()
        .ok()
        .map(|mut path| {
            path.pop();
            if path.ends_with("deps") || path.ends_with("examples") {
                path.pop();
            }
            path
        })
        .unwrap()
}

fn cargo_inferred_target_dir() -> PathBuf {
    let metadata = Command::new(env::var("CARGO").ok().unwrap_or_else(|| "cargo".into()))
        .arg("metadata")
        .output()
        .unwrap();
    let meta: MetaData = serde_json::from_slice(&metadata.stdout).unwrap();
    add_profile_dir(meta.target_directory)
}

fn find_exe(name: &str) -> PathBuf {
    let mut first = primary_target_dir();
    first.push(name);
    if first.is_file() {
        return first;
    }

    let mut alt = cargo_inferred_target_dir();
    alt.push(name);
    if alt.is_file() {
        return alt;
    }
    panic!("Cannot determine path to executable '{}'", name);
}

/// Helper function to return the path to an executable that cargo is building.
pub fn get_cargo_bin(name: &str) -> PathBuf {
    let env_var = format!("CARGO_BIN_EXE_{}", name);
    env::var_os(env_var)
        .map(|p| p.into())
        .unwrap_or_else(|| find_exe(&format!("{}{}", name, env::consts::EXE_SUFFIX)))
}

/// Helper function to return the path to an example that cargo is building.
pub fn get_cargo_example(name: &str) -> PathBuf {
    find_exe(&format!("examples/{}{}", name, env::consts::EXE_SUFFIX))
}
