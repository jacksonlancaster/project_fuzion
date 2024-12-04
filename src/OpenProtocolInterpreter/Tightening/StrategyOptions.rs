    use crate::OpenProtocolInterpreter::OpenProtocolConvert::OpenProtocolConvertT;

    /// <summary>
    /// Represents a Strategy Options entity
    /// </summary>
    
    #[derive(Clone)]
    pub struct StrategyOptionsT {
        //Byte 0
        pub torque:bool,
        pub angle:bool,
        pub batch:bool,
        pub pvt_monitoring:bool,
        pub pvt_compensate:bool,
        pub selftap:bool,
        pub rundown:bool,
        pub cm:bool,

        //Byte 1
        pub ds_control:bool,
        pub click_wrench:bool,
        pub rbw_monitoring:bool,
    }

    impl StrategyOptionsT {

        pub fn pack(&mut self)->String {
            let bytes = self.pack_bytes();
   
            std::str::from_utf8(&bytes).expect("Invalid ASCII").to_string()
        }

        pub fn pack_bytes(&mut self) -> Vec<u8> {

            let mut bytes: [u8; 5] = [0; 5];

            bytes[0] = OpenProtocolConvertT::bool_to_byte(vec![
                self.torque,
                self.angle,
                self.batch,
                self.pvt_monitoring,
                self.pvt_compensate,
                self.selftap,
                self.rundown,
                self.cm
            ]);

            bytes[1] = OpenProtocolConvertT::bool_to_byte(vec![
                 self.ds_control,
                 self.click_wrench,
                 self.rbw_monitoring,
                 false,
                 false,
                 false,
                 false,
                 false
            ]);

            let int_value = i32::from_le_bytes(bytes[0..4].try_into().unwrap());
            let ascii_int = format!("{:05}", int_value);
            ascii_int.into_bytes()
        }

        pub fn  parse_from_str(value:String)->StrategyOptionsT {
            
            let int_value = OpenProtocolConvertT::string_to_int32(value);
            let bytes = int_value.to_le_bytes().to_vec();
            
            Self::parse_from_bytes(bytes)
        }

        pub fn parse_from_bytes(value:Vec<u8>)->StrategyOptionsT {
            StrategyOptionsT
            {
                //Byte 0
                torque: OpenProtocolConvertT::get_bit(value[0], 1),
                angle: OpenProtocolConvertT::get_bit(value[0], 2),
                batch: OpenProtocolConvertT::get_bit(value[0], 3),
                pvt_monitoring: OpenProtocolConvertT::get_bit(value[0], 4),
                pvt_compensate: OpenProtocolConvertT::get_bit(value[0], 5),
                selftap: OpenProtocolConvertT::get_bit(value[0], 6),
                rundown: OpenProtocolConvertT::get_bit(value[0], 7),
                cm: OpenProtocolConvertT::get_bit(value[0], 8),

                //Byte 1
                ds_control: OpenProtocolConvertT::get_bit(value[1], 1),
                click_wrench: OpenProtocolConvertT::get_bit(value[1], 2),
                rbw_monitoring: OpenProtocolConvertT::get_bit(value[1], 3)
            }
        }
    }