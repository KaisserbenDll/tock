#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
/// This contains all data types and constants described in the VPP -
/// Firmware Format standard. It also implements the kernel ABI/API specific
/// to Firmware Management Service.

use crate::vpp::mloi::*;
use core::cell::Cell;

const MK_FORMAT_VERSION : u8 = 0b0100 ;

pub struct FW {
    pub FwHeader : FwHeader,
    pub FwBody : FwBody,
    pub state: Cell<FwState>,
}
/// Firmware State.
pub enum FwState {
    Enabled,
    Disabled,
}
impl From<FwState> for u8 {
    fn from(original: FwState) -> u8 {
        match original {
            FwState::Enabled => 0x01,
            FwState::Disabled=> 0x00,
        }
    }
}
pub struct FwHeader {
    pub fw_descriptor: FwDescriptor,
    pub process_descriptors: ProcessDescriptors,
    pub mailbox_descriptors: MailboxDescriptors,
    pub ipc_descriptor: IpcDescriptor,
}

impl FwHeader{}
pub struct FwDescriptor {
    /// Firmware Identifier
    pub m_xID: UUID_t,
    /// Fimrware Family Identifier
    pub  m_xFamilyID: UUID_t,
    /// Length of the Firmware header
    pub  m_uHeaderLength: u16,
    /// Major and minor version of the Firmware Format
    pub m_VersionFormat: u16,
    /// Major and minor version of the firmware format
    pub  m_VersionFirmware: u16,
    /// Number of Process Descriptors
    pub  m_uProcessCount: u8 ,
    /// Number of mailbox Descriptors
    pub m_uMailboxesCount: u8,
    /// Number of IPC Descriptors
    pub m_uIPCCount: u8,
    /// Number of shared LIBrary Descriptors
    pub m_uLIBCount: u8 ,
    /// Type of the Fimrware
    pub m_eFirmare_Software_Type: VPP_FRW_TYPE_e,
    /// Type of scheduling
    pub m_eSchedluingType: VPP_SCHEDULING_TYPE_e,
}
/// Definition of the Structure of the IPC Descriptor. The IPC array shall be sorted
/// in ascending order of the IPC identifiers (m_xID). The IPC Descriptor alignment is
/// 16 bits. The total size of the IPC Descriptor is 8 bytes == 64 bits.
pub struct IpcDescriptor {
    /// Identifiers of the IPC.
    pub m_xID : MK_IPC_ID_u,
    /// Length of the shared memory (in byte).
    pub m_uLength_IPC : u16,
    /// Index of the writer Process in the group of processes.
    pub m_uIX_Writer : MK_Index_t,
    /// Index of the reader Process in the group of processes.
    pub  m_uIX_Reader : MK_Index_t,
}
/// Definition of the structure of the Mailbox Descriptor. The mailboxes array shall
/// be sorted in ascending order of mailbox identifiers (m_xID). The Mailbox Descriptor
/// alignment is 16 bits and there is no padding for the fields smaller than 16 bits.
/// The total size of the Mailbox Descriptor is 6 bytes == 48 bits.
pub struct MailboxDescriptors {
    /// Identifiers of the mailbox
    pub m_xID : MK_MAILBOX_ID_u,
    /// Index of the owner/reciever Process in the group of processes
    pub m_uIX_Owner : MK_Index_t,
    /// Index of the sender Process in the group of processes
    pub m_uIX_Sender : MK_Index_t,
}


/// Definition of the structure and content of the Process Descriptor.
/// The Process array shall be sorted in ascending order of Process identifiers (m_xID)
/// The first entry shall be considered as _Main Process_. The Process Descriptor alignment
/// is 32 bit. The total size of the Process Descriptor is 32 bytes == 256 bits.
pub struct ProcessDescriptors {
    /// Offset within the CODE segment to compute a pointer to
    /// the Process Entry point function
    pub m_pvProcessCode: PROCESS_Function_t,
    /// Length of the whole segment for CODE
    pub m_uLength_Process_CODE : v32_u,
    /// Length of the whole segment for CONSTANT data
    pub m_uLength_Process_CONSTANTS: v32_u,
    /// Length of the whole segment for DATA
    pub m_uLength_Process_DATA: v32_u,
    /// Length of the whole segment for NVM
    pub m_uLength_Process_NVM : v32_u,
    // /// ORing of Mandatory Access Control for VRE
    // m_eVRE: MK_VRE_e,
    /// Process Identifier
    pub m_xID : MK_Process_ID_u,
    // /// Identifier of the mailbox receiving signals from the kernel
    // m_xKernel_Mailbox : MK_MAILBOX_ID_u,
    ///Index of the parent Process in the group of processes (self-reference for
    /// MK_PROCESS_MAIN_APP_ID Process)
    pub m_uParent_Process : MK_Index_t,
    /// Size of the Stack (in bytes and multiple of 4)
    pub m_uSizeStack : u16,
}
pub struct FwBody{
    /// binary executable code
    CODE:  &'static [u8],
    /// Constants
    RO: &'static [u8],
    /// initialized data
    DATA: &'static[u8],
    /// NVM data
    NVM:&'static [u8],
}
impl Default for FwBody {
    fn default() -> Self {
        FwBody{
            CODE: Default::default(),
            RO: Default::default(),
            DATA: Default::default(),
            NVM: Default::default()
        }
    }
}

