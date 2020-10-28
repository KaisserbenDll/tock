//! Minimum Level of Interoperability (MLOI)
//! This file defines several constants and data types necessary for the implementation
//! of VPP
//!
//! - Author: Kaisser Ben Dlala
//! - Date: Aug 2020

#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(non_snake_case)]
//extern crate alloc;
//use alloc::boxed::Box;
//the 8-bit, 16-bit, 32-bit and 64-bit unsigned types are already defined as
// u8, u16, u32, u64 respectively. No need to re-define them.
use core::fmt;
/// XLEN is the size memory address. It is platform dependent value and for 32-bit platforms
/// equals 4 bytes.
pub (crate) type XLEN = u32;

/// Firmware/Software Types (Table 7-6)
pub  enum VPP_FRW_TYPE_e {
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

impl fmt::Debug for VPP_FRW_TYPE_e {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VPP_FRW_TYPE_e::FIRMWARE_SOFTWARE_TYPE_APP => {
                write!(f, "App\n")
            }
            VPP_FRW_TYPE_e::FIRMWARE_SOFTWARE_TYPE_VPP => {
                write!(f, "Vpp\n")
            }
            VPP_FRW_TYPE_e::FIRMWARE_SOFTWARE_TYPE_SYSAPP => {
                write!(f, "Sys App\n")
            }
            VPP_FRW_TYPE_e::FIRMWARE_SOFTWARE_TYPE_LLOS => {
                write!(f, "LLOS\n")
            }


        }
    }
}
/// Index of an element in a typed array
pub(crate) type MK_Index_t = u16 ;

/// Execution Domain Types (Table 7-3)
pub(crate) enum MK_EXECUTION_DOMAIN_TYPE_e {
    /// System VPP Execution Domain
    MK_EXECUTION_DOMAIN_TYPE_VPP ,
    /// VPP Application Execution Domain
    MK_EXECUTION_DOMAIN_TYPE_APP  ,
}

impl From<MK_EXECUTION_DOMAIN_TYPE_e> for u16 {
    fn from(original: MK_EXECUTION_DOMAIN_TYPE_e) -> Self {
        match original {
            MK_EXECUTION_DOMAIN_TYPE_e::MK_EXECUTION_DOMAIN_TYPE_APP => 0x0002_u16,
            MK_EXECUTION_DOMAIN_TYPE_e::MK_EXECUTION_DOMAIN_TYPE_VPP => 0x0001_u16,
        }
    }
}
/// This macro takes two arguments:
/// 1) The first argument is the execution domain type as u16
/// 2) The second argument is the Enumerated identifier
/// and returns a composite identifier. This macro should be used to generate all composite
/// identifiers used in the VPP standard (Table 7-2 + Table 7-13)
///
/// Since there is no need to use the MK_EXECUTION_DOMAIN_TYPE enum every time this macro is
/// called, i tried to call it with 1 for APP and 2 for VPP execution domain types.
    macro_rules! composite_id {
        ($x:expr,$y:expr) => {(($y as u16) & 0x3FFF) | (($x as u16) << 14)} //(($x as u16) << 14) |
    }

/// Composite Identifier of an IPC (Table 7-2)
pub type MK_IPC_ID_u = u16;
pub fn convert_ipcID_to_handle(_eIPCID: MK_IPC_ID_u) -> MK_HANDLE_t {_eIPCID as u32}
pub fn convert_handle_to_ipcID(handle: MK_HANDLE_t) -> MK_IPC_ID_u {handle as u16}
/// Composite Identifier of a Mailbox (Table 7-2)
pub type MK_MAILBOX_ID_u = u16 ;
pub fn convert_mbid_to_handle(_eMailboxID: MK_MAILBOX_ID_u) -> MK_HANDLE_t{_eMailboxID as u32}
pub fn convert_handle_to_mbid(handle: MK_HANDLE_t) -> MK_MAILBOX_ID_u{handle as u16}
/// Composite Identifier of a Process (Table 7-2)
pub type MK_Process_ID_u = u16;
pub fn convert_to_handle(id: MK_Process_ID_u) -> MK_HANDLE_t{id as u32}
pub fn convert_to_id(handle: MK_HANDLE_t) -> MK_Process_ID_u{handle as u16}


