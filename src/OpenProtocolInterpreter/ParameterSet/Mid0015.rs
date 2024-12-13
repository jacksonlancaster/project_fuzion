/// <summary>
/// Parameter set selected
/// <para>
///     A new parameter set is selected in the controller. 
///     The message includes the ID of the parameter set selected as well as the date and time of the 
///     last change in the parameter set settings. This message is also sent as an immediate response to <see cref="Mid0014"/> 
///     Parameter set selected subscribe.
/// </para>
/// <para>Message sent by: Controller</para>
/// <para>Answer: <see cref="Mid0016"/> New parameter set selected acknowledge</para>
/// </summary>
use chrono::{DateTime, TimeZone, Local, Utc};
use crate::OpenProtocolInterpreter::MID::MidT;
use crate::OpenProtocolInterpreter::OpenProtocolConvert::OpenProtocolConvertT;
use crate::OpenProtocolInterpreter::Enums;
use crate::OpenProtocolInterpreter::Header::{self, HeaderT};
use std::collections::HashMap;
use crate::OpenProtocolInterpreter::DataField::DataFieldT;
//use std::time::SystemTime;

pub(crate)  enum DataFields
{
    ParameterSetId,
    LastChangeInParameterSet,
    //Rev 2
    ParameterSetName,
    RotationDirection,
    BatchSize,
    TorqueMin,
    TorqueMax,
    TorqueFinalTarget,
    AngleMin,
    AngleMax,
    FinalAngleTarget,
    FirstTarget,
    StartFinalAngle
}

#[derive(Clone)]
pub struct Mid0015T { //Mid, IParameterSet, IController, IAcknowledgeable<Mid0016>
    pub mid:MidT,
}

impl Mid0015T {
    pub const MID:i32 = 15;

    pub fn parameter_set_id(&mut self) ->i32 {
        self.mid.get_field(self.clone().mid.header.standardized_revision(), DataFields::ParameterSetId as i32).get_value(OpenProtocolConvertT::string_to_int32)
    }

    pub fn set_parameter_set_id(&mut self, value:i32) {
        self.mid.get_field(self.clone().mid.header.standardized_revision(), DataFields::ParameterSetId as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value);
    }

    /*pub fn LastChangeInParameterSet<Tz>(&mut self) ->DateTime<Tz>
    where
    Tz: TimeZone,*/
    pub fn last_change_in_parameter_set(&mut self) ->DateTime<Local>
    {
        self.mid.get_field(self.clone().mid.header.standardized_revision(), DataFields::LastChangeInParameterSet as i32).get_value(OpenProtocolConvertT::string_to_date_time)
    }

    pub fn set_last_change_in_parameter_set(&mut self, value:DateTime<Local>) {
        self.mid.get_field(self.clone().mid.header.standardized_revision(), DataFields::LastChangeInParameterSet as i32).set_value2::<DateTime<Local>>(OpenProtocolConvertT::tp_date_time_to_string, value);
    }

    //Rev 2
    pub fn parameter_set_name(&mut self) ->String {
        self.mid.get_field(2, DataFields::ParameterSetName as i32).value
    }

    pub fn set_parameter_set_name(&mut self, value:String) {
        self.mid.get_field(2, DataFields::ParameterSetName as i32).set_value(value);
    }

    pub fn rotation_direction(&mut self) ->Enums::RotationDirection {
        let val = self.mid.get_field(2, DataFields::RotationDirection as i32).get_value(OpenProtocolConvertT::string_to_int32);
        let rd:Enums::RotationDirection = unsafe { ::std::mem::transmute(val) };

        rd
    }

    pub fn set_rotation_direction(&mut self, value:Enums::RotationDirection) {
        self.mid.get_field(2, DataFields::RotationDirection as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value as i32);
    }

    pub fn batch_size(&mut self) ->i32 {
        self.mid.get_field(2, DataFields::BatchSize as i32).get_value(OpenProtocolConvertT::string_to_int32)
    }

    pub fn set_batch_size(&mut self, value:i32) {
        self.mid.get_field(2, DataFields::BatchSize as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value);
    }

    pub fn min_torque(&mut self) ->f32 {
        self.mid.get_field(2, DataFields::TorqueMin as i32).get_value(OpenProtocolConvertT::string_to_truncated_decimal)
    }

    pub fn set_min_torque(&mut self, value:f32) {
        self.mid.get_field(2, DataFields::TorqueMin as i32).set_value2::<f32>(OpenProtocolConvertT::tp_truncated_decimal_to_string, value);
    }

    pub fn max_torque(&mut self) ->f32 {
        self.mid.get_field(2, DataFields::TorqueMax as i32).get_value(OpenProtocolConvertT::string_to_truncated_decimal)
    }

    pub fn set_max_torque(&mut self, value:f32) {
        self.mid.get_field(2, DataFields::TorqueMax as i32).set_value2::<f32>(OpenProtocolConvertT::tp_truncated_decimal_to_string, value);
    }    

    pub fn torque_final_target(&mut self) ->f32 {
        self.mid.get_field(2, DataFields::TorqueFinalTarget as i32).get_value(OpenProtocolConvertT::string_to_truncated_decimal)
    }

    pub fn set_torque_final_target(&mut self, value:f32) {
        self.mid.get_field(2, DataFields::TorqueFinalTarget as i32).set_value2::<f32>(OpenProtocolConvertT::tp_truncated_decimal_to_string, value);
    }

