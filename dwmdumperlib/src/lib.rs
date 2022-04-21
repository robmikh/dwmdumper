mod debug;
mod handle;
mod integrity;
mod privilege;
mod process;

pub use integrity::IntegrityLevel;
pub use privilege::set_debug_privilege;

use integrity::{get_current_process_token, get_integrity_level_from_process_token};
use process::ProcessIterator;
use windows::{
    core::Result,
    Win32::System::{RemoteDesktop::ProcessIdToSessionId, Threading::GetCurrentProcessId},
};

use crate::debug::take_memory_dump;

pub fn get_session_for_current_process() -> Result<u32> {
    unsafe {
        let current_pid = GetCurrentProcessId();
        let mut session = 0;
        ProcessIdToSessionId(current_pid, &mut session).ok()?;
        Ok(session)
    }
}

pub fn find_and_dump_dwm(session_id: u32, file_name: &str) -> Result<()> {
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

pub fn find_dwm_process_id_in_session(session_id: u32) -> Result<Option<u32>> {
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

pub fn get_integrity_level_for_current_process() -> Result<IntegrityLevel> {
    let process_token = get_current_process_token();
    get_integrity_level_from_process_token(&process_token)
}
