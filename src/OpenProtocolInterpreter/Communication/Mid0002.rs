use std::collections::HashMap;

use crate::OpenProtocolInterpreter::DataField::DataFieldT;
/// <summary>
    /// Application Communication start acknowledge
    /// <para>
    ///     When accepting the communication start the controller sends as reply, 
    ///     a Communication start acknowledge. This message contains some basic information about the
    ///     controller, such as cell ID, channel ID, and name.
    /// </para>
    /// <para>Message sent by: Controller</para>
    /// <para>Answer: None</para>
    /// </summary>
use crate::OpenProtocolInterpreter::Header::{self, HeaderT};
use crate::OpenProtocolInterpreter::MID::MidT;
use crate::OpenProtocolInterpreter::OpenProtocolConvert::OpenProtocolConvertT;
use crate::OpenProtocolInterpreter::Enums;

 enum DataFields
{
    //Rev 1
    CellId,
    ChannelId,
    ControllerName,
    //Rev 2
    SupplierCode,
    //Rev 3
    OpenProtocolVersion,
    ControllerSoftwareVersion,
    ToolSoftwareVersion,
    //Rev 4
    RBUType,
    ControllerSerialNumber,
    //Rev 5
    SystemType,
    SystemSubtype,
    //Rev 6
    SequenceNumberSupport,
    LinkingHandlingSupport,
    StationCellId,
    StationCellName,
    ClientId,
    //Rev 7
    OptionalKeepAlive
}   

#[derive(Clone)]
pub struct Mid0002T {
    pub mid:MidT,
}

impl Mid0002T {
    pub const MID:i32 = 2;

    pub fn cell_id(&mut self) ->i32 {
        self.mid.get_field(1, DataFields::CellId as i32).get_value(OpenProtocolConvertT::string_to_int32)
    }

    pub fn set_cell_id(&mut self, value:i32) {
        self.mid.get_field(1, DataFields::CellId as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value);
    }

    pub fn channel_id(&mut self) ->i32 {
        self.mid.get_field(1, DataFields::ChannelId as i32).get_value(OpenProtocolConvertT::string_to_int32)
    }

    pub fn set_channel_id(&mut self, value:i32) {
        self.mid.get_field(1, DataFields::ChannelId as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value);
    }

    pub fn controller_name(&mut self) ->String {
        self.mid.get_field(1, DataFields::ControllerName as i32).value
    }

    pub fn set_controller_name(&mut self, value:String) {
        self.mid.get_field(1, DataFields::ControllerName as i32).set_value(value);
    }

    //Rev 2
    pub fn supplier_code(&mut self) ->String {
        self.mid.get_field(2, DataFields::SupplierCode as i32).value
    }

    pub fn set_supplier_code(&mut self, value:String) {
        self.mid.get_field(2, DataFields::SupplierCode as i32).set_value(value);
    }

    //Rev 3
    pub fn open_protocol_version(&mut self) ->String {
        self.mid.get_field(3, DataFields::OpenProtocolVersion as i32).value
    }

    pub fn set_open_protocol_version(&mut self, value:String) {
        self.mid.get_field(3, DataFields::OpenProtocolVersion as i32).set_value(value);
    }

    pub fn controller_software_version(&mut self) ->String {
        self.mid.get_field(3, DataFields::ControllerSoftwareVersion as i32).value
    }

    pub fn set_controller_software_version(&mut self, value:String) {
        self.mid.get_field(3, DataFields::ControllerSoftwareVersion as i32).set_value(value);
    }

    pub fn tool_software_version(&mut self) ->String {
        self.mid.get_field(3, DataFields::ToolSoftwareVersion as i32).value
    }

    pub fn set_tool_software_version(&mut self, value:String) {
        self.mid.get_field(3, DataFields::ToolSoftwareVersion as i32).set_value(value);
    }

    //Rev 4
    pub fn rbu_type(&mut self) ->String {
        self.mid.get_field(4, DataFields::RBUType as i32).value
    }

