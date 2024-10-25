//use serde::{Deserialize, Serialize};
//use bincode;
use substring::Substring;
use std::default::Default;

pub const DEFAULT_REVISION:i32 = 1;

//#[derive(Deserialize, Copy, Clone, Default)]
#[derive(Copy, Clone, Default)]
pub struct Header_t {
    pub Length:i32,
    pub Mid:i32,
    pub Revision:i32,
    pub StandardizedRevision:i32,
    pub NoAckFlag:bool,
    pub StationId:i32,
    pub SpindleId:i32,
    pub SequenceNumber:i32,
    pub NumberOfMessages:i32,
    pub MessageNumber:i32
}

impl Header_t {

    pub fn ParseInt(pkg:String) -> (bool, i32) {
        let mut value = 0;
        let mut parse_success = true;
        match pkg.parse::<i32>() {
            Ok(n) => value = n,
            Err(_e) => parse_success = false,
        }

        (parse_success, value)
    }
    pub fn IsNotEmptyOrZero(pkg:String) -> (bool, i32) {
        let (parse_success, value) = Self::ParseInt(pkg.clone());

        (!pkg.trim().is_empty() && parse_success  && value > 0, value)
    }

    /*pub fn ProcessHeader2(package:String) -> Self {
        let hdr: Header = bincode::deserialize(&package).unwrap();

        hdr
    }*/

    pub fn ProcessHeader(package:String) -> Self {
        let mut hdr: Header_t = Default::default();
        let mut parse_success:bool;
        let mut flag:bool;
        let mut value:i32;

        hdr.Length = package.substring(0, 4).parse::<i32>().expect("Failed parsing integer value");
        hdr.Mid = package.substring(4, 8).parse::<i32>().expect("Failed parsing integer value");
        (flag, value) = Self::IsNotEmptyOrZero(package.substring(8, 11).to_string());
        hdr.Revision = if flag {value} else {1};
        hdr.NoAckFlag = !package.substring(11, 12).trim().is_empty(); /* not IsNullOrWhiteSpace */
        (parse_success, hdr.StationId) = Self::ParseInt(package.substring(12, 14).to_string());
        if !parse_success { 
            hdr.StationId =1;
        }
        (parse_success, hdr.SpindleId) = Self::ParseInt(package.substring(14, 16).to_string());
        if !parse_success { 
            hdr.SpindleId =1;
        }

        (flag, value) = Self::IsNotEmptyOrZero(package.substring(16, 18).to_string());
        hdr.SequenceNumber = if flag {value} else {Default::default()};
        (flag, value) = Self::IsNotEmptyOrZero(package.substring(18, 19).to_string());
        hdr.NumberOfMessages = if flag {value} else {Default::default()};
        (flag, value) = Self::IsNotEmptyOrZero(package.substring(19, 20).to_string());
        hdr.MessageNumber = if flag {value} else {Default::default()};

        hdr
    }

    pub fn ToString(&mut self) -> String {
        let mut strVal = String::new();

        /*strVal = self.Length.to_string() + &self.Mid.to_string() + &self.Revision.to_string() + &self.StandardizedRevision.to_string();
        match self.NoAckFlag {
            true => {
                //strVal += "1".to_string();
                strVal += "1";
            } 
            false => {
                //strVal += " ".to_string();
                strVal += " ";
            }
        }
        strVal += &self.StationId.to_string() + &self.SpindleId.to_string() + &self.SequenceNumber.to_string();
        strVal += &self.NumberOfMessages.to_string() + &self.MessageNumber.to_string();
        */

        let mut NoAckFlag_str:String;

        match self.NoAckFlag {
            true => {
                NoAckFlag_str = "1".to_string();
            } 
            false => {
                NoAckFlag_str = " ".to_string();
            }
        }
        strVal = format!("{}{}{}{}{}{}{}{}{}{}", self.Length, self.Mid, self.Revision, self.StandardizedRevision,
                                    NoAckFlag_str, self.StationId, self.SpindleId, self.SequenceNumber,
                                    self.NumberOfMessages, self.MessageNumber);
        
        strVal
    }
}