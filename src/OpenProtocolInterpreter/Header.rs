use crate::OpenProtocolInterpreter::Utils;
use substring::Substring;
use std::default::Default;

pub const DEFAULT_REVISION:i32 = 1;

#[derive(Copy, Clone, Default)]
pub struct HeaderT {
    pub length:i32,
    pub mid:i32,
    pub revision:i32,
    pub standardized_revision:i32,
    pub no_ack_flag:bool,
    pub station_id:i32,
    pub spindle_id:i32,
    pub sequence_number:i32,
    pub number_of_messages:i32,
    pub message_number:i32
}

impl HeaderT {

    pub fn parse_int(pkg:String) -> (bool, i32) {
        let mut value = 0;
        let mut parse_success = true;
        match pkg.parse::<i32>() {
            Ok(n) => value = n,
            Err(_e) => parse_success = false,
        }

        (parse_success, value)
    }
    pub fn is_not_empty_or_zero(pkg:String) -> (bool, i32) {
        let (parse_success, value) = Self::parse_int(pkg.clone());

        (!Utils::is_null_or_white_space(pkg) && parse_success  && value > 0, value)
    }

    pub fn process_header(package:String) -> Self {
        let mut hdr: HeaderT = Default::default();
        let mut parse_success:bool;
        let mut flag:bool;
        let mut value:i32;

        hdr.length = package.substring(0, 4).parse::<i32>().expect("Failed parsing integer value");
        hdr.mid = package.substring(4, 8).parse::<i32>().expect("Failed parsing integer value");
        (flag, value) = Self::is_not_empty_or_zero(package.substring(8, 11).to_string());
        hdr.revision = if flag {value} else {1};
        hdr.no_ack_flag = !Utils::is_null_or_white_space(package.substring(11, 12).to_string());
        (parse_success, hdr.station_id) = Self::parse_int(package.substring(12, 14).to_string());
        if !parse_success { 
            hdr.station_id =1;
        }
        (parse_success, hdr.spindle_id) = Self::parse_int(package.substring(14, 16).to_string());
        if !parse_success { 
            hdr.spindle_id =1;
        }

        (flag, value) = Self::is_not_empty_or_zero(package.substring(16, 18).to_string());
        hdr.sequence_number = if flag {value} else {Default::default()};
        (flag, value) = Self::is_not_empty_or_zero(package.substring(18, 19).to_string());
        hdr.number_of_messages = if flag {value} else {Default::default()};
        (flag, value) = Self::is_not_empty_or_zero(package.substring(19, 20).to_string());
        hdr.message_number = if flag {value} else {Default::default()};

        hdr
    }

    pub fn to_string(&mut self) -> String {
        let str_val;

        let no_ack_flag_str:String;

        match self.no_ack_flag {
            true => {
                no_ack_flag_str = "1".to_string();
            } 
            false => {
                no_ack_flag_str = " ".to_string();
            }
        }
        str_val = format!("{}{}{}{}{}{}{}{}{}{}", self.length, self.mid, self.revision, self.standardized_revision,
                                    no_ack_flag_str, self.station_id, self.spindle_id, self.sequence_number,
                                    self.number_of_messages, self.message_number);
        
        str_val
    }
}