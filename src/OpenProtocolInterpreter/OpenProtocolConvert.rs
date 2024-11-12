use crate::OpenProtocolInterpreter::Enums::PaddingOrientation;

pub struct OpenProtocolConvertT {

}

impl OpenProtocolConvertT {
    
    pub fn bool_to_string(value:bool) -> String {
            if value  {"1".to_string()} else {"0".to_string()}
    }

    pub fn tp_bool_to_string(padding_char:char, size:i32, orientation:PaddingOrientation, value:bool)->String {
            Self::bool_to_string(value)
    }

    pub fn string_to_bool(value:String)->bool {
        let mut int_value:i32 = 0;
        if value != "" {
            int_value = value.parse::<i32>().unwrap();
        }

        int_value != 0
    }

    pub fn string_to_int32(value:String) ->i32 {
        let converted_value = value.parse::<i32>().unwrap();
        return converted_value;
    }

    pub fn int32_to_string(value:i32)->String {
        value.to_string()
    }

    pub fn tp_to_string(paddingChar:char, size:i32, orientation:PaddingOrientation, value:i32)->String {
        Self::truncate_padded(paddingChar, size, orientation, Self::int32_to_string(value))
    }

    pub fn truncate_padded(padding_char:char, size:i32, orientation:PaddingOrientation, value:String)->String {

        String::new()
    }
}