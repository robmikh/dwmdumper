mod hotkey;

use dwmdumperlib::{get_integrity_level_for_current_process, IntegrityLevel, set_debug_privilege, find_and_dump_dwm, get_session_for_current_process};
use hotkey::HotKey;
use windows::{
    core::Result,
    Win32::{
        Foundation::HWND,
        UI::{
            Input::KeyboardAndMouse::{MOD_CONTROL, MOD_SHIFT},
            WindowsAndMessaging::{DispatchMessageW, GetMessageW, MSG, WM_HOTKEY},
        },
    },
};

fn main() -> Result<()> {
    // TODO: Configurable file name/path
    let file_name = "dwmdump.dmp";

    // Make sure we're running as admin
    let integrity_level =
        get_integrity_level_for_current_process().unwrap_or(IntegrityLevel::Untrusted);
    println!(
        "Currently detected integrity level: {}",
        integrity_level.display_str()
    );
    if !integrity_level.is_admin() {
        println!("This tool requies admin privileges to properly dump the memory of dwm.exe.");
        println!("Please try again using an admin command prompt/terminal.");
        std::process::exit(1);
    }

    // We first need to give ourselves debug privileges.
    set_debug_privilege(true)?;

    // During RDP sessions, you'll have multiple sessions and muiltple
    // DWMs. We want the one the user is currently using, so find the
    // session our program is running in.
    println!("Getting the current session...");
    let current_session = get_session_for_current_process()?;
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
