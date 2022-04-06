mod debug;
mod handle;
mod hotkey;
mod privilege;
mod process;

use hotkey::HotKey;
use windows::{
    core::Result,
    Win32::{
        Foundation::HWND,
        System::{RemoteDesktop::ProcessIdToSessionId, Threading::GetCurrentProcessId},
        UI::{
            Input::KeyboardAndMouse::{MOD_CONTROL, MOD_SHIFT},
            WindowsAndMessaging::{DispatchMessageW, GetMessageW, MSG, WM_HOTKEY},
        },
    },
};

use crate::{debug::take_memory_dump, privilege::set_debug_privilege, process::ProcessIterator};

fn main() -> Result<()> {
    // TODO: Configurable file name/path
    let file_name = "dwmdump.dmp";

    // We first need to give ourselves debug privileges.
    set_debug_privilege(true)?;

    // During RDP sessions, you'll have multiple sessions and muiltple
    // DWMs. We want the one the user is currently using, so find the
    // session our program is running in.
    println!("Getting the current session...");
    let current_session = unsafe {
        let current_pid = GetCurrentProcessId();
        let mut session = 0;
        ProcessIdToSessionId(current_pid, &mut session).ok()?;
        session
    };
    println!("Current session id: {}", current_session);

    // Wait for the user to press the key-combo before we
    // look for the dwm and collect a dump.
    println!("Press SHIFT+CTRL+D to collect a dump of dwm.exe...");
    pump_messages(|| -> Result<bool> {
        find_and_dump_dwm(current_session, file_name)?;
        Ok(true)
    })?;

    // TODO: Properly evaulate this
    let mut dump_path = std::env::current_dir().unwrap();
    dump_path.push(file_name);

    println!("Done! Dump written to \"{}\"", dump_path.display());
    Ok(())
}

fn find_and_dump_dwm(session_id: u32, file_name: &str) -> Result<()> {
    // Find the dwm for the session
    println!("Looking for the dwm process of the current session...");
    let process_id = find_dwm_process_id_in_session(session_id)?
        .expect("Could not find a dwm process for this session!");
    println!("Found dwm.exe with pid: {}", process_id);

    // Take the memory dump
    println!("Taking memory dump...");
    take_memory_dump(process_id, file_name)?;

    Ok(())
}

fn find_dwm_process_id_in_session(session_id: u32) -> Result<Option<u32>> {
    let mut result = None;
    let process_iter = ProcessIterator::new()?;
    for process_info in process_iter {
        if process_info.name().starts_with("dwm.exe") {
            if process_info.session_id() == session_id {
                let process_id = process_info.process_id();
                result = Some(process_id);
                break;
            }
        }
    }
    Ok(result)
}

fn pump_messages<F: FnMut() -> Result<bool>>(mut hot_key_callback: F) -> Result<()> {
    let _hot_key = HotKey::new(MOD_SHIFT | MOD_CONTROL, 0x44 /* D */)?;
    unsafe {
        let mut message = MSG::default();
        while GetMessageW(&mut message, HWND(0), 0, 0).into() {
            if message.message == WM_HOTKEY {
                if hot_key_callback()? {
                    break;
                }
            }
            DispatchMessageW(&mut message);
        }
    }
    Ok(())
}
