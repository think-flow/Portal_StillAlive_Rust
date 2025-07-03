use std::path::PathBuf;
use std::{env, fs};

fn main() {
    // 只要 sa1.mp3 发生变化就重新执行 build.rs
    println!("cargo:rerun-if-changed=sa1.mp3");

    let profile = env::var("PROFILE").unwrap(); // debug 或 release

    // 构造 target/debug 或 target/release 路径
    let mut target_dir = PathBuf::from("target").join(&profile);

    // Windows 上，Cargo 有时会在 target/debug/deps 下执行，可兼容 fallback
    if !target_dir.exists() {
        if let Ok(p) = env::var("CARGO_TARGET_DIR") {
            target_dir = PathBuf::from(p).join(&profile);
        }
    }

    // 复制文件
    fs::copy("sa1.mp3", target_dir.join("sa1.mp3"))
        .expect("Failed to copy sa1.mp3 to target directory");
}