    pub fn set_rbu_type(&mut self, value:String) {
        self.mid.get_field(4, DataFields::RBUType as i32).set_value(value);
    }

    pub fn controller_serial_number(&mut self) ->String {
        self.mid.get_field(4, DataFields::ControllerSerialNumber as i32).value
    }

    pub fn set_controller_serial_number(&mut self, value:String) {
        self.mid.get_field(4, DataFields::ControllerSerialNumber as i32).set_value(value);
    }

    //Rev 5 
    pub fn system_type(&mut self) ->Enums::SystemType {
        let v = self.mid.get_field(5, DataFields::SystemType as i32).get_value(OpenProtocolConvertT::string_to_int32);
    
        let st:Enums::SystemType = unsafe { ::std::mem::transmute(v) };

        st
    }

    pub fn set_system_type(&mut self, value:Enums::SystemType) {
        self.mid.get_field(5, DataFields::SystemType as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value as i32);
    }

    /// <summary>
    /// <para>If no subtype exists it will be set to 000</para>
    /// <para>For a Power Focus 4000 and PF 6000 system the valid subtypes are:</para>
    /// <para>001 = a normal tightening system</para>
    /// <para>For a Power MACS 4000 system the valid subtypes are:</para>
    /// <para>001 = a normal tightening system </para>
    /// <para>002 = a system running presses instead of spindles.</para>
    /// </summary>
    pub fn system_sub_type(&mut self) ->Enums::SystemSubType {
        let v = self.mid.get_field(5, DataFields::SystemSubtype as i32).get_value(OpenProtocolConvertT::string_to_int32);
    
        let sst:Enums::SystemSubType = unsafe { ::std::mem::transmute(v) };

        sst
    }

    pub fn set_system_sub_type(&mut self, value:Enums::SystemSubType) {
        self.mid.get_field(5, DataFields::SystemSubtype as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value as i32);
    }

    //Rev 6
    pub fn sequence_number_support(&mut self) ->bool {
        self.mid.get_field(6, DataFields::SequenceNumberSupport as i32).get_value(OpenProtocolConvertT::string_to_bool)
    }

    pub fn set_sequence_number_support(&mut self, value:bool) {
        self.mid.get_field(6, DataFields::SequenceNumberSupport as i32).set_value2::<bool>(OpenProtocolConvertT::tp_bool_to_string, value);
    }

    pub fn linking_handling_support(&mut self) ->bool {
        self.mid.get_field(6, DataFields::LinkingHandlingSupport as i32).get_value(OpenProtocolConvertT::string_to_bool)
    }

    pub fn set_linking_handling_support(&mut self, value:bool) {
        self.mid.get_field(6, DataFields::LinkingHandlingSupport as i32).set_value2::<bool>(OpenProtocolConvertT::tp_bool_to_string, value);
    }

    /// <summary>
    /// <para>Station ID for PF6000</para>
    /// <para>Cell ID for PF4000</para>
    /// </summary>
    pub fn station_cell_id(&mut self) ->i64 {
        self.mid.get_field(6, DataFields::StationCellId as i32).get_value(OpenProtocolConvertT::string_to_int64)
    }

    pub fn set_station_cell_id(&mut self, value:i64) {
        self.mid.get_field(6, DataFields::StationCellId as i32).set_value2::<i64>(OpenProtocolConvertT::tp_i64_to_string, value);
    }

    /// <summary>
    /// <para>Station ID for PF6000</para>
    /// <para>Cell ID for PF4000</para>
    /// </summary>
    pub fn station_cell_name(&mut self) ->String {
        self.mid.get_field(6, DataFields::StationCellName as i32).value
    }

    pub fn set_station_cell_name(&mut self, value:String) {
        self.mid.get_field(6, DataFields::StationCellName as i32).set_value(value);
    }

    pub fn client_id(&mut self) ->String {
        self.mid.get_field(6, DataFields::ClientId as i32).value
    }

    pub fn set_client_id(&mut self, value:String) {
        self.mid.get_field(6, DataFields::ClientId as i32).set_value(value);
    }

