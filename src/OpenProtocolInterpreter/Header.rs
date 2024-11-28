use crate::OpenProtocolInterpreter::Utils;
use substring::Substring;
use std::default::Default;

pub const DEFAULT_REVISION:i32 = 1;

#[derive(Copy, Clone/*, Default*/)]
pub struct HeaderT {
    pub length:i32,
    pub mid:i32,
    pub revision:i32,
    pub standardized_revision:i32,
    pub no_ack_flag:bool,
    pub station_id:Option<i32>,
    pub spindle_id:Option<i32>,
    pub sequence_number:Option<i32>,
    pub number_of_messages:Option<i32>,
    pub message_number:Option<i32>
}

impl Default for HeaderT {
    fn default() -> Self {
        Self {
            length:0,
            mid:0,
            revision:0,
            standardized_revision:0,
            no_ack_flag:false,
            station_id: None,
            spindle_id: None,
            sequence_number: None,
            number_of_messages: None,
            message_number:None
        }
    }
}

impl HeaderT {

    pub fn standardized_revision(&mut self) -> i32 {
        self.standardized_revision = if self.revision > 0 {self.revision} else {1};
        self.standardized_revision
    }

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

        let package = if package.len() < 20 {format!("{: <20}", package)} else {package};

        hdr.length = package.substring(0, 4).parse::<i32>().expect("Failed parsing integer value");
        hdr.mid = package.substring(4, 8).parse::<i32>().expect("Failed parsing integer value");
        (flag, value) = Self::is_not_empty_or_zero(package.substring(8, 11).to_string());
        hdr.revision = if flag {value} else {1};
        hdr.no_ack_flag = !Utils::is_null_or_white_space(package.substring(11, 12).to_string());
        (parse_success, value) = Self::parse_int(package.substring(12, 14).to_string());
        if parse_success {
            hdr.station_id = Some(value);
        } else { 
            hdr.station_id =Some(1);
        }
        (parse_success, value) = Self::parse_int(package.substring(14, 16).to_string());
        if parse_success { 
            hdr.spindle_id = Some(value);
        } else {
            hdr.spindle_id =Some(1);
        }

        (flag, value) = Self::is_not_empty_or_zero(package.substring(16, 18).to_string());
        hdr.sequence_number = Some(if flag {value} else {Default::default()});
        (flag, value) = Self::is_not_empty_or_zero(package.substring(18, 19).to_string());
        hdr.number_of_messages = Some(if flag {value} else {Default::default()});
        (flag, value) = Self::is_not_empty_or_zero(package.substring(19, 20).to_string());
        hdr.message_number = Some(if flag {value} else {Default::default()});

        hdr
    }

    pub fn enforce_revision_standardization(&mut self) {
            self.revision = self.standardized_revision();
    }

    pub fn to_string(&mut self) -> String {
        let mut builder:String = String::new();

        builder.push_str(format!("{:04}", self.length).as_str());
        builder.push_str(format!("{:04}", self.mid).as_str());
        builder.push_str(Utils::format_i32_to_str(self.revision > 0, self.revision, 3).as_str());
        builder.push_str(Utils::format_to_str(self.no_ack_flag, 1, 1).as_str());
        builder.push_str(Utils::format_some_i32_to_str(self.station_id, 2).as_str()); 
        builder.push_str(Utils::format_some_i32_to_str(self.spindle_id, 2).as_str());
        builder.push_str(Utils::format_some_i32_to_str(self.sequence_number, 2).as_str());
        builder.push_str(Utils::format_some_i32_to_str(self.number_of_messages, 2).as_str());
        builder.push_str(Utils::format_some_i32_to_str(self.message_number, 2).as_str());
        
        builder
    }
}