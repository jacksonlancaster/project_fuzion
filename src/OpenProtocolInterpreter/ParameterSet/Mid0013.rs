/// <summary>
/// Parameter set data upload reply
/// <para>Upload of parameter set data reply.</para>
/// <para>Message sent by: Controller</para>
/// <para>Answer: None</para>
/// </summary>
    
use chrono::{DateTime, Local};
use crate::OpenProtocolInterpreter::MID::MidT;
use crate::OpenProtocolInterpreter::OpenProtocolConvert::OpenProtocolConvertT;
use crate::OpenProtocolInterpreter::Enums;
use crate::OpenProtocolInterpreter::Header::{self, HeaderT};
use std::collections::HashMap;
use crate::OpenProtocolInterpreter::DataField::DataFieldT;

pub(crate)  enum DataFields
{
    ParameterSetId,
    ParameterSetName,
    RotationDirection,
    BatchSize,
    MinTorque,
    MaxTorque,
    TorqueFinalTarget,
    MinAngle,
    MaxAngle,
    AngleFinalTarget,
    
    //Rev 2
    FirstTarget,
    StartFinalTarget,

    //Rev 5
    LastChangeInParameterSet
}

#[derive(Clone)]
pub struct Mid0013T { //:Mid, IParameterSet, IController
    pub mid:MidT,
}

impl Mid0013T {
    pub const MID:i32 = 13;


    pub fn parameter_set_id(&mut self) ->i32 {
        self.mid.get_field(1, DataFields::ParameterSetId as i32).get_value(OpenProtocolConvertT::string_to_int32)
    }

    pub fn set_parameter_set_id(&mut self, value:i32) {
        self.mid.get_field(1, DataFields::ParameterSetId as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value);
    }

    pub fn parameter_set_name(&mut self) ->String {
        self.mid.get_field(1, DataFields::ParameterSetName as i32).value
    }

    pub fn set_parameter_set_name(&mut self, value:String) {
        self.mid.get_field(1, DataFields::ParameterSetName as i32).set_value(value);
    }

    pub fn rotation_direction(&mut self) ->Enums::RotationDirection {
        let val = self.mid.get_field(1, DataFields::RotationDirection as i32).get_value(OpenProtocolConvertT::string_to_int32);
        let rd:Enums::RotationDirection = unsafe { ::std::mem::transmute(val) };

        rd
    }

    pub fn set_rotation_direction(&mut self, value:Enums::RotationDirection) {
        self.mid.get_field(1, DataFields::RotationDirection as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value as i32);
    }

    pub fn batch_size(&mut self) ->i32 {
        self.mid.get_field(1, DataFields::BatchSize as i32).get_value(OpenProtocolConvertT::string_to_int32)
    }

    pub fn set_batch_size(&mut self, value:i32) {
        self.mid.get_field(1, DataFields::BatchSize as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value);
    }

    pub fn min_torque(&mut self) ->f32 {
        self.mid.get_field(1, DataFields::MinTorque as i32).get_value(OpenProtocolConvertT::string_to_truncated_decimal)
    }

    pub fn set_min_torque(&mut self, value:f32) {
        self.mid.get_field(1, DataFields::MinTorque as i32).set_value2::<f32>(OpenProtocolConvertT::tp_truncated_decimal_to_string, value);
    }

    pub fn max_torque(&mut self) ->f32 {
        self.mid.get_field(1, DataFields::MaxTorque as i32).get_value(OpenProtocolConvertT::string_to_truncated_decimal)
    }

    pub fn set_max_torque(&mut self, value:f32) {
        self.mid.get_field(1, DataFields::MaxTorque as i32).set_value2::<f32>(OpenProtocolConvertT::tp_truncated_decimal_to_string, value);
    }
    
    pub fn torque_final_target(&mut self) ->f32 {
        self.mid.get_field(1, DataFields::TorqueFinalTarget as i32).get_value(OpenProtocolConvertT::string_to_truncated_decimal)
    }

    pub fn set_torque_final_target(&mut self, value:f32) {
        self.mid.get_field(1, DataFields::TorqueFinalTarget as i32).set_value2::<f32>(OpenProtocolConvertT::tp_truncated_decimal_to_string, value);
    }

    pub fn min_angle(&mut self) ->i32 {
        self.mid.get_field(1, DataFields::MinAngle as i32).get_value(OpenProtocolConvertT::string_to_int32)
    }

    pub fn set_min_angle(&mut self, value:i32) {
        self.mid.get_field(1, DataFields::MinAngle as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value);
    }

