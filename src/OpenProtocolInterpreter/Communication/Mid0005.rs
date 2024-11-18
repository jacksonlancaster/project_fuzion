use crate::OpenProtocolInterpreter::MID::MidT;
use crate::OpenProtocolInterpreter::OpenProtocolConvert::OpenProtocolConvertT;
use crate::OpenProtocolInterpreter::Enums;
use crate::OpenProtocolInterpreter::Header::{self, HeaderT};
use std::collections::HashMap;
use crate::OpenProtocolInterpreter::DataField::DataFieldT;

enum DataFields
{
    MidAccepted
}

#[derive(Clone)]
pub struct Mid0005T {
    pub mid:MidT,
}

impl Mid0005T {
    pub const MID:i32 = 5;

    pub fn mid_accepted(&mut self) ->i32 {
        self.mid.get_field(1, DataFields::MidAccepted as i32).get_value(OpenProtocolConvertT::string_to_int32)
    }

    pub fn set_mid_accepted(&mut self, value:i32) {
        self.mid.get_field(1, DataFields::MidAccepted as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value);
    }

    pub fn new()->Self
    {
        Self::new_rev(Header::DEFAULT_REVISION)
    }

    pub fn new_header(header:HeaderT)->Self
    {
        Mid0005T { mid: MidT::new(header) }
    }

    pub fn new_rev(revision:i32) ->Self {
        let mut h:HeaderT = HeaderT::default();
        h.mid = Self::MID;
        h.revision = revision;
        Self::new_header(h)
    }

    pub fn pack(&mut self) ->String
    {
        self.mid.pack()
    }

    pub(crate) fn register_datafields() -> HashMap<i32, Vec<DataFieldT>> {
        let mut hm:HashMap<i32, Vec<DataFieldT>>  = HashMap::new();
        
        let mut v1:Vec<DataFieldT>= Vec::new();
        v1.push(DataFieldT::number(DataFields::MidAccepted as i32, 20, 4, Some(false)));
        hm.insert(1, v1);

        hm
    }
}