#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::vpp::mloi::*;
// use crate::vpp::vff;
use crate::vpp::vff::{FW, FwHeader, parse_vpp_header, FwState, FwBody};
use crate::vpp::mgt_service::ServiceCommands::MGT_Store_Firmware_Header;
use crate::vpp::mgt_service::ResponseCode::MGT_ERROR_NONE;
use core::cell::Cell;


#[derive(Clone, Copy,PartialEq)]
pub enum ServiceCommands{
    MGT_Store_Firmware_Header,
    MGT_Retrieve_Firmware_Header,
    MGT_Allocate_Firmware,
    MGT_Delete_Firmware,
    MGT_Enable_Firmware,
    MGT_Disable_Firmware,
    MGT_Is_Firmware_Enabled,
    MGT_Open_Process_Impersonation,
    MGT_Close_Process_Impersonation,
    MGT_Open_Library_Impersonation,
    MGT_Close_Library_Impersonation,
    MGT_Open_LLOS_Impersonation,
    MGT_Close_LLOS_Impersonation,
}
impl From<ServiceCommands> for u8 {
    fn from(original: ServiceCommands) -> u8 {
        match original {
            ServiceCommands::MGT_Store_Firmware_Header => 0x00,
            ServiceCommands::MGT_Retrieve_Firmware_Header => 0x01,
            ServiceCommands::MGT_Allocate_Firmware => 0x02 ,
            ServiceCommands::MGT_Delete_Firmware => 0x03,
            ServiceCommands::MGT_Enable_Firmware => 0x04,
            ServiceCommands::MGT_Disable_Firmware => 0x05,
            ServiceCommands::MGT_Is_Firmware_Enabled => 0x06,
            ServiceCommands::MGT_Open_Process_Impersonation => 0x07,
            ServiceCommands::MGT_Close_Process_Impersonation => 0x08,
            ServiceCommands::MGT_Open_Library_Impersonation => 0x09,
            ServiceCommands::MGT_Close_Library_Impersonation => 0x0A,
            ServiceCommands::MGT_Open_LLOS_Impersonation => 0x0B,
            ServiceCommands::MGT_Close_LLOS_Impersonation => 0x0C,
        }
    }
}
#[derive(Clone, Copy)]
pub enum ResponseCode {
    MGT_ERROR_NONE,
    MGT_ERROR_ILLEGAL_PARAMETER,
    MGT_ERROR_INTERNAL,
    MGT_ERROR_UNKNOWN_UUID,
    MGT_ERROR_COMMAND_NOK,
}
impl From<ResponseCode> for u8 {
    fn from (original: ResponseCode) -> u8 {
        match original {
            ResponseCode::MGT_ERROR_NONE => 0x00,
            ResponseCode::MGT_ERROR_ILLEGAL_PARAMETER => 0x01 ,
            ResponseCode::MGT_ERROR_INTERNAL => 0x02,
            ResponseCode::MGT_ERROR_UNKNOWN_UUID => 0x03,
            ResponseCode::MGT_ERROR_COMMAND_NOK => 0x04,
        }
    }
}

pub struct mgt_process {
    firmware: FW
}
impl mgt_process {
    pub fn new(firmware: FW) -> mgt_process {
        mgt_process{
         firmware: firmware,
        }
    }
    // Firmware Header Management
    pub fn MGT_Store_Firmware_Header(
        firmware_header_data: &'static [u8],
        command: ServiceCommands )
        -> ResponseCode {
        if command == MGT_Store_Firmware_Header {
            //prase the firmware
            // add Error handling for parse_vpp_header
            let fw_header = parse_vpp_header(firmware_header_data);
            // compare the firmware_header to the Primary Platform capabilities
            // ?
            // if Firmware_header not supported => illegal_parameter ?
            // Extract the UUID from firmware_header
            let _firmware_uuid = fw_header.fw_descriptor.m_xID ;
            // Retrieve the firmware_header from the MGT Process NVM, based on its provided fw
            // ?
            // if Firmware_header cannot be found => add a new fw_hd, store the provided fw_id
            // asthe key to this record. State is disabled
            // missing ???
            let _Firmware  = FW {
                FwHeader : fw_header,
                FwBody : FwBody::default(),
                state: Cell::new(FwState::Disabled),
            };

        }
    MGT_ERROR_NONE
    }

    pub fn MGT_Retrieve_Firmware_Header(
        _firmware_identifier: UUID_t,
        _command: ServiceCommands)
    -> (ResponseCode,FwHeader) {
        unimplemented!();
    }
    // Firmware State Management
    pub fn MGT_Enable_Firmware(
        _firmware_identifier: UUID_t,
        _command: ServiceCommands)
        -> ResponseCode{
        unimplemented!();
    }
    pub fn MGT_Disable_Firmware(
        _firmware_identifier: UUID_t,
        _command: ServiceCommands)
        -> ResponseCode {
        unimplemented!();
    }
    pub fn MGT_Is_Firmware_Enabled(
        _firmware_identifier: UUID_t,
        _command: ServiceCommands)
        -> ResponseCode {
        unimplemented!();
    }
    pub fn MGT_Delete_Firmware(
        _firmware_identifier: UUID_t,
        _command: ServiceCommands)
        -> ResponseCode {
        unimplemented!();
    }
    // Firmware Impersonation Management
    pub fn MGT_Open_Process_Impersonation(
        _firmware_identifier: UUID_t,
        _command: ServiceCommands,
        _index: MK_Index_t)
        -> ResponseCode {
        unimplemented!();
    }
    pub fn MGT_Close_Process_Impersonation(
        _command: ServiceCommands)
    -> ResponseCode {
        unimplemented!();
    }
    pub fn MGT_Allocate_Firmware(
        _firmware_identifier: UUID_t,
        _command: ServiceCommands)
        -> ResponseCode {
        unimplemented!();
    }


}