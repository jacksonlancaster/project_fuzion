use crate::OpenProtocolInterpreter::Enums::PaddingOrientation;

pub struct OpenProtocolConvertT {

}

impl OpenProtocolConvertT {
    
    pub fn to_string(value:bool) -> String {
            if value  {"1".to_string()} else {"0".to_string()}
    }

    pub fn to_string2(padding_char:char, size:i32, orientation:PaddingOrientation, value:bool)->String {
            Self::to_string(value)
    }

    pub fn to_boolean(value:String)->bool {
        let mut int_value:i32 = 0;
        if value != "" {
            int_value = value.parse::<i32>().unwrap();
        }

        int_value != 0
        //return Convert.ToBoolean(intValue);
    }

    pub fn truncate_padded(padding_char:char, size:i32, orientation:PaddingOrientation, value:String)->String {

        String::new()
    }
}