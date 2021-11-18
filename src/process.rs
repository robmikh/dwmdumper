use windows::{
    core::Result,
    Win32::System::WindowsProgramming::{
        NtQuerySystemInformation, SystemProcessInformation, SYSTEM_PROCESS_INFORMATION,
    },
};

pub struct ProcessInfo {
    name: String,
    process_id: u32,
    session_id: u32,
}

pub struct ProcessIterator {
    data: Vec<u8>,
    current: *const SYSTEM_PROCESS_INFORMATION,
}

impl ProcessIterator {
    pub fn new() -> Result<Self> {
        let data = unsafe {
            let mut processes_len_bytes = 0;
            let _ = NtQuerySystemInformation(
                SystemProcessInformation,
                std::ptr::null_mut(),
                0,
                &mut processes_len_bytes,
            );
            let mut processes_data = vec![0u8; processes_len_bytes as usize];
            NtQuerySystemInformation(
                SystemProcessInformation,
                processes_data.as_mut_ptr() as *mut _,
                processes_len_bytes,
                &mut processes_len_bytes,
            )?;
            assert_eq!(processes_len_bytes as usize, processes_data.len());
            processes_data
        };
        Ok(Self {
            data,
            current: std::ptr::null(),
        })
    }
}

impl Iterator for ProcessIterator {
    type Item = ProcessInfo;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == std::ptr::null() {
            self.current = self.data.as_ptr() as *const SYSTEM_PROCESS_INFORMATION;
        } else {
            unsafe {
                let process_info = self.current.as_ref().unwrap();
                if process_info.NextEntryOffset == 0 {
                    return None;
                }
                self.current = (self.current as *const u8)
                    .offset(process_info.NextEntryOffset as isize)
                    as *const _;
            }
        }
        unsafe {
            let process_info = self.current.as_ref().unwrap();
            Some(ProcessInfo::new(process_info))
        }
    }
}

impl ProcessInfo {
    fn new(info: &SYSTEM_PROCESS_INFORMATION) -> Self {
        let process_id = info.UniqueProcessId.0 as u32;
        let session_id = info.SessionId;
        let name = if process_id != 0 {
            unsafe {
                let slice = std::slice::from_raw_parts(
                    info.ImageName.Buffer.0,
                    info.ImageName.Length as usize,
                );
                // Ideally we wouldn't use the lossy version... but it fails on some names
                String::from_utf16_lossy(slice)
            }
        } else {
            "System Idle Process".to_owned()
        };
        Self {
            name,
            process_id,
            session_id,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn process_id(&self) -> u32 {
        self.process_id
    }

    pub fn session_id(&self) -> u32 {
        self.session_id
    }
}
