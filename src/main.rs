mod handle;
mod process;

use windows::{
    core::Result,
    Win32::System::{RemoteDesktop::ProcessIdToSessionId, Threading::GetCurrentProcessId},
};

use crate::{handle::AutoCloseHandle, process::ProcessIterator};

fn main() -> Result<()> {
    println!("Getting the current session...");
    let current_session = unsafe {
        let current_pid = GetCurrentProcessId();
        let mut session = 0;
        ProcessIdToSessionId(current_pid, &mut session).ok()?;
        session
    };
    println!("Current session id: {}", current_session);

    println!("Looking for the dwm process of the current session...");
    let process_id = find_dwm_process_id_in_session(current_session)?
        .expect("Could not find a dwm process for this session!");
    println!("Found dwm.exe with pid: {}", process_id);

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
