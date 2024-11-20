const MID:i32 = 1;

use std::collections::HashMap;
use crate::OpenProtocolInterpreter::DataField::DataFieldT;

use crate::OpenProtocolInterpreter::Header::{self, HeaderT};
use crate::OpenProtocolInterpreter::OpenProtocolConvert::OpenProtocolConvertT;
use crate::OpenProtocolInterpreter::MID::MidT;
use crate::OpenProtocolInterpreter::Interfaces;
use crate::OpenProtocolInterpreter::Enums;


enum DataFields
{
    //Rev 7
    UseKeepAlive
}

#[derive(Default, Clone/*, Copy*/)]
pub struct Mid0001T { //:Mid, ICommunication, IIntegrator, IAnswerableBy<Mid0002>, IDeclinableCommand
    //optional_keep_alive: bool,
    pub mid:MidT,
    //pub idc:Interfaces::IDeclinableCommand
}

impl Interfaces::ICommunication for Mid0001T {

}

impl Interfaces::IIntegrator for Mid0001T {

}

impl Interfaces::IAnswerableBy<MidT> for Mid0001T {
    fn get_answer_mid(&self) -> MidT {
        todo!()
    }
}

impl Interfaces::IDeclinableCommand for Mid0001T {
    fn documented_possible_errors(&self) -> Box<dyn Iterator<Item = Enums::Error> + '_> {
        Box::new([Enums::Error::ClientAlreadyConnected, Enums::Error::MidRevisionUnsupported].into_iter())
    }
}

impl Mid0001T {


        pub const MID:i32 = 1;

        // OptionalKeepAlive property with getter and setter methods
        pub fn optional_keep_alive(self) -> bool {
            self.clone().mid.get_field(7, DataFields::UseKeepAlive as i32).get_value(OpenProtocolConvertT::string_to_bool)

        }

        pub fn set_optional_keep_alive(&mut self, value: bool) {
            self.mid.get_field(7, DataFields::UseKeepAlive as i32).set_value2(OpenProtocolConvertT::tp_bool_to_string, value);
        }

        /*The following 4 methods are Common Methods to all MIDs  */
        pub fn new() -> Self {
            Self::new_rev(Header::DEFAULT_REVISION)
        }

        pub fn new_header(hdr:HeaderT) -> Self {
            Self{mid:MidT::new(hdr), ..Default::default()}
        }

        pub fn new_rev(revision:i32) -> Self {
            let hdr1 = HeaderT{mid:Self::MID, revision:revision, ..Default::default()};
            Self{mid:MidT::new(hdr1), ..Default::default()}
        }

        pub fn pack(&mut self) ->String
        {
            self.mid.pack()
        }
        /* Common methods end here */
    
        fn register_datafields(self)->HashMap<i32, Vec<DataFieldT>> {
            let mut hmp:HashMap<i32, Vec<DataFieldT>> = HashMap::new();

            let lst = vec![DataFieldT::boolean(DataFields::UseKeepAlive as i32, 20, Some(true)),];
            hmp.insert(7, lst);

            hmp
        }

}