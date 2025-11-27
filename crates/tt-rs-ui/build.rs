//! Build script to capture git and build information.

use std::process::Command;

fn main() {
    let commit = git_commit();
    let host = hostname();
    let time = chrono::Utc::now().format("%Y-%m-%d %H:%M UTC").to_string();

    println!("cargo:rustc-env=BUILD_COMMIT={}", commit);
    println!("cargo:rustc-env=BUILD_HOST={}", host);
    println!("cargo:rustc-env=BUILD_TIME={}", time);
    println!("cargo:rerun-if-changed=.git/HEAD");
}

fn git_commit() -> String {
    Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string())
}

fn hostname() -> String {
    Command::new("hostname")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string())
}
