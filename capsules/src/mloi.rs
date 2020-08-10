//! Minimum Level of Interoperability (MLOI)
//! This file defines several constants and data types necessary for the implementation
//! of VPP
//!
//! - Author: Kaisser Ben Dlala
//! - Date: Aug 2020
#![allow(non_camel_case_types)]
#![allow(dead_code)]
// use crate::process::Process;
// use crate::process::ProcessType;
// use crate::platform::Chip;
///the 8-bit, 16-bit, 32-bit and 64-bit unsigned types are already defined as
/// u8, u16, u32, u64 respectively. No need to re-define them.

/// Enumerated VPP firmware/software type
// missing

type MK_Index_t = u16 ; /// Index of an element in a typed array
//type MK_IPC_ID_u = u8 ; /// Composite Identifier of an IPC
//type MK_MAILBOX_ID_u = u8 ; /// Composite Identifier of a Mailbox
// type MK_PROCESS_ID_u= u8 ; /// Composite Identifier of a Process
// type MK_Process_Priority_e = u8 ; /// Priority of a Process
// type MK_LIB_ID_u = u8 ; /// Composite Identifier of a shared library

/// Memory Address to a Process entry point
//PROCESS_Function_t
/// Memory Address to a LLOS entry point
// PLLOS_Function_t
/// Stack element
// StackType_t
/// Unique Universal Identifier
type UUID_t = u128;
/// void 32-bit
type v32_u = u32 ;
/// 32-bit bitmap for Exception, Signal or LIB Descriptor conveyor

/// Enumerated Signal type

/// Handle to a Kernel Object
// pub struct VppProcess{
// process: &dyn ProcessType,
// VPPstate: VPPSTATE,
// }

/// Time unsigned 64-bit integer
type MK_TIME_t = u64;

/// VPP States
pub enum VppState {
    READY,
    RUNNING,
    SUSPENDED_R,
    WAITING,
    SUSPENDED_W,
    SYNC,
    SUSPENDED_S,
    DEAD,
    ANY_STATE,
}

/// Priority Values of a Process (Table 7-4)
pub enum MK_PROCESS_PRIORITY_e{
    /// Lowest Priority
    MK_PROCESS_PRIORITY_LOW,
    /// Normal Priority(Default)
    MK_PROCESS_PRIORITY_NORMAL,
    /// Highest Priority
    MK_PROCESS_PRIORITY_HIGH,
    /// Indicates error when Process Priority is retrieved
    MK_PROCESS_PRIORITY_ERROR,
}
impl From<MK_PROCESS_PRIORITY_e> for u16 {
    fn from(original: MK_PROCESS_PRIORITY_e) -> u16 {
        match original{
            MK_PROCESS_PRIORITY_e::MK_PROCESS_PRIORITY_LOW => 0x0000,
            MK_PROCESS_PRIORITY_e::MK_PROCESS_PRIORITY_NORMAL => 0x0004,
            MK_PROCESS_PRIORITY_e::MK_PROCESS_PRIORITY_HIGH => 0x0008,
            MK_PROCESS_PRIORITY_e::MK_PROCESS_PRIORITY_ERROR => 0xFFFF,
        }
    }
}

/// VRE Identifiers (Table 7-5)
pub enum MK_VRE_e{
    /// Access to the interfaces of the AES Function
    MK_VRE_AES,
    /// Access to the interface of the ECC Function
    MK_VRE_ECC,
    /// Access to the interface of the RSA Function
    MK_VRE_RSA,
    /// Access to the interface of the Long-term credentials storage
    MK_VRE_ROT,
    /// Access to the interface of the Hash Function
    MK_VRE_HASH,
    /// Access to the interface of the RNG Function
    MK_VRE_RNG,
    /// Access to the interface of the Remote Audit Function
    MK_VRE_RAF,
    /// Access to the interface of additional Exectuion Domain hardware Functions
    MK_VRE_DOMAIN_BASE,
}
impl From<MK_VRE_e> for u32 {
    fn from(original: MK_VRE_e) -> u32 {
        match original {
            MK_VRE_e::MK_VRE_AES => 0x01,
            MK_VRE_e::MK_VRE_ECC => 0x04,
            MK_VRE_e::MK_VRE_RSA => 0x08,
            MK_VRE_e::MK_VRE_ROT => 0x10,
            MK_VRE_e::MK_VRE_HASH => 0x20,
            MK_VRE_e::MK_VRE_RNG => 0x40,
            MK_VRE_e::MK_VRE_RAF => 0x80,
            MK_VRE_e::MK_VRE_DOMAIN_BASE => 0x100,
        }
    }
}