    //Rev 7
    /// <summary>
    /// <para>False = Use Keep alive (Keep alive is mandatory)</para> 
    /// <para>True = Ignore Keep alive (Keep alive is optional)</para>
    /// </summary>
    pub fn optional_keep_alive(&mut self) ->bool {
        self.mid.get_field(7, DataFields::OptionalKeepAlive as i32).get_value(OpenProtocolConvertT::string_to_bool)
    }

    pub fn set_optional_keep_alive(&mut self, value:bool) {
        self.mid.get_field(7, DataFields::OptionalKeepAlive as i32).set_value2::<bool>(OpenProtocolConvertT::tp_bool_to_string, value);
    }

    pub fn new()->Self
    {
        Self::new_rev(Header::DEFAULT_REVISION)
    }

    pub fn new_header(header:HeaderT)->Self
    {
        Mid0002T { mid: MidT::new(header) }
    }

    pub fn new_rev(revision:i32) ->Self {
        let mut h:HeaderT = HeaderT::default();
        h.mid = Self::MID;
        h.revision = revision;
        Self::new_header(h)
    }

    pub(crate) fn register_datafields() -> HashMap<i32, Vec<DataFieldT>> {
        let mut hm:HashMap<i32, Vec<DataFieldT>>  = HashMap::new();
        let mut v1:Vec<DataFieldT>= Vec::new();
        v1.push(DataFieldT::number(DataFields::CellId as i32, 20, 4, Some(true)));
        v1.push(DataFieldT::number(DataFields::ChannelId as i32, 26, 2, Some(true)));
        v1.push(DataFieldT::string2(DataFields::ControllerName as i32, 30, 25, Some(true)));
        hm.insert(1, v1);

        let mut v2:Vec<DataFieldT>= Vec::new();
        v2.push(DataFieldT::string2(DataFields::SupplierCode as i32, 57, 3, Some(true)));
        hm.insert(2, v2);

        let mut v3:Vec<DataFieldT>= Vec::new();
        v3.push(DataFieldT::string2(DataFields::OpenProtocolVersion as i32, 62, 19, Some(true)));
        v3.push(DataFieldT::string2(DataFields::ControllerSoftwareVersion as i32, 83, 19, Some(true)));
        v3.push(DataFieldT::string2(DataFields::ToolSoftwareVersion as i32, 104, 19, Some(true)));
        hm.insert(3, v3);

        let mut v4:Vec<DataFieldT>= Vec::new();
        v4.push(DataFieldT::string2(DataFields::RBUType as i32, 125, 24, Some(true)));
        v4.push(DataFieldT::string2(DataFields::ControllerSerialNumber as i32, 151, 10, Some(true)));
        hm.insert(4, v4);

        let mut v5:Vec<DataFieldT>= Vec::new();
        v5.push(DataFieldT::number(DataFields::SystemType as i32, 163, 3, Some(true)));
        v5.push(DataFieldT::number(DataFields::SystemSubtype as i32, 168, 3, Some(true)));
        hm.insert(5, v5);

        let mut v6:Vec<DataFieldT>= Vec::new();
        v6.push(DataFieldT::boolean(DataFields::SequenceNumberSupport as i32, 173, Some(true)));
        v6.push(DataFieldT::boolean(DataFields::LinkingHandlingSupport as i32, 176, Some(true)));
        v6.push(DataFieldT::number(DataFields::StationCellId as i32, 179, 10, Some(true)));
        v6.push(DataFieldT::string2(DataFields::StationCellName as i32, 191, 25, Some(true)));
        v6.push(DataFieldT::string2(DataFields::ClientId as i32, 218, 1, Some(true)));
        hm.insert(6, v6);
        
        let mut v7:Vec<DataFieldT>= Vec::new();
        v7.push(DataFieldT::boolean(DataFields::OptionalKeepAlive as i32, 221, Some(true)));
        hm.insert(7, v7);
        
        hm
    }


}