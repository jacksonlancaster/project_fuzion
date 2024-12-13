    /// <summary>
    /// Parameter set data upload request
    /// <para>Request to upload parameter set data from the controller.</para>
    /// <para>Message sent by: Integrator</para>
    /// <para>
    /// Answer: <see cref="Mid0013"/> Parameter set data upload reply, or 
    ///         <see cref="Communication.Mid0004"/> Command error, Parameter set not present
    /// </para>
    /// </summary>
       
use crate::OpenProtocolInterpreter::MID::MidT;
use crate::OpenProtocolInterpreter::OpenProtocolConvert::OpenProtocolConvertT;
use crate::OpenProtocolInterpreter::Enums;
use crate::OpenProtocolInterpreter::Header::{self, HeaderT};
use std::collections::HashMap;
use crate::OpenProtocolInterpreter::DataField::DataFieldT;
use crate::OpenProtocolInterpreter::Interfaces;

pub(crate)  enum DataFields
{
    //Revision 1-2
    ParameterSetId,
    
    //Revision 3-4
    PSetFileVersion
}

#[derive(Clone)]
pub struct Mid0012T { //:Mid, IParameterSet, IIntegrator, IAnswerableBy<Mid0013>, IDeclinableCommand
    pub mid:MidT,
}

impl Interfaces::IDeclinableCommand for Mid0012T {
    fn documented_possible_errors(&self) -> Box<dyn Iterator<Item = Enums::Error> + '_> {
        Box::new([Enums::Error::ParameterSetIdNotPresent].into_iter())
    }
}

impl Mid0012T {
    pub const MID:i32 = 12;

    pub fn parameter_set_id(&mut self) ->i32 {
        self.mid.get_field(1, DataFields::ParameterSetId as i32).get_value(OpenProtocolConvertT::string_to_int32)
    }

    pub fn set_parameter_set_id(&mut self, value:i32) {
        self.mid.get_field(1, DataFields::ParameterSetId as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value);
    }

    pub fn parameter_set_file_version(&mut self) ->i32 {
        self.mid.get_field(3, DataFields::PSetFileVersion as i32).get_value(OpenProtocolConvertT::string_to_int32)
    }

    pub fn set_parameter_set_file_version(&mut self, value:i32) {
        self.mid.get_field(3, DataFields::PSetFileVersion as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value);
    }

    pub fn new()->Self {
        Self::new_rev(Header::DEFAULT_REVISION)
    }

    pub fn new_header(header:HeaderT)->Self {
        Mid0012T { mid: MidT::new(header) }
    }

    pub fn new_rev(revision:i32) -> Self {
        let hdr1 = HeaderT{mid:Self::MID, revision:revision, ..Default::default()};
        Self::new_header(hdr1)
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

    pub(crate) fn register_datafields() -> HashMap<i32, Vec<DataFieldT>> {
        let mut hm:HashMap<i32, Vec<DataFieldT>>  = HashMap::new();
        
        let mut v1:Vec<DataFieldT>= Vec::new();
        v1.push(DataFieldT::number(DataFields::ParameterSetId as i32, 20, 3, Some(false)));
    
        hm.insert(1, v1);

        let mut v2:Vec<DataFieldT>= Vec::new();
        v2.push(DataFieldT::number(DataFields::PSetFileVersion as i32, 23, 8, Some(false)));
    
        hm.insert(3, v2);

        hm
    }
}