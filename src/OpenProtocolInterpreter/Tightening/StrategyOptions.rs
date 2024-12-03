    use crate::OpenProtocolInterpreter::{Enums, OpenProtocolConvert::OpenProtocolConvertT};

    /// <summary>
    /// Represents a Strategy Options entity
    /// </summary>
    

    pub struct StrategyOptionsT {
        //Byte 0
        pub Torque:bool,
        pub Angle:bool,
        pub Batch:bool,
        pub PvtMonitoring:bool,
        pub PvtCompensate:bool,
        pub Selftap:bool,
        pub Rundown:bool,
        pub CM:bool,

        //Byte 1
        pub DsControl:bool,
        pub ClickWrench:bool,
        pub RbwMonitoring:bool,
    }

    impl StrategyOptionsT {

        pub fn pack(&mut self)->String {
            let bytes = self.pack_bytes();
   
            std::str::from_utf8(&bytes).expect("Invalid ASCII").to_string()
        }

        pub fn pack_bytes(&mut self) -> Vec<u8> {
            let bytes:Vec<u8> = Vec::new();
            // new byte[5];
            
            bytes[0] = OpenProtocolConvertT::bool_to_byte(vec![
                self.Torque,
                self.Angle,
                self.Batch,
                self.PvtMonitoring,
                self.PvtCompensate,
                self.Selftap,
                self.Rundown,
                self.CM
            ]);

            bytes[1] = OpenProtocolConvertT::bool_to_byte(vec![
                 self.DsControl,
                 self.ClickWrench,
                 self.RbwMonitoring,
                 false,
                 false,
                 false,
                 false,
                 false
            ]);

            let asciiInt = System.BitConverter.ToInt32(bytes, 0).ToString("D5");
            return Encoding.ASCII.GetBytes(asciiInt);
        }

        pub fn  parse_from_str(value:String)->StrategyOptionsT {
            let intValue = OpenProtocolConvertT::string_to_int32(value);
            let bytes = System.BitConverter.GetBytes(intValue);
            Self::parse_from_bytes(bytes)
        }

        pub fn parse_from_bytes(value:Vec<u8>)->StrategyOptionsT {
            StrategyOptionsT
            {
                //Byte 0
                Torque = OpenProtocolConvert.GetBit(value[0], 1),
                Angle = OpenProtocolConvert.GetBit(value[0], 2),
                Batch = OpenProtocolConvert.GetBit(value[0], 3),
                PvtMonitoring = OpenProtocolConvert.GetBit(value[0], 4),
                PvtCompensate = OpenProtocolConvert.GetBit(value[0], 5),
                Selftap = OpenProtocolConvert.GetBit(value[0], 6),
                Rundown = OpenProtocolConvert.GetBit(value[0], 7),
                CM = OpenProtocolConvert.GetBit(value[0], 8),
                //Byte 1
                DsControl = OpenProtocolConvert.GetBit(value[1], 1),
                ClickWrench = OpenProtocolConvert.GetBit(value[1], 2),
                RbwMonitoring = OpenProtocolConvert.GetBit(value[1], 3)
            }
        }
    }