#[derive(Copy, Clone, Eq, PartialEq)]
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
impl fmt::Debug for MK_PROCESS_PRIORITY_e {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MK_PROCESS_PRIORITY_e::MK_PROCESS_PRIORITY_LOW => {
                write!(f, "Low\n")
            }
            MK_PROCESS_PRIORITY_e::MK_PROCESS_PRIORITY_NORMAL => {
                write!(f, "Normal\n")
            }
            MK_PROCESS_PRIORITY_e::MK_PROCESS_PRIORITY_HIGH => {
                write!(f, "High\n")
            }
            MK_PROCESS_PRIORITY_e::MK_PROCESS_PRIORITY_ERROR => {
                write!(f, "Priority\n")
            }


        }
    }
}

/// Composite Identifier of a shared library (Table 7-2)
pub type MK_LIB_ID_u = u16 ;
pub type MK_LIB_ID_e = u16 ;

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

/// Memory Address to a Process entry point
// should be changed depending on pointers handling in rust (as_ptr)
pub type PROCESS_Function_t = XLEN ;

/// Memory Address to a LLOS entry point
// PLLOS_Function_t

/// Stack element
// StackType_t

/// Unique Universal Identifier
pub  type UUID_t = u128;

/// void 32-bit
pub type v32_u = u32 ;

/// Errors (Table 7-12)
#[derive(Clone, Copy)]
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
impl From<MK_ERROR_e> for usize {
    fn from (original: MK_ERROR_e) -> usize {
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
impl fmt::Debug for MK_ERROR_e {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MK_ERROR_e::MK_ERROR_NONE => {
                write!(f, "None\n")
            }
            MK_ERROR_e::MK_ERROR_UNKNOWN_UUID => {
                write!(f, "Unknown UUID\n")
            }
            MK_ERROR_e::MK_ERROR_SEVERE => {
                write!(f, "Severe error\n")
            }
            MK_ERROR_e::MK_ERROR_ILLEGAL_PARAMETER => {
                write!(f, "Illegal Parameter\n")
            }
            MK_ERROR_e::MK_ERROR_UNKNOWN_ID => {
                write!(f, "Unknown identifier\n")
            }
            MK_ERROR_e::MK_ERROR_UNKNOWN_HANDLE => {
                write!(f, "Unknown Handle\n")
            }
            MK_ERROR_e::MK_ERROR_UNKNOWN_PRIORITY => {
                write!(f, "Unknown Priority\n")
            }
            MK_ERROR_e::MK_ERROR_ACCESS_DENIED => {
                write!(f, "Access Denied\n")
            }
            MK_ERROR_e::MK_ERROR_INTERNAL => {
                write!(f, "Internal Error\n")
            }
            MK_ERROR_e::MK_ERROR_VENDOR_BASE=> {
                write!(f, "Reserved for VPP implementation specific\n")
            }
            MK_ERROR_e::MK_ERROR_MAX => {
                write!(f, "Maximal Error Value\n")
            }
        }
    }
}

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

