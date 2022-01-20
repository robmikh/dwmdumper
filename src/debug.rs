use windows::{
    core::{Handle, Result},
    Win32::{
        Storage::FileSystem::{
            CreateFileW, CREATE_ALWAYS, FILE_ACCESS_FLAGS, FILE_ATTRIBUTE_NORMAL, FILE_SHARE_MODE,
        },
        System::{
            Diagnostics::Debug::{
                MiniDumpWithAvxXStateContext, MiniDumpWithFullMemory, MiniDumpWithFullMemoryInfo,
                MiniDumpWithHandleData, MiniDumpWithIptTrace, MiniDumpWithThreadInfo,
                MiniDumpWithTokenInformation, MiniDumpWithUnloadedModules, MiniDumpWriteDump,
            },
            SystemServices::{GENERIC_READ, GENERIC_WRITE},
            Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ},
        },
    },
};

use crate::{handle::AutoCloseHandle, wide_string::ToWide};

pub fn take_memory_dump(process_id: u32, file_name: &str) -> Result<()> {
    // This might fail if we aren't running as admin
    let process_handle = unsafe {
        let handle = OpenProcess(
            PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
            false,
            process_id,
        )
        .ok()?;
        AutoCloseHandle(handle)
    };

    unsafe {
        // Create the dump file on disk
        let handle = {
            let file_name = file_name.to_wide();
            let handle = CreateFileW(
                file_name.as_pwstr(),
                FILE_ACCESS_FLAGS(GENERIC_READ | GENERIC_WRITE),
                FILE_SHARE_MODE(0),
                std::ptr::null(),
                CREATE_ALWAYS,
                FILE_ATTRIBUTE_NORMAL,
                None,
            )
            .ok()?;
            AutoCloseHandle(handle)
        };

        // Write the dump to the file
        MiniDumpWriteDump(
            &process_handle.0,
            process_id,
            &handle.0,
            MiniDumpWithFullMemory
                | MiniDumpWithHandleData
                | MiniDumpWithUnloadedModules
                | MiniDumpWithFullMemoryInfo
                | MiniDumpWithThreadInfo
                | MiniDumpWithTokenInformation
                | MiniDumpWithAvxXStateContext
                | MiniDumpWithIptTrace,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        )
        .ok()?;
    }

    Ok(())
}
