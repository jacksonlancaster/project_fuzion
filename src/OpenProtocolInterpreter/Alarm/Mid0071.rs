use std::collections::HashMap;

use chrono::{DateTime, Local};

use crate::OpenProtocolInterpreter::DataField::DataFieldT;
use crate::OpenProtocolInterpreter::Enums;
/// <summary>
/// Alarm
/// <para>An alarm has appeared in the controller. The current alarm is uploaded from the controller to the integrator.</para>
/// <para>Message sent by: Controller</para>
/// <para>Answer: <see cref="Mid0072"/> Alarm acknowledge</para>
/// </summary>
use crate::OpenProtocolInterpreter::Header::{self, HeaderT};
use crate::OpenProtocolInterpreter::OpenProtocolConvert::OpenProtocolConvertT;
use crate::OpenProtocolInterpreter::Utils;
use crate::OpenProtocolInterpreter::MID::MidT;

pub(crate) enum DataFields
{
    ErrorCode,
    ControllerReadyStatus,
    ToolReadyStatus,
    Time,
    //Rev 2
    AlarmText
}

#[derive(Default, Clone)]
pub struct Mid0071T { //:Mid, IAlarm, IController, IAcknowledgeable<Mid0072>
    pub mid:MidT,
}

impl Mid0071T {

        pub const MID:i32 = 71;

        pub fn error_code(&mut self) ->String {
            self.mid.get_field(1, DataFields::ErrorCode as i32).value
        }
    
        pub fn set_error_code(&mut self, value:String) {
            self.mid.get_field(1, DataFields::ErrorCode as i32).set_value(value);
        }

        pub fn controller_ready_status(&mut self) ->bool {
            self.mid.get_field(1, DataFields::ControllerReadyStatus as i32).get_value(OpenProtocolConvertT::string_to_bool)
        }
    
        pub fn set_controller_ready_status(&mut self, value:bool) {
            self.mid.get_field(1, DataFields::ControllerReadyStatus as i32).set_value2::<bool>(OpenProtocolConvertT::tp_bool_to_string, value);
        }

        pub fn tool_ready_status(&mut self) ->bool {
            self.mid.get_field(1, DataFields::ToolReadyStatus as i32).get_value(OpenProtocolConvertT::string_to_bool)
        }
    
        pub fn set_tool_ready_status(&mut self, value:bool) {
            self.mid.get_field(1, DataFields::ToolReadyStatus as i32).set_value2::<bool>(OpenProtocolConvertT::tp_bool_to_string, value);
        }

        pub fn time(&mut self) ->DateTime<Local>
        {
            self.mid.get_field(1, DataFields::Time as i32).get_value(OpenProtocolConvertT::string_to_date_time)
        }
    
        pub fn set_time(&mut self, value:DateTime<Local>) {
            self.mid.get_field(1, DataFields::Time as i32).set_value2::<DateTime<Local>>(OpenProtocolConvertT::tp_date_time_to_string, value);
        }

        //Rev 2
        pub fn alarm_text(&mut self) ->String {
            self.mid.get_field(2, DataFields::AlarmText as i32).value
        }
    
        pub fn set_alarm_text(&mut self, value:String) {
            self.mid.get_field(2, DataFields::AlarmText as i32).set_value(value);
        }

        /*The following 3 methods are Common Methods to all MIDs  */
        pub fn new() -> Self {
            Self::new_rev(Header::DEFAULT_REVISION)
        }

        pub fn new_header(hdr:HeaderT) -> Self {
            Self{mid:MidT::new(hdr)}
        }

        pub fn new_rev(revision:i32) ->Self {
            let mut h:HeaderT = HeaderT::default();
            h.mid = Self::MID;
            h.revision = revision;
            Self::new_header(h)
        }

        pub fn pack(&mut self)->String {
            self.mid.pack()
        }

        pub fn process_header(&mut self, package:String)->HeaderT {
            self.mid.process_header(package)
        }

        pub fn parse(&mut self, package:String)->Self
        {
            self.mid.header = self.process_header(package.clone());
            self.handle_revision();
            self.mid.process_data_fields(package);
            
            self.clone()
        }

        pub(crate)  fn register_datafields()->HashMap<i32, Vec<DataFieldT>> 
        {
            let mut hm:HashMap<i32, Vec<DataFieldT>>  = HashMap::new();
        
            let mut v1:Vec<DataFieldT>= Vec::new();
            v1.push(DataFieldT::string(DataFields::ErrorCode as i32, 20, 4, Enums::PaddingOrientation::LeftPadded, None));
            v1.push(DataFieldT::boolean(DataFields::ControllerReadyStatus as i32, 26, None));
            v1.push(DataFieldT::boolean(DataFields::ToolReadyStatus as i32, 29, None));
            v1.push(DataFieldT::timestamp(DataFields::Time as i32, 32, None));
            hm.insert(1, v1);

            let mut v2:Vec<DataFieldT>= Vec::new();
            v2.push(DataFieldT::string(DataFields::AlarmText as i32, 54, 50, Enums::PaddingOrientation::LeftPadded, None));
            hm.insert(2, v2);

            hm
        }

        fn handle_revision(&mut self) {
            
            let mut error_code_field = self.mid.get_field(1, DataFields::ErrorCode as i32);
            if self.mid.header.revision > 1 {
                error_code_field.size = 5;
            }
            else {
                error_code_field.size = 4;
            }

            let mut index = error_code_field.index + error_code_field.size;
            let mut i:i32 = Utils::get_hash_code::<i32>(DataFields::ControllerReadyStatus as i32);
            let k = 1;
            let data_fields = self.mid.revisions_by_fields.get(&k).unwrap();

            while i < data_fields.len() as i32 {
                let mut field = self.clone().mid.get_field(1, i);
                field.index = 2 + index;
                index = field.index + field.size;
                i +=1;
            }
        }

    }