/// 32-bit bitmap for Exception, Signal or LIB Descriptor conveyor
pub type MK_BITMAP_t = u32;
/// Enumerated Signal type (table 7-8)
pub enum MK_SIGNAL_e{
    /// Timeout notification
    MK_SIGNAL_TIME_OUT,
    /// _mk_Get_Signal function or VRE has generated an error
    MK_SIGNAL_ERROR,
    /// Notification for an Exception from a child Process
    MK_SIGNAL_EXCEPTION,
    /// Mailbox defined Signals for generic use within the scope of
    /// the Execution Domains MK_EXECUTION_DOMAIN_TYPE_VPP and
    /// MK_EXECUTION_DOMAIN_TYPE_APP (Table 7-3)
    MK_SIGNAL_DOMAIN_BASE_0,
    MK_SIGNAL_DOMAIN_BASE_1,
    MK_SIGNAL_DOMAIN_BASE_2,
    MK_SIGNAL_DOMAIN_BASE_3,
    MK_SIGNAL_DOMAIN_BASE_4,
    MK_SIGNAL_DOMAIN_BASE_5,
    MK_SIGNAL_DOMAIN_BASE_6,
    MK_SIGNAL_DOMAIN_BASE_7,
    MK_SIGNAL_DOMAIN_BASE_8,
    MK_SIGNAL_DOMAIN_BASE_9,
    MK_SIGNAL_DOMAIN_BASE_10,
    MK_SIGNAL_DOMAIN_BASE_11,
    MK_SIGNAL_DOMAIN_BASE_12,
    MK_SIGNAL_DOMAIN_BASE_13,
    MK_SIGNAL_DOMAIN_BASE_14,
    MK_SIGNAL_DOMAIN_BASE_15,
    MK_SIGNAL_DOMAIN_BASE_16,
    MK_SIGNAL_DOMAIN_BASE_17,
    MK_SIGNAL_DOMAIN_BASE_18,
    MK_SIGNAL_DOMAIN_BASE_19,
    MK_SIGNAL_DOMAIN_BASE_20,
    MK_SIGNAL_DOMAIN_BASE_21,
    MK_SIGNAL_DOMAIN_BASE_22,
    MK_SIGNAL_DOMAIN_BASE_23,
    MK_SIGNAL_DOMAIN_BASE_24,
    MK_SIGNAL_DOMAIN_BASE_25,
    MK_SIGNAL_DOMAIN_BASE_26,
    MK_SIGNAL_DOMAIN_BASE_27,
    MK_SIGNAL_DOMAIN_BASE_28,
}

impl From<MK_SIGNAL_e> for u32 {
    fn from(original: MK_SIGNAL_e) -> u32 {
        match original {
            MK_SIGNAL_e::MK_SIGNAL_TIME_OUT =>       0x0000_0001_u32,
            MK_SIGNAL_e::MK_SIGNAL_ERROR =>          0x0000_0002_u32,
            MK_SIGNAL_e::MK_SIGNAL_EXCEPTION =>      0x0000_0004_u32,
            MK_SIGNAL_e::MK_SIGNAL_DOMAIN_BASE_0 =>  0x0000_0008_u32,
            MK_SIGNAL_e::MK_SIGNAL_DOMAIN_BASE_1 =>  0x0000_0010_u32,
            MK_SIGNAL_e::MK_SIGNAL_DOMAIN_BASE_2 =>  0x0000_0020_u32,
            MK_SIGNAL_e::MK_SIGNAL_DOMAIN_BASE_3 =>  0x0000_0040_u32,
            MK_SIGNAL_e::MK_SIGNAL_DOMAIN_BASE_4 =>  0x0000_0080_u32,
            MK_SIGNAL_e::MK_SIGNAL_DOMAIN_BASE_5 =>  0x0000_0100_u32,
            MK_SIGNAL_e::MK_SIGNAL_DOMAIN_BASE_6 =>  0x0000_0200_u32,
            MK_SIGNAL_e::MK_SIGNAL_DOMAIN_BASE_7 =>  0x0000_0400_u32,
            MK_SIGNAL_e::MK_SIGNAL_DOMAIN_BASE_8 =>  0x0000_0800_u32,
            MK_SIGNAL_e::MK_SIGNAL_DOMAIN_BASE_9 =>  0x0000_1000_u32,
            MK_SIGNAL_e::MK_SIGNAL_DOMAIN_BASE_10 => 0x0000_2000_u32,
            MK_SIGNAL_e::MK_SIGNAL_DOMAIN_BASE_11 => 0x0000_4000_u32,
            MK_SIGNAL_e::MK_SIGNAL_DOMAIN_BASE_12 => 0x0000_8000_u32,
            MK_SIGNAL_e::MK_SIGNAL_DOMAIN_BASE_13 => 0x0001_0000_u32,
            MK_SIGNAL_e::MK_SIGNAL_DOMAIN_BASE_14 => 0x0002_0000_u32,
            MK_SIGNAL_e::MK_SIGNAL_DOMAIN_BASE_15 => 0x0004_0000_u32,
            MK_SIGNAL_e::MK_SIGNAL_DOMAIN_BASE_16 => 0x0008_0000_u32,
            MK_SIGNAL_e::MK_SIGNAL_DOMAIN_BASE_17 => 0x0010_0000_u32,
            MK_SIGNAL_e::MK_SIGNAL_DOMAIN_BASE_18 => 0x0020_0000_u32,
            MK_SIGNAL_e::MK_SIGNAL_DOMAIN_BASE_19 => 0x0040_0000_u32,
            MK_SIGNAL_e::MK_SIGNAL_DOMAIN_BASE_20 => 0x0080_0000_u32,
            MK_SIGNAL_e::MK_SIGNAL_DOMAIN_BASE_21 => 0x0100_0000_u32,
            MK_SIGNAL_e::MK_SIGNAL_DOMAIN_BASE_22 => 0x0200_0000_u32,
            MK_SIGNAL_e::MK_SIGNAL_DOMAIN_BASE_23 => 0x0400_0000_u32,
            MK_SIGNAL_e::MK_SIGNAL_DOMAIN_BASE_24 => 0x0800_0000_u32,
            MK_SIGNAL_e::MK_SIGNAL_DOMAIN_BASE_25 => 0x1000_0000_u32,
            MK_SIGNAL_e::MK_SIGNAL_DOMAIN_BASE_26 => 0x2000_0000_u32,
            MK_SIGNAL_e::MK_SIGNAL_DOMAIN_BASE_27 => 0x4000_0000_u32,
            MK_SIGNAL_e::MK_SIGNAL_DOMAIN_BASE_28 => 0x8000_0000_u32,


        }
    }
}

