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


/// Index of an element in a typed array
type MK_Index_t = u16 ;

/// Execution Domain Types (Table 7-3)
#[repr(u16)]
pub enum MK_EXECUTION_DOMAIN_TYPE_e {
    /// System VPP Execution Domain
    MK_EXECUTION_DOMAIN_TYPE_VPP ,
    /// VPP Application Execution Domain
    MK_EXECUTION_DOMAIN_TYPE_APP  ,
}
impl MK_EXECUTION_DOMAIN_TYPE_e {
    ///  The `value` function converts the enum `MK_EXECUTION_DOMAIN_TYPE_e` to
    /// the associated values
    pub  fn value(&self) -> u16 {
        match *self {
            MK_EXECUTION_DOMAIN_TYPE_e::MK_EXECUTION_DOMAIN_TYPE_VPP => 0b10_u16,
            MK_EXECUTION_DOMAIN_TYPE_e::MK_EXECUTION_DOMAIN_TYPE_APP => 0b01_u16,

        }
    }
    /// The `convert` function does the opposite of what the `value` function does.
    /// It converts the values associated with the enum `MK_EXECUTION_DOMAIN_TYPE_e`
    /// to the domain type (App or Vpp)
    pub  fn convert(value: u16) -> Option<MK_EXECUTION_DOMAIN_TYPE_e> {
        match value {
            0b10_u16 => Some(MK_EXECUTION_DOMAIN_TYPE_e::MK_EXECUTION_DOMAIN_TYPE_VPP),
            0b01_u16 => Some(MK_EXECUTION_DOMAIN_TYPE_e::MK_EXECUTION_DOMAIN_TYPE_APP),
            _ => None
        }

    }
}



// trait Kernel_Object_Type <T>{
//     fn new(id_e: T, exec_domain: MK_EXECUTION_DOMAIN_TYPE_e) -> Box<Self> {
//         id_e & (MK_EXECUTION_DOMAIN_TYPE_e::value(&exec_domain)<<14)
//     }
//     fn get_id (&self) -> T {
//         0b0011_1111_1111_1111_u16 & self
//     }
//     fn get_exdomain (&self) -> MK_EXECUTION_DOMAIN_TYPE_e {
//         MK_EXECUTION_DOMAIN_TYPE_e::convert(self >>14).unwrap()
//     }
//     fn set_id (&self ,id_e: T) -> Box<Self> {
//         (  self & 0xC000_u16)  | id_e
//     }
//     fn set_exdomain (&self , exec_domain: MK_EXECUTION_DOMAIN_TYPE_e) -> Box<Self> {
//         (self & 0x3FFF_u16 )| MK_EXECUTION_DOMAIN_TYPE_e::value(&exec_domain)
//     }
// }

/// Composite Identifier of an IPC (Table 7-2)
pub type MK_IPC_ID_u = u16;
pub type MK_IPC_ID_e = u16;

//impl Kernel_Object_Type<MK_IPC_ID_e> for MK_IPC_ID_u {}

/// Composite Identifier of a Mailbox (Table 7-2)
pub type MK_MAILBOX_ID_u = u16 ;
pub type MK_MAILBOX_ID_e = u16 ;
//impl Kernel_Object_Type<MK_MAILBOX_ID_e> for MK_MAILBOX_ID_u {}

/// Composite Identifier of a Process (Table 7-2)
pub type MK_Process_ID_u= u16 ;
pub type MK_Process_ID_e= u16 ;
//impl Kernel_Object_Type<MK_PROCESS_ID_e> for MK_PROCESS_ID_u {}

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
                write!(f, "Priority Low\n")
            }
            MK_PROCESS_PRIORITY_e::MK_PROCESS_PRIORITY_NORMAL => {
                write!(f, "Priority Normal\n")
            }
            MK_PROCESS_PRIORITY_e::MK_PROCESS_PRIORITY_HIGH => {
                write!(f, "Priority High\n")
            }
            MK_PROCESS_PRIORITY_e::MK_PROCESS_PRIORITY_ERROR => {
                write!(f, "Error Priority\n")
            }


        }
    }
}