/// Firmware/Software Types (Table 7-6)
pub enum VPP_FRW_TYPE_e {
    /// The firmware of a VPP Application
    FIRMWARE_SOFTWARE_TYPE_APP,
    /// The Primary Platform Software excluding the LLOS Software
    FIRMWARE_SOFTWARE_TYPE_VPP,
    /// The Firmware of the System VPP Application
    FIRMWARE_SOFTWARE_TYPE_SYSAPP,
    /// The Software of the LLOS
    FIRMWARE_SOFTWARE_TYPE_LLOS,
}
impl From<VPP_FRW_TYPE_e> for u8 {
    fn from(original: VPP_FRW_TYPE_e) -> u8 {
        match original {
            VPP_FRW_TYPE_e::FIRMWARE_SOFTWARE_TYPE_APP => 0x01,
            VPP_FRW_TYPE_e::FIRMWARE_SOFTWARE_TYPE_VPP => 0x02,
            VPP_FRW_TYPE_e::FIRMWARE_SOFTWARE_TYPE_SYSAPP => 0x04,
            VPP_FRW_TYPE_e::FIRMWARE_SOFTWARE_TYPE_LLOS => 0x08,
        }
    }
}
/// Scheduling Types (Table 7-7)
/// Collaborative Scheduling
pub const MK_SCHEDULING_TYPE_COLLABORATIVE: u8 = 0x01;
/// Pree-emptive Scheduling
pub const MK_SCHEDULING_TYPE_PREEMPTIVE: u8 = 0x02;

///Constants and Limits for Any Primary Platfrom (Table 7-9)


/// Exceptions (Table 7-11)
pub enum MK_EXCEPTION_e {
    /// An error has occured in a child of the Process
    MK_EXCEPTION_ERROR,
    /// A severe Exception has occured (e.g memory violation)
    MK_EXCEPTION_SEVERE,
    /// A child Process has died
    MK_EXCEPTION_CHILD_PROCESS_DIED,
    ///A VRE has been detached while a Process was waiting on it
    MK_EXCEPTION_VRE_DETACHED,
    /// Starting index for VPP implementation-specific Exceptions
    MK_EXCEPTION_VENDOR_BASE,
    /// Maximal Exception rank value allowed
    MK_EXCEPTION_MAX,
}
impl From<MK_EXCEPTION_e> for u16 {
    fn from(original: MK_EXCEPTION_e) -> u16 {
        match original {
            MK_EXCEPTION_e::MK_EXCEPTION_ERROR  => 0,
            MK_EXCEPTION_e::MK_EXCEPTION_SEVERE => 1,
            MK_EXCEPTION_e::MK_EXCEPTION_CHILD_PROCESS_DIED => 2,
            MK_EXCEPTION_e::MK_EXCEPTION_VRE_DETACHED => 3,
            MK_EXCEPTION_e::MK_EXCEPTION_VENDOR_BASE => 16,
            MK_EXCEPTION_e::MK_EXCEPTION_MAX => 31
        }
    }
}
/// Errors (Table 7-12)
pub enum MK_ERROR_e {
    /// No error
    MK_ERROR_NONE,
    /// Unknown UUID
    MK_ERROR_UNKNOWN_UUID,
    /// Severe error
    MK_ERROR_SEVERE,
    /// Illegal Parameter
    MK_ERROR_ILLEGAL_PARAMETER,
    /// Uknown identifier
    MK_ERROR_UNKNOWN_ID,
    /// Unknown Handle
    MK_ERROR_UNKNOWN_HANDLE,
    /// Unknown priority
    MK_ERROR_UNKNOWN_PRIORITY,
    /// Access denied
    MK_ERROR_ACCESS_DENIED,
    /// Internal Error
    MK_ERROR_INTERNAL,
    /// Reserved for VPP imlementation-specific
    MK_ERROR_VENDOR_BASE,
    /// Maximal error value
    MK_ERROR_MAX,
}
impl From<MK_ERROR_e> for u16 {
    fn from (original: MK_ERROR_e) -> u16 {
        match original {
            MK_ERROR_e::MK_ERROR_NONE => 0 ,
            MK_ERROR_e::MK_ERROR_UNKNOWN_UUID => 1,
            MK_ERROR_e::MK_ERROR_SEVERE => 2 ,
            MK_ERROR_e::MK_ERROR_ILLEGAL_PARAMETER => 3,
            MK_ERROR_e::MK_ERROR_UNKNOWN_ID => 4 ,
            MK_ERROR_e::MK_ERROR_UNKNOWN_HANDLE => 5 ,
            MK_ERROR_e::MK_ERROR_UNKNOWN_PRIORITY => 6 ,
            MK_ERROR_e::MK_ERROR_ACCESS_DENIED => 7,
            MK_ERROR_e::MK_ERROR_INTERNAL => 8,
            MK_ERROR_e::MK_ERROR_VENDOR_BASE => 32,
            MK_ERROR_e::MK_ERROR_MAX => 255
        }
    }
}
