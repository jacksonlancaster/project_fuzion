const MID:i32 = 1;

use crate::OpenProtocolInterpreter::Header::{self, HeaderT};

#[derive(Default)]
pub struct Mid0001T {
    pub header:HeaderT,
    pub use_keep_alive:u16
}

impl Mid0001T {

        pub fn new() -> Self {
            Self::new3(Header::DEFAULT_REVISION)
        }

        pub fn new2(hdr:HeaderT) -> Self {
            Self{header:hdr, use_keep_alive:Default::default()}
        }

        pub fn new3(revision:i32) -> Self {
            let hdr1 = HeaderT{mid:MID, revision:revision, ..Default::default()};
            Self{header:hdr1, use_keep_alive:Default::default()}
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