/// Handle to a Kernel Object
pub (crate) type  MK_HANDLE_t = u32;

/// Time unsigned 64-bit integer
pub (crate) type  MK_TIME_t = u64;

#[derive(Copy, Clone, Eq, PartialEq)]
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
impl fmt::Debug for VppState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VppState::READY => {
                write!(f, "Ready\n")
            }
            VppState::RUNNING => {
                write!(f, "Running\n")
            }
            VppState::SUSPENDED_R => {
                write!(f, "S_R\n")
            }
            VppState::WAITING => {
                write!(f, "Waiting\n")
            }
            VppState::SUSPENDED_W  => {
                write!(f, "S_W\n")
            }
            VppState::SYNC => {
                write!(f, "Sync\n")
            }
            VppState::SUSPENDED_S => {
                write!(f, "S_S\n")
            }
            VppState::DEAD => {
                write!(f, "Dead\n")
            }
            VppState::ANY_STATE => {
                write!(f, "ANY_STATE\n")
            }

        }
    }
}

// Scheduling Types (Table 7-7)
pub enum VPP_SCHEDULING_TYPE_e {
    /// Collaborative Scheduling
    MK_SCHEDULING_TYPE_COLLABORATIVE,
    /// Pree-emptive Scheduling
    MK_SCHEDULING_TYPE_PREEMPTIVE,
}
impl From<VPP_SCHEDULING_TYPE_e> for u8 {
    fn from(original: VPP_SCHEDULING_TYPE_e) -> u8 {
        match original {
            VPP_SCHEDULING_TYPE_e::MK_SCHEDULING_TYPE_COLLABORATIVE => 0x01,
            VPP_SCHEDULING_TYPE_e::MK_SCHEDULING_TYPE_PREEMPTIVE =>0x02
        }
    }
}


//Constants and Limits for Any Primary Platfrom (Table 7-9)
/// A grace period, in ticks, given to a VPP Application, so it may
/// shutdown gracefully
pub (crate) const MK_APP_STOP_GRACEFUL_TICKS : u8 = 10;

