use substring::Substring;

/// <summary>
/// Represents a Stage Result entity
/// </summary>

use crate::OpenProtocolInterpreter::{Enums, OpenProtocolConvert::OpenProtocolConvertT};

#[derive(Clone)]
pub struct StageResultT {
    pub torque:f32,
    pub angle:i32,
}

impl StageResultT {
    pub fn pack(&mut self)->String
    {
        OpenProtocolConvertT::tp_truncated_decimal_to_string('0', 6, Enums::PaddingOrientation::LeftPadded, self.torque) +
                OpenProtocolConvertT::tp_i32_to_string('0', 5, Enums::PaddingOrientation::LeftPadded, self.angle).as_str()
    }

    pub fn parse(value: &str) -> StageResultT {
        StageResultT {
            torque: OpenProtocolConvertT::string_to_truncated_decimal(value[..6].to_string()),
            angle: OpenProtocolConvertT::string_to_int32(value[6..11].to_string()),
        }
    }

    /*pub fn parse_all(value: &str) -> impl Iterator<Item = StageResultT> + '_ {
        const SECTION_SIZE: usize = 11;
        value.chars()
            .collect::<Vec<_>>() // Split the string into characters for safe slicing
            .chunks(SECTION_SIZE)
            .map(|chunk| {
                let section: String = chunk.iter().collect();
                StageResultT::parse(&section)
            })
    }*/

    pub fn parse_all(value:String)->Vec<StageResultT> 
    {
        const SECTION_SIZE: usize = 11;
        let mut i = 0;
        let mut vc:Vec<StageResultT>= Vec::new();

        while i < value.len() {
            let section = value.substring(i, i+ SECTION_SIZE);
            vc.push(Self::parse(section));
            i += SECTION_SIZE;
        }
        vc
    }

}