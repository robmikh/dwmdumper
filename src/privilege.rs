use windows::{
    core::Result,
    Win32::{
        Foundation::{HANDLE, LUID, PWSTR},
        Security::{
            AdjustTokenPrivileges, LookupPrivilegeValueW, LUID_AND_ATTRIBUTES,
            SE_PRIVILEGE_ENABLED, TOKEN_ADJUST_PRIVILEGES, TOKEN_PRIVILEGES,
            TOKEN_PRIVILEGES_ATTRIBUTES,
        },
        System::Threading::{GetCurrentProcess, OpenProcessToken},
    },
};

use crate::{
    handle::AutoCloseHandle,
    wide_string::{ToWide, WideString},
};

pub fn set_debug_privilege(enable: bool) -> Result<()> {
    let token = get_process_token()?;
    let se_debug_name = get_debug_privilege_name();
    set_privilege(&token.0, se_debug_name.as_pwstr(), enable)?;
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

fn set_privilege(token: &HANDLE, privilege_name: PWSTR, enable: bool) -> Result<()> {
    let mut luid = LUID::default();
    let attribute = if enable {
        SE_PRIVILEGE_ENABLED
    } else {
        TOKEN_PRIVILEGES_ATTRIBUTES(0)
    };
    unsafe {
        LookupPrivilegeValueW(None, privilege_name, &mut luid).ok()?;
        let token_privileges = TOKEN_PRIVILEGES {
            PrivilegeCount: 1,
            Privileges: [LUID_AND_ATTRIBUTES {
                Luid: luid,
                Attributes: attribute,
            }],
            ..Default::default()
        };
        AdjustTokenPrivileges(
            token,
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

// Workaround for missing SE_DEBUG_NAME
// https://docs.microsoft.com/en-us/windows/win32/secauthz/privilege-constants
fn get_debug_privilege_name() -> WideString {
    let string = "SeDebugPrivilege".to_wide();
    string
}