///Minimal enumerated IPC identifier within the scope of an
/// Execution Domain (including BASE_ID)
pub (crate) const MK_IPC_DOMAIN_BASE_ID : u8 = 100;

/// Length of the IPC identified by MK_IPC_MAIN_COM_ID
/// and MK_IPC_COM_MAIN_ID
// const MK_IPC_COM_LENGTH: u8 =

/// Maximal number of IPC descriptors per Firmware
pub  const MK_IPC_LIMIT : usize = 64 ;

/// Maximal enumerated IPC identifier value within the
/// scope of an Execution Domain (including MAX_ID)
pub (crate) const MK_IPC_MAX_ID : u16 = 0x3FFF;

///Length of the IPC identified by MK_IPC_MAIN_MGT_ID and
/// MK_IPC_MGT_MAIN_ID
// const  MK_IPC_MGT_LENGTH : =6KB;

///Maximal IPC length
// const MK_IPC_SIZE_LIMIT :  =32Kb ;

///Minimal enumerated library identifier within the scope
/// of an Execution Domain (including BASE_ID)
pub(crate) const MK_LIB_DOMAIN_BASE_ID: u8 = 100 ;

///Maximal number of LIB Descriptors in the Firmware
pub(crate) const MK_LIB_LIMIT : u8 = 32 ;

/// Maximal enumerated shared library identifier value
/// within the scope of an Execution Domain (including MAX_ID)
pub(crate) const MK_LIB_MAX_ID : u16 = 0x3FFF ;

/// Minimal enumerated Mailbox identifiers within the
/// scope of an Execution Domain
const MK_MAILBOX_DOMAIN_BASE_ID : u8 = 100 ;

///Maximal number of Mailbox descriptors per Firmware
/// excluding the kernel Mailbox
pub const MK_MAILBOX_LIMIT : usize = 64 ;

/// Maximal enumerated Mailbox identifier value within
/// the scope of a domain (including MAX_ID)
pub (crate) const MK_MAILBOX_MAX_ID : u16 = 0x3FFF ;

/// Minimal number of IPCs accessible concurrently by
/// a Process
pub (crate) const MK_MIN_CONCURRENT_IPC_LIMIT : u8 = 6 ;

///The minimal size in bytes, supported by the Primary Platform,
/// for the sum of all stack memory used by all Processes in a VPP Application
pub (crate) const MK_MIN_STACKS_SUM_SUPPORTED : u16 = 24_000 ; //24Kb

///Minimal number of IPC descriptors in a Firmware
pub (crate) const MK_MIN_APP_IPC : u8 = 0 ;

///Minimal number of Mailbox descriptor of a Firmware,
/// not including kernel Mailboxes
pub (crate) const MK_MIN_APP_MAILBOXES : u8 = 0 ;

/// Minimal number of Process supported by a VPP
/// Application
pub (crate) const MK_MIN_APP_PROCESSS : u8 = 1 ;

/// Minimal size in bytes of Memory Partition in [VFF]
/// supported by the Primary Platform
//const MK_MIN_SUPPORTED_MEMORY_PARTITION_SIZE : u16 = 8_000_000 ; // 8MB

/// Minimum size of the Virtual Memory that the MMF
/// shall manage
pub (crate) const MK_MIN_VIRTUAL_MEMORY_SIZE : u16 = 1_000 ; // 1KB

/// Minimal enumerated Process identifier within the
/// scope of an Execution Domain (including BASE_ID)
pub (crate) const MK_PROCESS_DOMAIN_BASE_ID : u8 = 100 ;

/// Maximal number of Process Descriptors in the Firmware
pub (crate) const MK_PROCESS_LIMIT : u8 = 32 ;

/// Maximal enumerated Process identifier value within
/// the scope of an Execution Domain (included)
pub (crate) const MK_MAX_PROCESS_ID : u16 = 0x3FFF ;

