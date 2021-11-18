mod debug;
mod handle;
mod privilege;
mod process;
mod wide_string;

use windows::{
    core::Result,
    Win32::System::{RemoteDesktop::ProcessIdToSessionId, Threading::GetCurrentProcessId},
};

use crate::{debug::take_memory_dump, privilege::set_debug_privilege, process::ProcessIterator};

fn main() -> Result<()> {
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

    // Find the dwm for the session
    println!("Looking for the dwm process of the current session...");
    let process_id = find_dwm_process_id_in_session(current_session)?
        .expect("Could not find a dwm process for this session!");
    println!("Found dwm.exe with pid: {}", process_id);

    // Take the memory dump
    println!("Taking memory dump...");
    take_memory_dump(process_id, "dwmdump.dmp")?;

    println!("Done!");
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