/// Composite Identifier of a shared library (Table 7-2)
pub type MK_LIB_ID_u = u16 ;
pub type MK_LIB_ID_e = u16 ;
//impl Kernel_Object_Type<MK_LIB_ID_e> for MK_LIB_ID_u {}

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
//PROCESS_Function_t

/// Memory Address to a LLOS entry point
// PLLOS_Function_t

/// Stack element
// StackType_t

/// Unique Universal Identifier
type UUID_t = u128;

/// void 32-bit
type v32_u = u32 ;

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

impl fmt::Debug for MK_ERROR_e {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MK_ERROR_e::MK_ERROR_NONE => {
                write!(f, "No error\n")
            }
            MK_ERROR_e::MK_ERROR_UNKNOWN_ID => {
                write!(f, "Unknown ID\n")
            }
            _ => write!(f, "FUCK\n")
            /*

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
            */
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
pub type  MK_HANDLE_t = u32;

pub(crate) fn convert_to_handle(id: MK_Process_ID_u) -> MK_HANDLE_t{
    id as u32
}
pub(crate) fn convert_to_id(handle: MK_HANDLE_t) -> MK_Process_ID_u{
    handle as u16
}


/// Time unsigned 64-bit integer
type MK_TIME_t = u64;

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
                write!(f, "Suspended Ready\n")
            }
            VppState::WAITING => {
                write!(f, "Waiting\n")
            }
            VppState::SUSPENDED_W  => {
                write!(f, "Suspended Waiting\n")
            }
            VppState::SYNC => {
                write!(f, "Sync\n")
            }
            VppState::SUSPENDED_S => {
                write!(f, "Suspended Sync\n")
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
/// Collaborative Scheduling
pub const MK_SCHEDULING_TYPE_COLLABORATIVE: u8 = 0x01;
/// Pree-emptive Scheduling
pub const MK_SCHEDULING_TYPE_PREEMPTIVE: u8 = 0x02;


//Constants and Limits for Any Primary Platfrom (Table 7-9)
/// A grace period, in ticks, given to a VPP Application, so it may
/// shutdown gracefully
const MK_APP_STOP_GRACEFUL_TICKS : u8 = 10;

///Minimal enumerated IPC identifier within the scope of an
/// Execution Domain (including BASE_ID)
const MK_IPC_DOMAIN_BASE_ID : u8 = 100;

/// Length of the IPC identified by MK_IPC_MAIN_COM_ID
/// and MK_IPC_COM_MAIN_ID
// const MK_IPC_COM_LENGTH: u8 =

/// Maximal number of IPC descriptors per Firmware
const MK_IPC_LIMIT : u8 = 64 ;

/// Maximal enumerated IPC identifier value within the
/// scope of an Execution Domain (including MAX_ID)
const MK_IPC_MAX_ID : u16 = 0x3FFF;

///Length of the IPC identified by MK_IPC_MAIN_MGT_ID and
/// MK_IPC_MGT_MAIN_ID
// const  MK_IPC_MGT_LENGTH : =6KB;

///Maximal IPC length
// const MK_IPC_SIZE_LIMIT :  =32Kb ;

///Minimal enumerated library identifier within the scope
/// of an Execution Domain (including BASE_ID)
const MK_LIB_DOMAIN_BASE_ID: u8 = 100 ;

///Maximal number of LIB Descriptors in the Firmware
const MK_LIB_LIMIT : u8 = 32 ;

/// Maximal enumerated shared library identifier value
/// within the scope of an Execution Domain (including MAX_ID)
const MK_LIB_MAX_ID : u16 = 0x3FFF ;

/// Minimal enumerated Mailbox identifiers within the
/// scope of an Execution Domain
const MK_MAILBOX_DOMAIN_BASE_ID : u8 = 100 ;

///Maximal number of Mailbox descriptors per Firmware
/// excluding the kernel Mailbox
const MK_MAILBOX_LIMIT : u8 = 64 ;

/// Maximal enumerated Mailbox identifier value within
/// the scope of a domain (including MAX_ID)
const MK_MAILBOX_MAX_ID : u16 = 0x3FFF ;

/// Minimal number of IPCs accessible concurrently by
/// a Process
const MK_MIN_CONCURRENT_IPC_LIMIT : u8 = 6 ;

///The minimal size in bytes, supported by the Primary Platform,
/// for the sum of all stack memory used by all Processes in a VPP Application
const MK_MIN_STACKS_SUM_SUPPORTED : u16 = 24_000 ; //24Kb

///Minimal number of IPC descriptors in a Firmware
const MK_MIN_APP_IPC : u8 = 0 ;

///Minimal number of Mailbox descriptor of a Firmware,
/// not including kernel Mailboxes
const MK_MIN_APP_MAILBOXES : u8 = 0 ;

/// Minimal number of Process supported by a VPP
/// Application
const MK_MIN_APP_PROCESSS : u8 = 1 ;

/// Minimal size in bytes of Memory Partition in [VFF]
/// supported by the Primary Platform
//const MK_MIN_SUPPORTED_MEMORY_PARTITION_SIZE : u16 = 8_000_000 ; // 8MB

/// Minimum size of the Virtual Memory that the MMF
/// shall manage
const MK_MIN_VIRTUAL_MEMORY_SIZE : u16 = 1_000 ; // 1KB

/// Minimal enumerated Process identifier within the
/// scope of an Execution Domain (including BASE_ID)
const MK_PROCESS_DOMAIN_BASE_ID : u8 = 100 ;

/// Maximal number of Process Descriptors in the Firmware
const MK_PROCESS_LIMIT : u8 = 32 ;

/// Maximal enumerated Process identifier value within
/// the scope of an Execution Domain (included)
const MK_MAX_PROCESS_ID : u16 = 0x3FFF ;

/// Minimal stack size supported for a Process, given in
/// StackType_t units
const MK_MIN_SUPPORTED_STACK : u16 = 512 ; // 512 StackType_t units (2KB if 32bit)



// Cross-Execution Domain Composite Identifiers (Table 7-13)

// Cross-Execution-Domain IPCs and Mailbox descriptors are automatically
// instantiated by the kernel. As such,they cannot be defined in by
// Firmware. Their ID and IPC size are fixed.
//
// /// VPP COM Process Cross-Execution Domain Composite Identifier
// const MK_PROCESS_COM_VPP_ID : dyn Kernel_Object_Type<MK_PROCESS_ID_e>
// = Kernel_Object_Type::new(0, MK_EXECUTION_DOMAIN_TYPE_e::MK_EXECUTION_DOMAIN_TYPE_VPP);
//
// /// MGT Process Cross-Execution Domain Composite Identifier
// const MK_PROCESS_MGT_VPP_ID : dyn Kernel_Object_Type<MK_PROCESS_ID_e>
// = Kernel_Object_Type::new(1, MK_EXECUTION_DOMAIN_TYPE_e::MK_EXECUTION_DOMAIN_TYPE_VPP) ;
//
// /// Main Process Cross-Execution Domain Composite Identifier
// const MK_PROCESS_APP_ID :dyn  Kernel_Object_Type<MK_PROCESS_ID_e> =
// Kernel_Object_Type::new(0, MK_EXECUTION_DOMAIN_TYPE_e::MK_EXECUTION_DOMAIN_TYPE_APP) ;
//
// /// Com Process Mailbox (sender: Main Process)
// const MK_MAILBOX_COM_MAIN_ID : dyn Kernel_Object_Type<MK_MAILBOX_ID_e> =
// Kernel_Object_Type::new(0, MK_EXECUTION_DOMAIN_TYPE_e::MK_EXECUTION_DOMAIN_TYPE_VPP) ;
//
// /// MGT Process Mailbox (sender: Main Process)
// const MK_MAILBOX_MGT_MAIN_ID : dyn Kernel_Object_Type<MK_MAILBOX_ID_e> =
// Kernel_Object_Type::new(1, MK_EXECUTION_DOMAIN_TYPE_e::MK_EXECUTION_DOMAIN_TYPE_VPP);
//
// /// Main Process Mailbox (sender: COM Process)
// const MK_MAILBOX_MAIN_COM_ID : dyn Kernel_Object_Type<MK_MAILBOX_ID_e> =
// Kernel_Object_Type::new(0, MK_EXECUTION_DOMAIN_TYPE_e::MK_EXECUTION_DOMAIN_TYPE_APP);
//
// /// Main Process Mailbox (sender: COM Process)
// const MK_MAILBOX_MAIN_MGT_ID : dyn Kernel_Object_Type<MK_MAILBOX_ID_e> =
// Kernel_Object_Type::new(1, MK_EXECUTION_DOMAIN_TYPE_e::MK_EXECUTION_DOMAIN_TYPE_APP);
//
// /// IPC from the Main Process to the COM Process
// const MK_IPC_MAIN_COM_ID : dyn Kernel_Object_Type<MK_IPC_ID_e> =
// Kernel_Object_Type::new(0,MK_EXECUTION_DOMAIN_TYPE_e::MK_EXECUTION_DOMAIN_TYPE_APP);
//
// /// IPC from the COM Process to the Main Process
// const MK_IPC_COM_MAIN_ID : dyn Kernel_Object_Type<MK_IPC_ID_e> =
// Kernel_Object_Type::new(0,MK_EXECUTION_DOMAIN_TYPE_e::MK_EXECUTION_DOMAIN_TYPE_VPP);
//
// /// IPC from the Main Process to the MGT Process
// /// (for SYSTEM VPP APP)
// const MK_IPC_MAIN_MGT_ID : dyn Kernel_Object_Type<MK_IPC_ID_e> =
// Kernel_Object_Type::new(1,MK_EXECUTION_DOMAIN_TYPE_e::MK_EXECUTION_DOMAIN_TYPE_APP);
//
// /// IPC from the MGT Process to the Main Process
// ///(for SYSTEM VPP APP)
// const MK_IPC_MGT_MAIN_ID : dyn Kernel_Object_Type<MK_IPC_ID_e> =
// Kernel_Object_Type::new(1,MK_EXECUTION_DOMAIN_TYPE_e::MK_EXECUTION_DOMAIN_TYPE_VPP);

// Cross-Execution-Domain Signals (Table 7-14)
// As a Mailbox has only a single reader and writer, Cross-Execution-Domain
// Signals use the same values when different Signals are used by different
// Mailboxes. For example, MK_SIGNAL_KILL_REQUESTED has the same value as
// MK_SIGNAL_KILL_ACCEPTED, but they are being used on different mailboxes.

/// The IPC updated
const MK_SIGNAL_IPC_UPDATED: MK_SIGNAL_e = MK_SIGNAL_e::MK_SIGNAL_DOMAIN_BASE_0 ;
/// MGT signaled the Main Process to terminate itself
const MK_SIGNAL_KILL_REQUESTED: MK_SIGNAL_e = MK_SIGNAL_e::MK_SIGNAL_DOMAIN_BASE_1 ;
/// The Main Process signaled MGT that it has accepted the kill request
const MK_SIGNAL_KILL_ACCEPTED : MK_SIGNAL_e = MK_SIGNAL_e::MK_SIGNAL_DOMAIN_BASE_1 ;
/// The Main Process signaled MGT to restart the VPP Application
const MK_SIGNAL_APP_RESTART :  MK_SIGNAL_e = MK_SIGNAL_e::MK_SIGNAL_DOMAIN_BASE_2 ;
/// The Main Process committed suicide
const MK_SIGNAL_KILL_ITSELF: MK_SIGNAL_e = MK_SIGNAL_e::MK_SIGNAL_DOMAIN_BASE_3 ;
