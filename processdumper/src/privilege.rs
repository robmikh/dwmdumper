use windows::{
    core::{Result, PCWSTR},
    Win32::{
        Foundation::{HANDLE, LUID},
        Security::{
            AdjustTokenPrivileges, LookupPrivilegeValueW, LUID_AND_ATTRIBUTES, SE_DEBUG_NAME,
            SE_PRIVILEGE_ENABLED, TOKEN_ADJUST_PRIVILEGES, TOKEN_PRIVILEGES,
            TOKEN_PRIVILEGES_ATTRIBUTES,
        },
        System::Threading::{GetCurrentProcess, OpenProcessToken},
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
        OpenProcessToken(process_handle, TOKEN_ADJUST_PRIVILEGES, &mut result)?;
        Ok(AutoCloseHandle(result))
    }
}

fn set_privilege(token: &HANDLE, privilege_name: PCWSTR, enable: bool) -> Result<()> {
    let mut luid = LUID::default();
    let attribute = if enable {
        SE_PRIVILEGE_ENABLED
    } else {
        TOKEN_PRIVILEGES_ATTRIBUTES(0)
    };
    unsafe {
        LookupPrivilegeValueW(None, privilege_name, &mut luid)?;
        let token_privileges = TOKEN_PRIVILEGES {
            PrivilegeCount: 1,
            Privileges: [LUID_AND_ATTRIBUTES {
                Luid: luid,
                Attributes: attribute,
            }],
            ..Default::default()
        };
        AdjustTokenPrivileges(*token, false, Some(&token_privileges), 0, None, None)?;
    }
    Ok(())
}
