/// <summary>
/// Vehicle ID Number
/// <para>Transmission of the current identifiers of the tightening by the controller to the subscriber.</para>
/// <para>The tightening result can be stamped with up to four identifiers:</para>
/// <list type="bullet">
///     <item>VIN number (identifier result part 1)</item>
///     <item>Identifier result part 2</item>
///     <item>Identifier result part 3</item>
///     <item>Identifier result part 4</item>
/// </list>
/// <para>
///     The identifiers are received by the controller from several input sources,
///     for example serial, Ethernet, or field bus.
/// </para>
/// <para>Message sent by: Controller</para>
/// <para>Answer: <see cref="Mid0053"/> Vehicle ID Number acknowledge</para>
/// </summary>


use crate::OpenProtocolInterpreter::MID::MidT;
use crate::OpenProtocolInterpreter::Header::{self, HeaderT};
use std::collections::HashMap;
use crate::OpenProtocolInterpreter::DataField::DataFieldT;

pub(crate)  enum DataFields
{
    VinNumber,
    IdentifierResultPart2,
    IdentifierResultPart3,
    IdentifierResultPart4,
}

#[derive(Clone)]
pub struct Mid0052T { //:Mid, IVin, IController, IAcknowledgeable<Mid0053>
    pub mid:MidT,
}

impl Mid0052T {
    pub const MID:i32 = 52;

    pub fn vin_number(&mut self) ->String {
        self.mid.get_field(1, DataFields::VinNumber as i32).value
    }

    pub fn set_vin_number(&mut self, value:String) {
        self.mid.get_field(1, DataFields::VinNumber as i32).set_value(value);
    }

    pub fn identifier_result_part2(&mut self) ->String {
        self.mid.get_field(2, DataFields::IdentifierResultPart2 as i32).value
    }

    pub fn set_identifier_result_part2(&mut self, value:String) {
        self.mid.get_field(2, DataFields::IdentifierResultPart2 as i32).set_value(value);
    }

    pub fn identifier_result_part3(&mut self) ->String {
        self.mid.get_field(2, DataFields::IdentifierResultPart3 as i32).value
    }

    pub fn set_identifier_result_part3(&mut self, value:String) {
        self.mid.get_field(2, DataFields::IdentifierResultPart3 as i32).set_value(value);
    }

    pub fn identifier_result_part4(&mut self) ->String {
        self.mid.get_field(2, DataFields::IdentifierResultPart4 as i32).value
    }

    pub fn set_identifier_result_part4(&mut self, value:String) {
        self.mid.get_field(2, DataFields::IdentifierResultPart4 as i32).set_value(value);
    }

    pub fn new()->Self
    {
        Self::new_rev(Header::DEFAULT_REVISION)
    }

    pub fn new_header(header:HeaderT)->Self
    {
        Mid0052T { mid: MidT::new(header) }
    }

    pub fn new_rev(revision:i32) ->Self {
        let mut h:HeaderT = HeaderT::default();
        h.mid = Self::MID;
        h.revision = revision;
        Self::new_header(h)
    }

    pub fn pack(&mut self)->String {
        let mut vin_number_field = self.mid.get_field(1, DataFields::VinNumber as i32);
        if self.mid.header.revision > 1 {
            vin_number_field.has_prefix = true;
        }

        //Can be up to 40 bytes long
        let vn_len = self.vin_number().len() as i32;
        vin_number_field.size = if  vn_len > 25 {vn_len} else {25};
        self.mid.pack()
    }

    pub fn parse(&mut self, package:String)->Self
    {
        self.mid.header = self.mid.process_header(package.clone());
        if self.mid.header.revision > 1 {
            let mut vin_number_field = self.mid.get_field(1, DataFields::VinNumber as i32);
            vin_number_field.has_prefix = true;
            vin_number_field.size = self.mid.header.length - 103;
            if vin_number_field.size > 25 {
                let added_size = vin_number_field.size - 25;
                self.mid.get_field(2, DataFields::IdentifierResultPart2 as i32).index += added_size;
                self.mid.get_field(2, DataFields::IdentifierResultPart3 as i32).index += added_size;
                self.mid.get_field(2, DataFields::IdentifierResultPart4 as i32).index += added_size;
            }
        } else {
            self.mid.get_field(1, DataFields::VinNumber as i32).size = self.mid.header.length - 20;
        }
        self.mid.process_data_fields(package);

        self.clone()
    }


    pub(crate) fn register_datafields() -> HashMap<i32, Vec<DataFieldT>> {
        let mut hm:HashMap<i32, Vec<DataFieldT>>  = HashMap::new();
        
        let mut v1:Vec<DataFieldT>= Vec::new();
        v1.push(DataFieldT::string2(DataFields::VinNumber as i32, 20, 25, Some(false)));
    
        hm.insert(1, v1);

        let mut v2:Vec<DataFieldT>= Vec::new();
        v2.push(DataFieldT::string2(DataFields::IdentifierResultPart2 as i32, 47, 25, None));
        v2.push(DataFieldT::string2(DataFields::IdentifierResultPart3 as i32, 74, 25, None));
        v2.push(DataFieldT::string2(DataFields::IdentifierResultPart4 as i32, 101, 25, None));
        hm.insert(2, v2);
        hm
    }
}