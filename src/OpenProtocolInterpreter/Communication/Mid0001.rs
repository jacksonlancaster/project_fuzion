const MID:i32 = 1;

use crate::OpenProtocolInterpreter::Header::{self, Header_t};

#[derive(Default)]
pub struct Mid0001_t {
    pub Header:Header_t,
    pub UseKeepAlive:u16
}

impl Mid0001_t {

        pub fn new() -> Self {
            Self::new3(Header::DEFAULT_REVISION)
        }

        pub fn new2(hdr:Header_t) -> Self {
            Self{Header:hdr, UseKeepAlive:Default::default()}
        }

        pub fn new3(revision:i32) -> Self {
            let mut hdr1 = Header_t{Mid:MID, Revision:revision, ..Default::default()};
            Self{Header:hdr1, UseKeepAlive:Default::default()}
        }

        //TBD
        /*(protected override Dictionary<int, List<DataField>> RegisterDatafields()
        {
            return new Dictionary<int, List<DataField>>()
            {
                {
                    7, new List<DataField>()
                            {
                                DataField.Boolean(DataFields.UseKeepAlive, 20)
                            }
                }
            };
        }*/

}