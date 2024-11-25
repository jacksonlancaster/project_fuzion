/// <summary>
/// Parameter set ID upload reply
/// <para>
///     The transmission of all the valid parameter set IDs of the controller. In the revision 000-001 the data
///     field contains the number of valid parameter sets currently present in the controller, and the ID of each
///     parameter set present.In revision 2 is the number of stages on each Pset/Mset added.
/// </para>    
/// <para>Message sent by: Controller</para>
/// <para>Answer: None</para>
/// </summary>

use std::collections::HashMap;
use substring::Substring;

use crate::OpenProtocolInterpreter::DataField::DataFieldT;

use crate::OpenProtocolInterpreter::Header::{self, HeaderT};
use crate::OpenProtocolInterpreter::OpenProtocolConvert::OpenProtocolConvertT;
use crate::OpenProtocolInterpreter::MID::MidT;
use crate::OpenProtocolInterpreter::Enums;


pub(crate) enum DataFields
{
    TotalParameterSets,
    EachParameterSet
}

#[derive(Default, Clone)]
pub struct Mid0011T { //:Mid, IParameterSet, IController
    pub mid:MidT,
    pub parameter_sets:Vec<i32>,
}

impl Mid0011T {

        pub const MID:i32 = 11;

        pub fn total_parameter_sets(&mut self) ->usize {
            self.parameter_sets.len()
        }

        /*The following 4 methods are Common Methods to all MIDs  */
        pub fn new() -> Self {
            Self::new_rev(Header::DEFAULT_REVISION)
        }

        pub fn new_header(hdr:HeaderT) -> Self {
            Self{mid:MidT::new(hdr), parameter_sets:Vec::new()}
        }

        fn new_rev(revision:i32) -> Self {
            let hdr1 = HeaderT{mid:Self::MID, revision:revision, ..Default::default()};
            Self::new_header(hdr1)
        }

        pub fn pack(&mut self) ->String
        {
            self.mid.get_field(1, DataFields::TotalParameterSets as i32).set_value2(OpenProtocolConvertT::tp_i32_to_string, self.total_parameter_sets() as i32);
            let mut each_parameter_field = self.mid.get_field(1, DataFields::EachParameterSet as i32);
            each_parameter_field.value = self.pack_parameter_set_id_list();
            each_parameter_field.size = each_parameter_field.value.len() as i32;
            self.mid.pack()
        }
        /* Common methods end here */
    
        pub(crate)  fn parse(&mut self, package:String)->Self {
            self.mid.header = self.mid.process_header(package.clone());

            self.mid.get_field(1, DataFields::EachParameterSet as i32).size = self.mid.header.length - self.mid.get_field(1, DataFields::EachParameterSet as i32).index;
            self.mid.process_data_fields(package);
            self.parameter_sets = self.parse_parameter_set_id_list(self.clone().mid.get_field(1, DataFields::EachParameterSet as i32).value).to_vec();
            return self.clone();
        }

        pub(crate) fn pack_parameter_set_id_list(&mut self)->String 
        {
            let mut pack = String::new();
            for v in self.clone().parameter_sets {
                pack.push_str(OpenProtocolConvertT::tp_i32_to_string('0', 3, Enums::PaddingOrientation::LeftPadded, v).as_str());
            }
            return pack;
        }

        pub(crate)  fn parse_parameter_set_id_list(&mut self, section:String)->Vec<i32> {
            let mut list:Vec<i32> = Vec::new();

            let mut i:usize =0;
            while  i < section.len() {
                list.push(OpenProtocolConvertT::string_to_int32(section.substring(i, i+3).to_string()));
                i += 3;
            }
            return list;
        }

        pub(crate) fn register_datafields(&mut self)->HashMap<i32, Vec<DataFieldT>> {
            let mut hmp:HashMap<i32, Vec<DataFieldT>> = HashMap::new();

            let mut v1:Vec<DataFieldT>= Vec::new();
            v1.push(DataFieldT::number(DataFields::TotalParameterSets as i32, 20, 3, Some(false)));
            v1.push(DataFieldT::new(DataFields::EachParameterSet as i32, 23, 3, Some(false)));
            hmp.insert(1, v1);

            hmp
        }

}