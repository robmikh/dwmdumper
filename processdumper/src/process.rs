use windows::{
    core::Result,
    Wdk::System::SystemInformation::{NtQuerySystemInformation, SystemProcessInformation},
    Win32::System::WindowsProgramming::SYSTEM_PROCESS_INFORMATION,
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
            let mut processes_data;
            // Sometimes the number of bytes can change inbetween calls. Keep
            // trying until it's stable.
            loop {
                let _ = NtQuerySystemInformation(
                    SystemProcessInformation,
                    std::ptr::null_mut(),
                    0,
                    &mut processes_len_bytes,
                );
                processes_data = vec![0u8; processes_len_bytes as usize];
                NtQuerySystemInformation(
                    SystemProcessInformation,
                    processes_data.as_mut_ptr() as *mut _,
                    processes_len_bytes,
                    &mut processes_len_bytes,
                )
                .ok()?;
                if processes_len_bytes as usize == processes_data.len() {
                    break;
                }
            }
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
            let mut name = unsafe {
                let slice = std::slice::from_raw_parts(
                    info.ImageName.Buffer.0,
                    info.ImageName.Length as usize,
                );
                // Ideally we wouldn't use the lossy version... but it fails on some names
                String::from_utf16_lossy(slice)
            };
            truncate_to_first_null_char(&mut name);
            name
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

fn truncate_to_first_null_char(input: &mut String) {
    if let Some(index) = input.find('\0') {
        input.truncate(index);
    }
}
