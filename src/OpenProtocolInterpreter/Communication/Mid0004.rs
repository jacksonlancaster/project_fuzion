use crate::OpenProtocolInterpreter::MID::MidT;
use crate::OpenProtocolInterpreter::OpenProtocolConvert::OpenProtocolConvertT;
use crate::OpenProtocolInterpreter::Enums;
use crate::OpenProtocolInterpreter::Header::{self, HeaderT};
use std::collections::HashMap;
use crate::OpenProtocolInterpreter::DataField::DataFieldT;

enum DataFields
{
    Mid,
    ErrorCode
}

#[derive(Clone)]
pub struct Mid0004T {
    pub mid:MidT,
}

impl Mid0004T {
    pub const MID:i32 = 4;

    pub fn failed_mid(&mut self) ->i32 {
        self.mid.get_field(1, DataFields::Mid as i32).get_value(OpenProtocolConvertT::string_to_int32)
    }

    pub fn set_failed_mid(&mut self, value:i32) {
        self.mid.get_field(1, DataFields::Mid as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value);
    }

    pub fn error_code(&mut self) ->Enums::Error {
        let val = self.mid.get_field(1, DataFields::ErrorCode as i32).get_value(OpenProtocolConvertT::string_to_int32);
        let err:Enums::Error = unsafe { ::std::mem::transmute(val) };

        err
    }

    pub fn set_error_code(&mut self, value:Enums::Error) {
        self.mid.get_field(1, DataFields::ErrorCode as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value as i32);
    }

    pub fn new()->Self
    {
        Self::new_rev(Header::DEFAULT_REVISION)
    }

    pub fn new_header(header:HeaderT)->Self
    {
        Mid0004T { mid: MidT::new(header) }
    }

    pub fn new_rev(revision:i32) ->Self {
        let mut h:HeaderT = HeaderT::default();
        h.mid = Self::MID;
        h.revision = revision;
        Self::new_header(h)
    }

    pub fn pack(&mut self) ->String
    {
        self.handle_revision();
        self.mid.pack()
    }

    pub fn parse(&mut self, package:String)->Self
    {
        self.mid.header = self.mid.process_header(package.clone());
        self.handle_revision();
        self.mid.process_data_fields(package);
        return self.clone();
    }

    fn handle_revision(&mut self)
    {
        let mut  error_code_field = self.mid.get_field(1, DataFields::ErrorCode as i32);
        error_code_field.size = if self.mid.header.revision > 1 {3} else {2};
    }

    pub(crate) fn register_datafields() -> HashMap<i32, Vec<DataFieldT>> {
        let mut hm:HashMap<i32, Vec<DataFieldT>>  = HashMap::new();
        
        let mut v1:Vec<DataFieldT>= Vec::new();
        v1.push(DataFieldT::number(DataFields::Mid as i32, 20, 4, Some(false)));
        v1.push(DataFieldT::number(DataFields::ErrorCode as i32, 24, 2, Some(false)));
        hm.insert(1, v1);

        hm
    }
}