use crate::OpenProtocolInterpreter::MID::MidT;
use crate::OpenProtocolInterpreter::OpenProtocolConvert::OpenProtocolConvertT;
use crate::OpenProtocolInterpreter::Enums;
use crate::OpenProtocolInterpreter::Header::{self, HeaderT};
use std::collections::HashMap;
use crate::OpenProtocolInterpreter::DataField::DataFieldT;

enum DataFields
{
    SubscriptionMid,
    WantedRevision,
    ExtraDataLength,
    ExtraData
}

#[derive(Clone)]
pub struct Mid0008T {
    pub mid:MidT,
}

impl Mid0008T {
    pub const MID:i32 = 8;

    pub fn subscription_mid(&mut self) ->String {
        self.mid.get_field(1, DataFields::SubscriptionMid as i32).value
    }

    pub fn set_subscription_mid(&mut self, value:String) {
        self.mid.get_field(1, DataFields::SubscriptionMid as i32).set_value(value);
    }

    pub fn wanted_revision(&mut self) ->i32 {
        self.mid.get_field(1, DataFields::WantedRevision as i32).get_value(OpenProtocolConvertT::string_to_int32)
    }

    pub fn set_wanted_revision(&mut self, value:i32) {
        self.mid.get_field(1, DataFields::WantedRevision as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value);
    }

    pub fn extra_data_length(&mut self) ->i32 {
        self.mid.get_field(1, DataFields::ExtraDataLength as i32).get_value(OpenProtocolConvertT::string_to_int32)
    }

    pub fn set_extra_data_length(&mut self, value:i32) {
        self.mid.get_field(1, DataFields::ExtraDataLength as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value);
    }

    pub fn extra_data(&mut self) ->String {
        self.mid.get_field(1, DataFields::ExtraData as i32).value
    }

    pub fn set_extra_data(&mut self, value:String) {
        self.mid.get_field(1, DataFields::ExtraData as i32).set_value(value);
    }

    pub fn new()->Self
    {
        let mut h:HeaderT = HeaderT::default();
        h.mid = Self::MID;
        h.revision = Header::DEFAULT_REVISION;
        Self::new_header(h)
    }

    pub fn new_header(header:HeaderT)->Self
    {
        Mid0008T { mid: MidT::new(header) }
    }

    pub fn set_header(&mut self, hdr:HeaderT) {
        self.mid.header = hdr
    }
    
    pub fn pack(&mut self)->String {
        self.mid.pack()
    }

    pub fn process_header(&mut self, package:String)->HeaderT {
        self.mid.process_header(package)
    }

    pub fn parse(&mut self, package:String)->Self
    {
        self.mid.header = self.mid.process_header(package.clone());
        self.mid.get_field(1, DataFields::ExtraData as i32).size = self.mid.header.length - 29;
        self.mid.process_data_fields(package);
        return self.clone();
    }

    pub(crate) fn register_datafields() -> HashMap<i32, Vec<DataFieldT>> {
        let mut hm:HashMap<i32, Vec<DataFieldT>>  = HashMap::new();
        
        let mut v1:Vec<DataFieldT>= Vec::new();
        v1.push(DataFieldT::number(DataFields::SubscriptionMid as i32, 20, 4, Some(false)));
        v1.push(DataFieldT::number(DataFields::WantedRevision as i32, 24, 3, Some(false)));
        v1.push(DataFieldT::number(DataFields::ExtraDataLength as i32, 27, 2, Some(false)));
        v1.push(DataFieldT::volatile3(DataFields::ExtraData as i32, 29, Some(false)));
        
        hm.insert(1, v1);

        hm
    }
}