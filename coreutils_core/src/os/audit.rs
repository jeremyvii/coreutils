//! Module of audit session compability for FreeBSD and MacOS.
//!
//! I got the info from FreeBSD man pages `GETAUDIT(2)`
//! the names defined on `GETAUDIT(2)` will have a '¹' on them.

use std::{
    fmt::{self, Display},
    io,
    mem::MaybeUninit,
};

#[cfg(target_os = "macos")]
use libc::dev_t;
use libc::{c_int, c_uint, pid_t, uid_t};

/// This type contains the audit identifier which is recorded in the audit log for each
/// event the process caused.
///
/// ¹Same as `au_id_t`
pub type AuditUserId = uid_t;

/// This type contains the audit session ID which is recorded with every event caused
/// by the process.
///
/// ¹Same as `au_asid_t`
pub type AuditSessionId = pid_t;

pub type AuditEvent = u16;
pub type AuditEmod = u16;
pub type AuditClass = u32;

/// This struct defines the bit mask for auditing successful and failed events out of the
/// predefined list of event classes.
///
/// ¹Same as `au_mask_t`
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(C)]
pub struct AuditMask {
    pub am_success: c_uint,
    pub am_failure: c_uint,
}

/// This struct defines the Terminal ID recorded with every event caused by the process.
///
/// ¹Same as `au_tid_t`
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(C)]
pub struct AuditTerminalId {
    #[cfg(target_os = "freebsd")]
    pub port: u32,
    #[cfg(target_os = "macos")]
    pub port: dev_t,
    pub machine: u32,
}

/// This struct includes a larger address storage field and an additional field with the
/// type of address stored.
///
/// ¹Same as `au_tid_addr_t`
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(C)]
pub struct AuditTerminalIdAddr {
    #[cfg(target_os = "freebsd")]
    pub at_port: u32,
    #[cfg(target_os = "macos")]
    pub at_port: dev_t,
    pub at_type: u32,
    pub at_addr: [u32; 4],
}

/// This struct represents a active audit session.
///
/// ¹Same as `auditinfo_t`
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(C)]
pub struct AuditInfo {
    /// Audit user ID
    pub ai_auid: AuditUserId,
    /// Audit masks.
    pub ai_mask: AuditMask,
    /// Terminal ID.
    pub ai_termid: AuditTerminalId,
    /// Audit session ID.
    pub ai_asid: AuditSessionId,
}

impl Display for AuditInfo {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "auid={}", self.ai_auid)?;
        writeln!(f, "mask.success={:#X}", self.ai_mask.am_success)?;
        writeln!(f, "mask.failure={:#X}", self.ai_mask.am_failure)?;
        writeln!(f, "asid={}", self.ai_asid)?;
        writeln!(f, "termid.port={:#X}", self.ai_termid.port)?;
        write!(f, "termid.machine={:#X}", self.ai_termid.machine)
    }
}

/// This struct represents a active audit session address.
///
/// ¹Same as `auditinfo_addr_t`
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(C)]
pub struct AuditInfoAddr {
    /// Audit user ID.
    pub ai_auid: AuditUserId,
    /// Audit masks.
    pub ai_mask: AuditMask,
    /// Terminal ID.
    pub ai_termid: AuditTerminalIdAddr,
    /// Audit session ID.
    pub ai_asid: AuditSessionId,
    /// Audit session flags.
    pub ai_flags: u64,
}

impl Display for AuditInfoAddr {
    // TODO: Incomplete, We need more info on how it is normally displayed.
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "auid={}", self.ai_auid)?;
        writeln!(f, "mask.success={:#X}", self.ai_mask.am_success)?;
        writeln!(f, "mask.failure={:#X}", self.ai_mask.am_failure)?;
        writeln!(f, "asid={}", self.ai_asid)?;
        writeln!(f, "termid.at_port={:#X}", self.ai_termid.at_port)?;
        write!(f, "termid.at_type={}", self.ai_termid.at_type)
    }
}

// TODO: Add other system calls related to audit
extern "C" {
    /// This system call retrieves the active audit session state for the current
    /// process via the `AuditInfo` pointed to by `auditinfo`.
    ///
    /// Returns `0` is successful, `-1` otherwise.
    pub fn getaudit(auditinfo: *mut AuditInfo) -> c_int;

    /// This system call uses the expanded `AuditInfoAddr` data structure and supports
    /// Terminal IDs with larger addresses such as those used in IP version 6.
    ///
    /// Returns `0` is successful, `-1` otherwise.
    pub fn getaudit_addr(auditinfo_addr: *mut AuditInfoAddr, length: c_int) -> c_int;
}

/// Returns the `AuditInfo`.
///
/// # Errors
/// If a internal call set a errno (I/O OS error), an error variant will be returned.
#[inline]
pub fn audit_info() -> io::Result<AuditInfo> {
    let mut auditinfo: MaybeUninit<AuditInfo> = MaybeUninit::zeroed();
    let address = auditinfo.as_mut_ptr() as *mut AuditInfo;

    if unsafe { getaudit(address) } == -1 {
        return Err(io::Error::last_os_error());
    }

    let auditinfo = unsafe { auditinfo.assume_init() };

    Ok(auditinfo)
}