/// Minimal stack size supported for a Process, given in
/// StackType_t units
pub (crate) const MK_MIN_SUPPORTED_STACK : u16 = 512 ; // 512 StackType_t units (2KB if 32bit)

// Cross-Execution Domain Composite Identifiers (Table 7-13)

// Cross-Execution-Domain IPCs and Mailbox descriptors are automatically
// instantiated by the kernel. As such,they cannot be defined in by
// Firmware. Their ID and IPC size are fixed.

/// VPP COM Process Cross-Execution Domain Composite Identifier
pub(crate) const MK_PROCESS_COM_VPP_ID : MK_Process_ID_u = composite_id!(2,0);
/// MGT Process Cross-Execution Domain Composite Identifier
pub(crate) const MK_PROCESS_MGT_VPP_ID : MK_Process_ID_u = composite_id!(2,1);
/// Main Process Cross-Execution Domain Composite Identifier
pub(crate) const MK_PROCESS_MAIN_APP_ID: MK_Process_ID_u = composite_id!(1,0);
/// Com Process Mailbox (sender: Main Process)
pub(crate) const MK_MAILBOX_COM_MAIN_ID: MK_MAILBOX_ID_u = composite_id!(2,0);
/// MGT Process Mailbox (sender: Main Process)
pub(crate) const MK_MAILBOX_MGT_MAIN_ID: MK_MAILBOX_ID_u = composite_id!(2,1);
/// Main Process Mailbox (sender: COM Process)
pub(crate) const MK_MAILBOX_MAIN_COM_ID: MK_MAILBOX_ID_u = composite_id!(1,0);
/// Main Process Mailbox (sender: MGT Process)
pub(crate)  const MK_MAILBOX_MAIN_MGT_ID: MK_MAILBOX_ID_u = composite_id!(1,1);
/// IPC from the Main Process to the COM Process
pub(crate) const MK_IPC_MAIN_COM_ID : MK_IPC_ID_u = composite_id!(1,0);
/// IPC from the COM Process to the Main Process
pub(crate) const MK_IPC_COM_MAIN_ID : MK_IPC_ID_u = composite_id!(2,0);
/// IPC from the Main Process to the MGT Process (for SYSTEM VPP APP only)
pub(crate) const MK_IPC_MAIN_MGT_ID : MK_IPC_ID_u = composite_id!(1,1);
/// IPC from the MGT Process to the Main Process (for SYSTEM VPP APP only)
pub(crate) const MK_IPC_MGT_MAIN_ID : MK_IPC_ID_u = composite_id!(2,1);


// Cross-Execution-Domain Signals (Table 7-14)
// As a Mailbox has only a single reader and writer, Cross-Execution-Domain
// Signals use the same values when different Signals are used by different
// Mailboxes. For example, MK_SIGNAL_KILL_REQUESTED has the same value as
// MK_SIGNAL_KILL_ACCEPTED, but they are being used on different mailboxes.

/// The IPC updated
pub const MK_SIGNAL_IPC_UPDATED: MK_SIGNAL_e = MK_SIGNAL_e::MK_SIGNAL_DOMAIN_BASE_0 ;
/// MGT signaled the Main Process to terminate itself
pub const MK_SIGNAL_KILL_REQUESTED: MK_SIGNAL_e = MK_SIGNAL_e::MK_SIGNAL_DOMAIN_BASE_1 ;
/// The Main Process signaled MGT that it has accepted the kill request
pub const MK_SIGNAL_KILL_ACCEPTED : MK_SIGNAL_e = MK_SIGNAL_e::MK_SIGNAL_DOMAIN_BASE_1 ;
/// The Main Process signaled MGT to restart the VPP Application
pub  const MK_SIGNAL_APP_RESTART :  MK_SIGNAL_e = MK_SIGNAL_e::MK_SIGNAL_DOMAIN_BASE_2 ;
/// The Main Process committed suicide
pub const MK_SIGNAL_KILL_ITSELF: MK_SIGNAL_e = MK_SIGNAL_e::MK_SIGNAL_DOMAIN_BASE_3 ;
