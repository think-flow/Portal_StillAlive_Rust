use anyhow::bail;
use std::path::Path;

pub fn play<P: AsRef<Path>>(file_name: P) -> anyhow::Result<()> {
    play_core(file_name)?;
    Ok(())
}

/// linux下 使用 mpg123 播放音乐
#[cfg(target_os = "linux")]
fn play_core<P: AsRef<Path>>(file_name: P) -> anyhow::Result<()> {
    use anyhow::Context;
    use std::process::Command;

    ensure_mpg123_is_available()?;

    Command::new("mpg123")
        .arg("-q") // 静默模式
        .arg(file_name.as_ref())
        .spawn()
        .context("play failed")?;

    Ok(())
}

#[cfg(target_os = "linux")]
fn ensure_mpg123_is_available() -> anyhow::Result<()> {
    use anyhow::Context;
    use std::process::Command;

    // 执行 `which mpg123` 命令
    let output = Command::new("which")
        .arg("mpg123")
        .output()
        .context("which command not found")?;

    // 检查命令是否成功执行且输出不为空
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if stdout.is_empty() {
        bail!("mpg123 command not found\nUse the \"sudo apt install mpg123\" to install");
    }

    Ok(())
}

/// windows下，使用 winmm.dll 播放音乐
#[cfg(target_os = "windows")]
fn play_core<P: AsRef<Path>>(file_name: P) -> anyhow::Result<()> {
    let path = file_name.as_ref();

    mci_send_string("Close All")?;

    mci_send_string(&format!("Play {}", path.to_string_lossy()))?;

    Ok(())
}

#[cfg(target_os = "windows")]
#[link(name = "winmm")]
unsafe extern "system" {
    fn mciSendStringW(
        lpszCommand: *const u16,
        lpszReturnString: *mut u16,
        cchReturn: u32,
        hwndCallback: *mut std::ffi::c_void,
    ) -> u32;

    fn mciGetErrorStringW(fdwError: u32, lpszErrorText: *mut u16, cchErrorText: u32) -> bool;
}

#[cfg(target_os = "windows")]
fn mci_send_string(command: &str) -> anyhow::Result<()> {
    use std::ptr::null_mut;
    use std::{ffi::OsStr, iter::once, os::windows::ffi::OsStrExt};

    let command_u16: Vec<u16> = OsStr::new(command).encode_wide().chain(once(0)).collect();

    unsafe {
        let res = mciSendStringW(command_u16.as_ptr(), null_mut(), 0, null_mut());

        if res != 0 {
            let mut err_str =
                format!("Error executing MCI command '{command}'. Error code: {res}. Error Msg: ");
            let mut buffer = [0u16; 256];

            let _ = mciGetErrorStringW(res, buffer.as_mut_ptr(), buffer.len() as u32);

            let s = String::from_utf16(&buffer)?;
            err_str.push_str(&s);

            bail!(err_str)
        }
    }

    Ok(())
}