pub struct _mk_Fw_Mgt{
    Fw: FW,
}
impl _mk_Fw_Mgt {
    pub fn new (fw: FW) -> _mk_Fw_Mgt {
        _mk_Fw_Mgt{
            Fw: fw,
        }
    }
    /// Inform the Kernel that the impersonation of a Firmware started.
    /// This function can only be used by the System VPP application.
    ///
    /// _mk_Open_Impersonation allows reading and writing in the Memory Partition
    /// of the registered Firmware.
    pub fn _mk_Open_Impersonation(&self,_uFirmwareID: UUID_t) -> MK_ERROR_e {
       unimplemented!();
    }

    /// Inform the kernel that the impersonation of a Firmware has completed.
    /// This function can only be used by the System VPP application
    ///
    /// Clear UUID Kernel Object
    pub fn _mk_Close_Impersonation(&self) ->MK_ERROR_e {
        unimplemented!();
    }

}
pub enum MGT_ERROR_e{
    MGT_ERROR_NONE,
    MGT_ERROR_ILLEGAL_PARAMETER,
    MGT_ERROR_INTERNAL,
    MGT_ERROR_UNKNOWN_UUID,
    MGT_ERROR_COMMAND_NOK,
}

pub fn parse_vpp_header(firmware: &'static [u8]) -> FwHeader{//-> Result<FwHeader,MGT_ERROR_e>{
    // let id_bytes = (0..15).map(|byte|firmware[byte]).collect::<[u8;16]>();
    // let id_bytes = (0..15).map(|byte|firmware[byte]).collect::<Vec<_>>();
    //
    // // let id_byt = firmware.get(0..15)?;
    // let firmware_id = u128::from_le_bytes(id_bytes) as UUID_t;
    // // use from_ne_bytes ?
    // let family_id_bytes = (16..31).map(|byte|firmware[byte]).collect::<[u8;16]>();
    // let firmware_family_id = u128::from_le_bytes(family_id_bytes) as UUID_t;


    let header_length = u16::from_le_bytes([firmware[32],firmware[33]]) ;
    let version_format = u16::from_le_bytes([firmware[34],firmware[35]]);
    let version_firmware = u16::from_le_bytes([firmware[36],firmware[37]]);

    let process_count = u8::from_le_bytes([firmware[38]]);
    let mailbox_count = u8::from_le_bytes([firmware[39]]);
    let ipc_count = u8::from_le_bytes([firmware[40]]);
    let lib_count = u8::from_le_bytes([firmware[41]]);

    // let fw_sw_type = firmware.get(42).ok_or(MGT_ERROR_ILLEGAL_PARAMETER)?;
    // let fw_sw_type : VPP_FRW_TYPE_e = firmware.get(42).from();
    // let scheduling :VPP_SCHEDULING_TYPE_e = firmware.get(43).into();
    let firmware_header = FwHeader {
        fw_descriptor: FwDescriptor {
            m_xID: 0,
            m_xFamilyID: 0,
            m_uHeaderLength: header_length,
            m_VersionFormat: version_format,
            m_VersionFirmware: version_firmware,
            m_uProcessCount: process_count,
            m_uMailboxesCount: mailbox_count,
            m_uIPCCount: ipc_count,
            m_uLIBCount: lib_count,
            m_eFirmare_Software_Type: VPP_FRW_TYPE_e::FIRMWARE_SOFTWARE_TYPE_APP,
            m_eSchedluingType: VPP_SCHEDULING_TYPE_e::MK_SCHEDULING_TYPE_COLLABORATIVE
        },
        process_descriptors: ProcessDescriptors {
            m_pvProcessCode: 0,
            m_uLength_Process_CODE: 0,
            m_uLength_Process_CONSTANTS: 0,
            m_uLength_Process_DATA: 0,
            m_uLength_Process_NVM: 0,
            m_xID: 0,
            m_uParent_Process: 0,
            m_uSizeStack: 0
        },
        mailbox_descriptors: MailboxDescriptors {
            m_xID: 0,
            m_uIX_Owner: 0,
            m_uIX_Sender: 0
        },
        ipc_descriptor: IpcDescriptor {
            m_xID: 0,
            m_uLength_IPC: 0,
            m_uIX_Writer: 0,
            m_uIX_Reader: 0
        }
    };
    firmware_header


}

/// Consistency Rules:
/// The rules below can be used to evaluate the correctness and validity of the Firmware
/// Format. To assume correctness, all rules shall be evaluated.

pub fn firmware_descriptor_rules(firmware : FW) -> bool {
    let fw_descriptor : FwDescriptor = firmware.FwHeader.fw_descriptor;
    // 1. m_xID and m_xFamilyID should be according to [UUID] of type 4
    // ??

    // 2. m_uHeaderLength shall be equal to:
    // * size of Firmware Descriptor + (m_uProcessCount x size of Firmware Descriptor) +
    // (m_uMailboxesCount x size of Mailbox Descriptor) + (m_uIPCCount x size of IPC Descriptor) +
    // (m_uLIBCount x size of LIB Descriptor), or
    // * size of Firmware Descriptor + size of LLOS Descriptor.
    let _expected_length = fw_descriptor.m_uHeaderLength;
    // double check what the size_of function does.
    //let _size_fw_descriptor = mem::size_of::<fw_descriptor>() as u16 ;
   // let size_processes = fw_descriptor.
   // let calculated_length = ;

    false
}



/*
            let (process_option, unused_memory) = unsafe {
                Process::create(
                    kernel,
                    chip,
                    entry_flash,
                    header_length as usize,
                    version,
                    remaining_memory,
                    fault_response,
                    i,
                )?
            };*/