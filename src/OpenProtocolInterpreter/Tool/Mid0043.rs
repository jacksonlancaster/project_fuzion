    /// <summary>
    /// Enable tool
    /// <para>Message sent by: Integrator</para>
    /// <para>Answer: <see cref="Communication.Mid0005"/> Command accepted</para>
    /// </summary>
    
    use crate::OpenProtocolInterpreter::MID::MidT;
    use crate::OpenProtocolInterpreter::OpenProtocolConvert::OpenProtocolConvertT;
    use crate::OpenProtocolInterpreter::Header::{self, HeaderT};
    use std::collections::HashMap;
    use crate::OpenProtocolInterpreter::DataField::DataFieldT;
    
    enum DataFields
    {
        ToolNumber,
        DisableType
    }
    
    #[derive(Clone)]
    pub struct Mid0043T { //:Mid, ITool, IIntegrator, IAcceptableCommand
        pub mid:MidT,
    }
    
    impl Mid0043T {
        pub const MID:i32 = 43;
    
        pub fn tool_number(&mut self) ->i32 {
            self.mid.get_field(2, DataFields::ToolNumber as i32).get_value(OpenProtocolConvertT::string_to_int32)
        }
    
        pub fn set_tool_number(&mut self, value:i32) {
            self.mid.get_field(2, DataFields::ToolNumber as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value);
        }
    
        pub fn new()->Self
        {
            Self::new_rev(Header::DEFAULT_REVISION)
        }
    
        pub fn new_header(header:HeaderT)->Self
        {
            Mid0043T { mid: MidT::new(header) }
        }
    
        pub fn new_rev(revision:i32) ->Self {
            let mut h:HeaderT = HeaderT::default();
            h.mid = Self::MID;
            h.revision = revision;
            Self::new_header(h)
        }
    
        pub(crate) fn register_datafields() -> HashMap<i32, Vec<DataFieldT>> {
            let mut hm:HashMap<i32, Vec<DataFieldT>>  = HashMap::new();
            
            let mut v1:Vec<DataFieldT>= Vec::new();
            v1.push(DataFieldT::number(DataFields::ToolNumber as i32, 20, 4, None));
            hm.insert(2, v1);
    
            hm
        }
    }    