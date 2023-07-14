use windows::{
    core::{Result, HSTRING},
    Win32::{
        Foundation::{GENERIC_READ, GENERIC_WRITE},
        Storage::FileSystem::{CreateFileW, CREATE_ALWAYS, FILE_ATTRIBUTE_NORMAL, FILE_SHARE_MODE},
        System::{
            Diagnostics::Debug::{
                MiniDumpWithAvxXStateContext, MiniDumpWithFullMemory, MiniDumpWithFullMemoryInfo,
                MiniDumpWithHandleData, MiniDumpWithIptTrace, MiniDumpWithThreadInfo,
                MiniDumpWithTokenInformation, MiniDumpWithUnloadedModules, MiniDumpWriteDump,
            },
            Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ},
        },
    },
};

use crate::handle::AutoCloseHandle;

pub fn take_memory_dump(process_id: u32, file_name: &str) -> Result<()> {
    // This might fail if we aren't running as admin
    let process_handle = unsafe {
        let handle = OpenProcess(
            PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
            false,
            process_id,
        )?;
        AutoCloseHandle(handle)
    };

    unsafe {
        // Create the dump file on disk
        let handle = {
            let handle = CreateFileW(
                &HSTRING::from(file_name),
                GENERIC_READ.0 | GENERIC_WRITE.0,
                FILE_SHARE_MODE(0),
                None,
                CREATE_ALWAYS,
                FILE_ATTRIBUTE_NORMAL,
                None,
            )?;
            AutoCloseHandle(handle)
        };

        // Write the dump to the file
        MiniDumpWriteDump(
            process_handle.0,
            process_id,
            handle.0,
            MiniDumpWithFullMemory
                | MiniDumpWithHandleData
                | MiniDumpWithUnloadedModules
                | MiniDumpWithFullMemoryInfo
                | MiniDumpWithThreadInfo
                | MiniDumpWithTokenInformation
                | MiniDumpWithAvxXStateContext
                | MiniDumpWithIptTrace,
            None,
            None,
            None,
        )
        .ok()?;
    }

    Ok(())
}
