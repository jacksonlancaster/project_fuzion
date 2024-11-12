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

pub struct Mid0002T {
    pub mid:MidT,
}

impl Mid0002T {
    pub const MID:i32 = 2;

    pub fn cell_id(self) ->i32 {
        self.mid.get_field(1, DataFields::CellId as i32).get_value(OpenProtocolConvertT::string_to_int32)
    }

    pub fn set_cell_id(self, value:i32) {
        self.mid.get_field(1, DataFields::CellId as i32).set_value2::<i32>(OpenProtocolConvertT::tp_to_string, value);
    }

    pub fn channel_id(self) ->i32 {
        self.mid.get_field(1, DataFields::ChannelId as i32).get_value(OpenProtocolConvertT::string_to_int32)
    }

    pub fn set_channel_id(self, value:i32) {
        self.mid.get_field(1, DataFields::ChannelId as i32).set_value2::<i32>(OpenProtocolConvertT::tp_to_string, value);
    }

    pub fn controller_name(self) ->String {
        self.mid.get_field(1, DataFields::ControllerName as i32).value
    }

    pub fn set_controller_name(self, value:String) {
        self.mid.get_field(1, DataFields::ControllerName as i32).set_value(value);
    }

    //Rev 2
    pub fn supplier_code(self) ->String {
        self.mid.get_field(2, DataFields::SupplierCode as i32).value
    }

    pub fn set_supplier_code(self, value:String) {
        self.mid.get_field(2, DataFields::SupplierCode as i32).set_value(value);
    }

    //Rev 3
    pub fn open_protocol_version(self) ->String {
        self.mid.get_field(3, DataFields::OpenProtocolVersion as i32).value
    }

    pub fn set_open_protocol_version(self, value:String) {
        self.mid.get_field(3, DataFields::OpenProtocolVersion as i32).set_value(value);
    }

    pub fn controller_software_version(self) ->String {
        self.mid.get_field(3, DataFields::ControllerSoftwareVersion as i32).value
    }

    pub fn set_controller_software_version(self, value:String) {
        self.mid.get_field(3, DataFields::ControllerSoftwareVersion as i32).set_value(value);
    }

    pub fn tool_software_version(self) ->String {
        self.mid.get_field(3, DataFields::ToolSoftwareVersion as i32).value
    }

    pub fn set_tool_software_version(self, value:String) {
        self.mid.get_field(3, DataFields::ToolSoftwareVersion as i32).set_value(value);
    }

    //Rev 4
    pub fn rbu_type(self) ->String {
        self.mid.get_field(4, DataFields::RBUType as i32).value
    }

    pub fn set_rbu_type(self, value:String) {
        self.mid.get_field(4, DataFields::RBUType as i32).set_value(value);
    }

    pub fn controller_serial_number(self) ->String {
        self.mid.get_field(4, DataFields::ControllerSerialNumber as i32).value
    }

    pub fn set_controller_serial_number(self, value:String) {
        self.mid.get_field(4, DataFields::ControllerSerialNumber as i32).set_value(value);
    }

    //Rev 5 
    pub fn system_type(self) ->Enums::SystemType {
        let v = self.mid.get_field(5, DataFields::SystemType as i32).get_value(OpenProtocolConvertT::string_to_int32);
    
        let st:Enums::SystemType = unsafe { ::std::mem::transmute(v) };

        st
    }

    pub fn set_system_type(self, value:Enums::SystemType) {
        self.mid.get_field(5, DataFields::SystemType as i32).set_value2::<i32>(OpenProtocolConvertT::tp_to_string, value as i32);
    }

    /// <summary>
    /// <para>If no subtype exists it will be set to 000</para>
    /// <para>For a Power Focus 4000 and PF 6000 system the valid subtypes are:</para>
    /// <para>001 = a normal tightening system</para>
    /// <para>For a Power MACS 4000 system the valid subtypes are:</para>
    /// <para>001 = a normal tightening system </para>
    /// <para>002 = a system running presses instead of spindles.</para>
    /// </summary>
    pub fn system_sub_type(self) ->Enums::SystemSubType {
        let v = self.mid.get_field(5, DataFields::SystemSubtype as i32).get_value(OpenProtocolConvertT::string_to_int32);
    
        let sst:Enums::SystemSubType = unsafe { ::std::mem::transmute(v) };

        sst
    }

    pub fn set_system_sub_type(self, value:Enums::SystemSubType) {
        self.mid.get_field(5, DataFields::SystemSubtype as i32).set_value2::<i32>(OpenProtocolConvertT::tp_to_string, value as i32);
    }

    //Rev 6
    pub fn sequence_number_support(self) ->bool {
        self.mid.get_field(6, DataFields::SequenceNumberSupport as i32).get_value(OpenProtocolConvertT::string_to_bool)
    }

    pub fn set_sequence_number_support(self, value:bool) {
        self.mid.get_field(6, DataFields::SequenceNumberSupport as i32).set_value2::<bool>(OpenProtocolConvertT::tp_bool_to_string, value);
    }

    pub fn linking_handling_support(self) ->bool {
        self.mid.get_field(6, DataFields::LinkingHandlingSupport as i32).get_value(OpenProtocolConvertT::string_to_bool)
    }

    pub fn set_linking_handling_support(self, value:bool) {
        self.mid.get_field(6, DataFields::LinkingHandlingSupport as i32).set_value2::<bool>(OpenProtocolConvertT::tp_bool_to_string, value);
    }
}