    pub fn max_angle(&mut self) ->i32 {
        self.mid.get_field(1, DataFields::MaxAngle as i32).get_value(OpenProtocolConvertT::string_to_int32)
    }

    pub fn set_max_angle(&mut self, value:i32) {
        self.mid.get_field(1, DataFields::MaxAngle as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value);
    }

    pub fn angle_final_target(&mut self) ->i32 {
        self.mid.get_field(1, DataFields::AngleFinalTarget as i32).get_value(OpenProtocolConvertT::string_to_int32)
    }

    pub fn set_angle_final_target(&mut self, value:i32) {
        self.mid.get_field(1, DataFields::AngleFinalTarget as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value);
    }

    //Rev 2
    pub fn first_target(&mut self) ->f32 {
        self.mid.get_field(2, DataFields::FirstTarget as i32).get_value(OpenProtocolConvertT::string_to_truncated_decimal)
    }

    pub fn set_first_target(&mut self, value:f32) {
        self.mid.get_field(2, DataFields::FirstTarget as i32).set_value2::<f32>(OpenProtocolConvertT::tp_truncated_decimal_to_string, value);
    }

    pub fn start_final_angle(&mut self) ->f32 {
        self.mid.get_field(2, DataFields::StartFinalTarget as i32).get_value(OpenProtocolConvertT::string_to_truncated_decimal)
    }

    pub fn set_start_final_angle(&mut self, value:f32) {
        self.mid.get_field(2, DataFields::StartFinalTarget as i32).set_value2::<f32>(OpenProtocolConvertT::tp_truncated_decimal_to_string, value);
    }

    //Rev 5
    pub fn last_change_in_parameter_set(&mut self) ->DateTime<Local>
    {
        self.mid.get_field(5, DataFields::LastChangeInParameterSet as i32).get_value(OpenProtocolConvertT::string_to_date_time)
    }

    pub fn set_last_change_in_parameter_set(&mut self, value:DateTime<Local>) {
        self.mid.get_field(5, DataFields::LastChangeInParameterSet as i32).set_value2::<DateTime<Local>>(OpenProtocolConvertT::tp_date_time_to_string, value);
    }

    pub fn new()->Self {
        Self::new_rev(Header::DEFAULT_REVISION)
    }

    pub fn new_header(header:HeaderT)->Self {
        Mid0013T { mid: MidT::new(header) }
    }

    pub fn new_rev(revision:i32) -> Self {
        let hdr1 = HeaderT{mid:Self::MID, revision:revision, ..Default::default()};
        Self::new_header(hdr1)
    }
    
    pub fn pack(&mut self)->String {
        self.mid.pack()
    }

    pub fn process_header(&mut self, package:String)->HeaderT {
        self.mid.process_header(package)
    }

    pub(crate) fn register_datafields() -> HashMap<i32, Vec<DataFieldT>> {
        let mut hm:HashMap<i32, Vec<DataFieldT>>  = HashMap::new();
        
        let mut v1:Vec<DataFieldT>= Vec::new();
        v1.push(DataFieldT::number(DataFields::ParameterSetId as i32, 20, 3, None));
        v1.push(DataFieldT::string2(DataFields::ParameterSetName as i32, 25, 25, None));
        v1.push(DataFieldT::number(DataFields::RotationDirection as i32, 52, 1, None));
        v1.push(DataFieldT::number(DataFields::BatchSize as i32, 55, 2, None));
        v1.push(DataFieldT::number(DataFields::MinTorque as i32, 59, 6, None));
        v1.push(DataFieldT::number(DataFields::MaxTorque as i32, 67, 6, None));
        v1.push(DataFieldT::number(DataFields::TorqueFinalTarget as i32, 75, 6, None));
        v1.push(DataFieldT::number(DataFields::MinAngle as i32, 83, 5, None));
        v1.push(DataFieldT::number(DataFields::MaxAngle as i32, 90, 5, None));
        v1.push(DataFieldT::number(DataFields::AngleFinalTarget as i32, 97, 5, None));
        hm.insert(1, v1);

        let mut v2:Vec<DataFieldT>= Vec::new();
        v2.push(DataFieldT::number(DataFields::FirstTarget as i32, 104, 6, None));
        v2.push(DataFieldT::number(DataFields::StartFinalTarget as i32, 112, 6, None));
        hm.insert(2, v2);

        let mut v3:Vec<DataFieldT>= Vec::new();
        v3.push(DataFieldT::timestamp(DataFields::LastChangeInParameterSet as i32, 120, None));
        hm.insert(5, v3);

        hm
    }
}