    pub fn min_angle(&mut self) ->i32 {
        self.mid.get_field(2, DataFields::AngleMin as i32).get_value(OpenProtocolConvertT::string_to_int32)
    }

    pub fn set_min_angle(&mut self, value:i32) {
        self.mid.get_field(2, DataFields::AngleMin as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value);
    }

    pub fn max_angle(&mut self) ->i32 {
        self.mid.get_field(2, DataFields::AngleMax as i32).get_value(OpenProtocolConvertT::string_to_int32)
    }

    pub fn set_max_angle(&mut self, value:i32) {
        self.mid.get_field(2, DataFields::AngleMax as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value);
    }

    pub fn angle_final_target(&mut self) ->i32 {
        self.mid.get_field(2, DataFields::FinalAngleTarget as i32).get_value(OpenProtocolConvertT::string_to_int32)
    }

    pub fn set_angle_final_target(&mut self, value:i32) {
        self.mid.get_field(2, DataFields::FinalAngleTarget as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value);
    }

    pub fn first_target(&mut self) ->f32 {
        self.mid.get_field(2, DataFields::FirstTarget as i32).get_value(OpenProtocolConvertT::string_to_truncated_decimal)
    }

    pub fn set_first_target(&mut self, value:f32) {
        self.mid.get_field(2, DataFields::FirstTarget as i32).set_value2::<f32>(OpenProtocolConvertT::tp_truncated_decimal_to_string, value);
    }

    pub fn start_final_angle(&mut self) ->f32 {
        self.mid.get_field(2, DataFields::StartFinalAngle as i32).get_value(OpenProtocolConvertT::string_to_truncated_decimal)
    }

    pub fn set_start_final_angle(&mut self, value:f32) {
        self.mid.get_field(2, DataFields::StartFinalAngle as i32).set_value2::<f32>(OpenProtocolConvertT::tp_truncated_decimal_to_string, value);
    }

    pub fn new()->Self {
        Self::new_rev(Header::DEFAULT_REVISION)
    }

    pub fn new_header(header:HeaderT)->Self {
        Mid0015T { mid: MidT::new(header) }
    }

    pub fn new_rev(revision:i32) -> Self {
        let hdr1 = HeaderT{mid:Self::MID, revision:revision, ..Default::default()};
        Self::new_header(hdr1)
    }

    pub(crate) fn build_header(&mut self)->String {
        self.mid.header.length = 20;

        let data_fields = self.mid.revisions_by_fields.get(&self.mid.header.standardized_revision());
        for df in data_fields.unwrap() {
            self.mid.header.length += (if df.has_prefix {2} else {0}) + df.size;
        }
        
        self.mid.header.to_string()
    }
    
    pub fn set_header(&mut self, hdr:HeaderT) {
        self.mid.header = hdr
    }

    pub fn process_header(&mut self, package:String)->HeaderT {
        self.mid.process_header(package)
    }

    pub fn pack(&mut self) ->String
    {
        let mut index = 1;
        self.mid.build_header() + self.mid.pack2(self.mid.header.clone().standardized_revision(), &mut index).as_str()
    }

    pub fn parse(&mut self, package:String)->Self
    {
        self.mid.header = self.mid.process_header(package.clone());
        self.mid.process_data_fields2(self.mid.header.clone().standardized_revision(), package);
        return self.clone();
    }

    pub(crate) fn register_datafields() -> HashMap<i32, Vec<DataFieldT>> {
        let mut hm:HashMap<i32, Vec<DataFieldT>>  = HashMap::new();
        
        let mut v1:Vec<DataFieldT>= Vec::new();
        v1.push(DataFieldT::number(DataFields::ParameterSetId as i32, 20, 3, Some(false)));
        v1.push(DataFieldT::timestamp(DataFields::LastChangeInParameterSet as i32, 23, Some(false)));

        hm.insert(1, v1);

        let mut v2:Vec<DataFieldT>= Vec::new();
        v2.push(DataFieldT::number(DataFields::ParameterSetId as i32, 20, 3, None));
        v2.push(DataFieldT::string2(DataFields::ParameterSetName as i32, 25, 25, None));
        v2.push(DataFieldT::timestamp(DataFields::LastChangeInParameterSet as i32, 52, None));
        v2.push(DataFieldT::number(DataFields::RotationDirection as i32, 73, 1, None));
        v2.push(DataFieldT::number(DataFields::BatchSize as i32, 76, 2, None));
        v2.push(DataFieldT::number(DataFields::TorqueMin as i32, 80, 6, None));
        v2.push(DataFieldT::number(DataFields::TorqueMax as i32, 88, 6, None));
        v2.push(DataFieldT::number(DataFields::TorqueFinalTarget as i32, 96, 6, None));
        v2.push(DataFieldT::number(DataFields::AngleMin as i32, 104, 5, None));
        v2.push(DataFieldT::number(DataFields::AngleMax as i32, 111, 5, None));
        v2.push(DataFieldT::number(DataFields::FinalAngleTarget as i32, 118, 5, None));
        v2.push(DataFieldT::number(DataFields::FirstTarget as i32, 125, 6, None));
        v2.push(DataFieldT::number(DataFields::StartFinalAngle as i32, 133, 6, None));
        hm.insert(2, v2);

        hm
    }
}