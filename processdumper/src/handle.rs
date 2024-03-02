use windows::Win32::Foundation::{CloseHandle, HANDLE};

#[repr(transparent)]
pub struct AutoCloseHandle(pub HANDLE);

impl Drop for AutoCloseHandle {
    fn drop(&mut self) {
        unsafe {
            let _ = CloseHandle(self.0);
        }
    }
}
