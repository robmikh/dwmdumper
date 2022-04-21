use windows::{
    core::Result,
    Win32::{
        Foundation::{ERROR_INSUFFICIENT_BUFFER, HANDLE},
        Security::{
            GetSidSubAuthority, GetSidSubAuthorityCount, GetTokenInformation, TokenIntegrityLevel,
            TOKEN_MANDATORY_LABEL,
        },
        System::SystemServices::{
            SECURITY_MANDATORY_HIGH_RID, SECURITY_MANDATORY_LOW_RID,
            SECURITY_MANDATORY_MEDIUM_PLUS_RID, SECURITY_MANDATORY_MEDIUM_RID,
            SECURITY_MANDATORY_PROTECTED_PROCESS_RID, SECURITY_MANDATORY_SYSTEM_RID,
            SECURITY_MANDATORY_UNTRUSTED_RID,
        },
    },
};

// TODO: Why is this the only one expressed as a u32?
const FIXED_SECURITY_MANDATORY_MEDIUM_PLUS_RID: i32 = SECURITY_MANDATORY_MEDIUM_PLUS_RID as i32;

#[repr(i32)]
pub enum IntegrityLevel {
    Untrusted = SECURITY_MANDATORY_UNTRUSTED_RID,
    Low = SECURITY_MANDATORY_LOW_RID,
    Medium = SECURITY_MANDATORY_MEDIUM_RID,
    MediumPlus = FIXED_SECURITY_MANDATORY_MEDIUM_PLUS_RID,
    High = SECURITY_MANDATORY_HIGH_RID,
    System = SECURITY_MANDATORY_SYSTEM_RID,
    ProtectedProcess = SECURITY_MANDATORY_PROTECTED_PROCESS_RID,
}

impl IntegrityLevel {
    pub fn display_str(&self) -> &'static str {
        match self {
            IntegrityLevel::Untrusted => "Untrusted",
            IntegrityLevel::Low => "Low",
            IntegrityLevel::Medium => "Medium",
            IntegrityLevel::MediumPlus => "MediumPlus",
            IntegrityLevel::High => "High",
            IntegrityLevel::System => "System",
            IntegrityLevel::ProtectedProcess => "ProtectedProcess",
        }
    }

    pub fn is_admin(&self) -> bool {
        match self {
            IntegrityLevel::Untrusted => false,
            IntegrityLevel::Low => false,
            IntegrityLevel::Medium => false,
            IntegrityLevel::MediumPlus => false,
            IntegrityLevel::High => true,
            IntegrityLevel::System => true,
            IntegrityLevel::ProtectedProcess => true,
        }
    }
}

pub fn get_integrity_level_from_process_token(process_token: &HANDLE) -> Result<IntegrityLevel> {
    // Get the size of the data we'll get back
    let mut information_length = 0;
    unsafe {
        let result = GetTokenInformation(
            process_token,
            TokenIntegrityLevel,
            std::ptr::null_mut(),
            0,
            &mut information_length,
        )
        .ok();
        if let Err(error) = &result {
            if error.win32_error().unwrap() != ERROR_INSUFFICIENT_BUFFER {
                result?;
            }
        }
    }

    // Allocate the memory for the integrity level data
    let mut info_data = vec![0u8; information_length as usize];
    let info: *mut TOKEN_MANDATORY_LABEL = unsafe { std::mem::transmute(info_data.as_mut_ptr()) };

    // Get the data for our integrity level
    unsafe {
        GetTokenInformation(
            process_token,
            TokenIntegrityLevel,
            info as *mut _,
            information_length,
            &mut information_length,
        )
        .ok()?;
    }

    // Get the integrity level from the info we got back
    let authority_count = unsafe { *GetSidSubAuthorityCount((*info).Label.Sid) } as u32;
    let integrity_level =
        unsafe { *GetSidSubAuthority((*info).Label.Sid, authority_count - 1) } as i32;

    let result = match integrity_level {
        SECURITY_MANDATORY_UNTRUSTED_RID => IntegrityLevel::Untrusted,
        SECURITY_MANDATORY_LOW_RID => IntegrityLevel::Low,
        SECURITY_MANDATORY_MEDIUM_RID => IntegrityLevel::Medium,
        FIXED_SECURITY_MANDATORY_MEDIUM_PLUS_RID => IntegrityLevel::MediumPlus,
        SECURITY_MANDATORY_HIGH_RID => IntegrityLevel::High,
        SECURITY_MANDATORY_SYSTEM_RID => IntegrityLevel::System,
        SECURITY_MANDATORY_PROTECTED_PROCESS_RID => IntegrityLevel::ProtectedProcess,
        _ => IntegrityLevel::Untrusted,
    };
    Ok(result)
}

pub fn get_current_process_token() -> HANDLE {
    HANDLE(-4)
}
