    /// <summary>
    /// Select Parameter set
    /// <para>Message sent by: Integrator</para>
    /// <para>
    ///     Answer: <see cref="Communication.Mid0005"/> Command accepted or 
    ///     <see cref="Communication.Mid0004"/> Command error, Parameter set can not be set
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
    ParameterSetId
}

#[derive(Clone)]
pub struct Mid0018T { //Mid, IParameterSet, IIntegrator, IAcceptableCommand, IDeclinableCommand
    pub mid:MidT,
}

impl Interfaces::IDeclinableCommand for Mid0018T {
    fn documented_possible_errors(&self) -> Box<dyn Iterator<Item = Enums::Error> + '_> {
        Box::new([Enums::Error::ParameterSetCannotBeSet].into_iter())
    }
}

impl Mid0018T {
    pub const MID:i32 = 18;

    pub fn parameter_set_id(&mut self) ->i32 {
        self.mid.get_field(self.clone().mid.header.standardized_revision(), DataFields::ParameterSetId as i32).get_value(OpenProtocolConvertT::string_to_int32)
    }

    pub fn set_parameter_set_id(&mut self, value:i32) {
        self.mid.get_field(self.clone().mid.header.standardized_revision(), DataFields::ParameterSetId as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value);
    }

    pub fn new()->Self {
        let hdr1 = HeaderT{mid:Self::MID, revision:Header::DEFAULT_REVISION, ..Default::default()};
        Self::new_header(hdr1)
    }

    pub fn new_header(header:HeaderT)->Self {
        Mid0018T { mid: MidT::new(header) }
    }

    pub(crate) fn register_datafields() -> HashMap<i32, Vec<DataFieldT>> {
        let mut hm:HashMap<i32, Vec<DataFieldT>>  = HashMap::new();
        
        let mut v1:Vec<DataFieldT>= Vec::new();
        v1.push(DataFieldT::number(DataFields::ParameterSetId as i32, 20, 3, Some(false)));
    
        hm.insert(1, v1);

        hm
    }
}