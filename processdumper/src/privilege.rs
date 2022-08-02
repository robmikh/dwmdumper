use windows::{
    core::{Result, HSTRING},
    Win32::{
        Foundation::{HANDLE, LUID},
        Security::{
            AdjustTokenPrivileges, LookupPrivilegeValueW, LUID_AND_ATTRIBUTES,
            SE_PRIVILEGE_ENABLED, TOKEN_ADJUST_PRIVILEGES, TOKEN_PRIVILEGES,
            TOKEN_PRIVILEGES_ATTRIBUTES,
        },
        System::{
            SystemServices::SE_DEBUG_NAME,
            Threading::{GetCurrentProcess, OpenProcessToken},
        },
    },
};

use crate::handle::AutoCloseHandle;

pub fn set_debug_privilege(enable: bool) -> Result<()> {
    let token = get_process_token()?;
    set_privilege(&token.0, SE_DEBUG_NAME, enable)?;
    Ok(())
}

fn get_process_token() -> Result<AutoCloseHandle> {
    unsafe {
        // This is a pseudo-handle, so we don't need to close or check it.
        let process_handle = GetCurrentProcess();
        let mut result = HANDLE(0);
        OpenProcessToken(process_handle, TOKEN_ADJUST_PRIVILEGES, &mut result).ok()?;
        Ok(AutoCloseHandle(result))
    }
}

fn set_privilege(token: &HANDLE, privilege_name: &str, enable: bool) -> Result<()> {
    let mut luid = LUID::default();
    let attribute = if enable {
        SE_PRIVILEGE_ENABLED
    } else {
        TOKEN_PRIVILEGES_ATTRIBUTES(0)
    };
    unsafe {
        LookupPrivilegeValueW(None, &HSTRING::from(privilege_name), &mut luid).ok()?;
        let token_privileges = TOKEN_PRIVILEGES {
            PrivilegeCount: 1,
            Privileges: [LUID_AND_ATTRIBUTES {
                Luid: luid,
                Attributes: attribute,
            }],
            ..Default::default()
        };
        AdjustTokenPrivileges(
            *token,
            false,
            &token_privileges,
            0,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        )
        .ok()?;
    }
    Ok(())
}
