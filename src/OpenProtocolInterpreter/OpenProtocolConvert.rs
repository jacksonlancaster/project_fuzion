use crate::OpenProtocolInterpreter::Enums::PaddingOrientation;
use chrono::{format::parse, DateTime, TimeZone, Local, Utc};

use super::Utils;

pub struct OpenProtocolConvertT {

}

impl OpenProtocolConvertT {
    const DATE_TIME_FORMAT:&str = "%Y-%m-%d:%H:%M:%S";

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

    /*pub fn date_time_to_string<Tz>(value:DateTime<Tz>) ->String 
    where
    Tz: TimeZone,
    Tz::Offset: std::fmt::Display,*/
    pub fn date_time_to_string(value:DateTime<Local>) ->String 
    {
        /*=> value.ToString("yyyy-MM-dd:HH:mm:ss");*/
        value.format(Self::DATE_TIME_FORMAT).to_string()
    }

    /*pub fn tp_date_time_to_string<Tz>(paddingChar:char, size:i32, orientation:PaddingOrientation, value:DateTime<Tz>)->String 
    where
    Tz: TimeZone,
    Tz::Offset: std::fmt::Display,*/
    pub fn tp_date_time_to_string(paddingChar:char, size:i32, orientation:PaddingOrientation, value:DateTime<Local>)->String 
    {
            /*=> ToString(value);*/
            //Self::date_time_to_string::<Tz>(value)
            Self::date_time_to_string(value)
    }

    pub fn string_to_date_time(value:String)->DateTime<Local>
    {
        
        //let convertedValue:DateTime<Utc> = Utc::now();
        //if (!string.IsNullOrWhiteSpace(value.ToString()))
        if !Utils::is_null_or_white_space(value.clone()) {
            //var date = value.ToString();
            //DateTime.TryParse($"{date.Substring(0, 10)} {date.Substring(11, 8)}", out convertedValue);
            DateTime::parse_from_str(value.as_str(), Self::DATE_TIME_FORMAT).unwrap().into()
        } else {
            Local::now()
        }

        //return convertedValue;
    }

    pub fn string_to_int32(value:String) ->i32 {
        let converted_value = value.parse::<i32>().unwrap();
        return converted_value;
    }

    pub fn int32_to_string(value:i32)->String {
        value.to_string()
    }

    pub fn string_to_decimal(value:String) -> f32
    {
        let mut decimal_value:f32 = 0.0;
        if !Utils::is_null_or_white_space(value.clone()) {
            decimal_value = value.replace(",", ".").parse::<f32>().unwrap();
        }
        decimal_value
    }

    pub fn truncated_decimal_to_string(value:f32) -> String
    {
        let converted_value:i32 =  (value * 100.0).round() as i32;// (int)(Math.Round(value, 2) * 100m);
        converted_value.to_string()
    }

    pub fn tp_truncated_decimal_to_string(padding_char:char, size:i32, orientation:PaddingOrientation, value:f32)->String
    {
        let str = Self::truncated_decimal_to_string(value);
        Self::truncate_padded(padding_char, size, orientation, str)
    }

    pub fn string_to_truncated_decimal(value:String) -> f32
    {
        let int_value:i32 = Self::string_to_int32(value);
        int_value as f32 / 100.0
    }

    pub fn tp_i32_to_string(padding_char:char, size:i32, orientation:PaddingOrientation, value:i32)->String {
        Self::truncate_padded(padding_char, size, orientation, Self::int32_to_string(value))
    }

    pub fn truncate_padded(padding_char:char, size:i32, orientation:PaddingOrientation, value:String)->String {

        String::new()
    }

    pub fn string_to_int64(value:String) ->i64 {
        let converted_value = value.parse::<i64>().unwrap();
        return converted_value;
    }

    pub fn int64_to_string(value:i64)->String {
        value.to_string()
    }

    pub fn tp_i64_to_string(padding_char:char, size:i32, orientation:PaddingOrientation, value:i64)->String {
            //=> TruncatePadded(padding_char, size, orientation, ToString(value));
            Self::truncate_padded(padding_char, size, orientation, Self::int64_to_string(value))
    }


}