    /// <summary>
    /// Disable tool
    /// <para>Message sent by: Integrator</para>
    /// <para>Answer: <see cref="Communication.Mid0005"/> Command accepted</para>
    /// </summary>
    
    use crate::OpenProtocolInterpreter::MID::MidT;
    use crate::OpenProtocolInterpreter::OpenProtocolConvert::OpenProtocolConvertT;
    use crate::OpenProtocolInterpreter::Enums;
    use crate::OpenProtocolInterpreter::Header::{self, HeaderT};
    use std::collections::HashMap;
    use crate::OpenProtocolInterpreter::DataField::DataFieldT;
    
    enum DataFields
    {
        ToolNumber,
        DisableType
    }
    
    #[derive(Clone)]
    pub struct Mid0042T { //:Mid, ITool, IIntegrator, IAcceptableCommand
        pub mid:MidT,
    }
    
    impl Mid0042T {
        pub const MID:i32 = 42;
    
        pub fn tool_number(&mut self) ->i32 {
            self.mid.get_field(2, DataFields::ToolNumber as i32).get_value(OpenProtocolConvertT::string_to_int32)
        }
    
        pub fn set_tool_number(&mut self, value:i32) {
            self.mid.get_field(2, DataFields::ToolNumber as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value);
        }

        pub fn disable_type(&mut self) ->Enums::DisableType {
            let val = self.mid.get_field(2, DataFields::DisableType as i32).get_value(OpenProtocolConvertT::string_to_int32);
            let dtype:Enums::DisableType = unsafe { ::std::mem::transmute(val) };
    
            dtype
        }
    
        pub fn set_disable_type(&mut self, value:Enums::DisableType) {
            self.mid.get_field(2, DataFields::DisableType as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value as i32);
        }
    
        pub fn new()->Self
        {
            Self::new_rev(Header::DEFAULT_REVISION)
        }
    
        pub fn new_header(header:HeaderT)->Self
        {
            Mid0042T { mid: MidT::new(header) }
        }
    
        pub fn new_rev(revision:i32) ->Self {
            let mut h:HeaderT = HeaderT::default();
            h.mid = Self::MID;
            h.revision = revision;
            Self::new_header(h)
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
            v1.push(DataFieldT::number(DataFields::ToolNumber as i32, 20, 4, None));
            v1.push(DataFieldT::number(DataFields::DisableType as i32, 26, 2, None));
            hm.insert(2, v1);
    
            hm
        }
    }    