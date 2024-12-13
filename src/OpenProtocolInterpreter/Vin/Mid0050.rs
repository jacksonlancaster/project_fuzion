/// <summary>
/// Vehicle ID Number download request
/// <para>
///     This message is replaced by <see cref="MultipleIdentifiers.Mid0150"/>. <see cref="Mid0050"/> is still supported.
/// </para>
/// <para>Used by the integrator to send a VIN number to the controller.</para>
/// <para>Message sent by: Integrator</para>
/// <para>
///     Answer: <see cref="Communication.Mid0005"/> Command accepted or 
///             <see cref="Communication.Mid0004"/> Command error, VIN input source not granted
/// </para>
/// </summary>  
   
use crate::OpenProtocolInterpreter::MID::MidT;
use crate::OpenProtocolInterpreter::Enums;
use crate::OpenProtocolInterpreter::Header::{self, HeaderT};
use std::collections::HashMap;
use crate::OpenProtocolInterpreter::DataField::DataFieldT;
use crate::OpenProtocolInterpreter::Interfaces;

pub(crate)  enum DataFields
{
    VinNumber
}

#[derive(Clone)]
pub struct Mid0050T { //:Mid, IVin, IIntegrator, IAcceptableCommand, IDeclinableCommand
    pub mid:MidT,
}

impl Interfaces::IDeclinableCommand for Mid0050T {
    fn documented_possible_errors(&self) -> Box<dyn Iterator<Item = Enums::Error> + '_> {
        Box::new([Enums::Error::VINInputSourceNotGranted].into_iter())
    }
}

impl Mid0050T {
    pub const MID:i32 = 50;

    pub fn vin_number(&mut self) ->String {
        self.mid.get_field(1, DataFields::VinNumber as i32).value
    }

    pub fn set_vin_number(&mut self, value:String) {
        self.mid.get_field(1, DataFields::VinNumber as i32).set_value(value);
    }

    pub fn new()->Self {
        let hdr1 = HeaderT{mid:Self::MID, revision:Header::DEFAULT_REVISION, ..Default::default()};
        Self::new_header(hdr1)
    }

    pub fn new_header(header:HeaderT)->Self {
        Mid0050T { mid: MidT::new(header) }
    }

    pub fn set_header(&mut self, hdr:HeaderT) {
        self.mid.header = hdr
    }

    pub fn process_header(&mut self, package:String)->HeaderT {
        self.mid.process_header(package)
    }

    pub fn pack(&mut self)->String {
        self.mid.get_field(1, DataFields::VinNumber as i32).size = self.vin_number().len() as i32;
        return self.mid.pack();
    }

    pub fn parse(&mut self, package:String)->Self
    {
        self.mid.header = self.mid.process_header(package.clone());
        self.mid.get_field(1, DataFields::VinNumber as i32).size = self.mid.header.length - 20;
        self.mid.process_data_fields(package);// ProcessDataFields(package);
        
        self.clone()
    }


    pub(crate) fn register_datafields() -> HashMap<i32, Vec<DataFieldT>> {
        let mut hm:HashMap<i32, Vec<DataFieldT>>  = HashMap::new();
        
        let mut v1:Vec<DataFieldT>= Vec::new();
        v1.push(DataFieldT::volatile3(DataFields::VinNumber as i32, 20, Some(false)));//dynamic
    
        hm.insert(1, v1);

        hm
    }
}