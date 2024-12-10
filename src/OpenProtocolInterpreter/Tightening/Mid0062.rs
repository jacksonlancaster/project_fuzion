/// <summary>
/// Last tightening result data acknowledge
/// <para>Acknowledgement of last tightening result data.</para>
/// <para>Message sent by: Integrator</para>
/// <para>Answer: None</para>
/// </summary>
use crate::OpenProtocolInterpreter::Header::{self, HeaderT};
use crate::OpenProtocolInterpreter::MID::MidT;

#[derive(Default, Clone)]
pub struct Mid0062T { //:Mid, ITightening, IIntegrator, IAcknowledge
    pub mid:MidT,
}

impl Mid0062T {

        pub const MID:i32 = 62;

        /*The following 3 methods are Common Methods to all MIDs  */
        pub fn new() -> Self {
            Self::new_rev(Header::DEFAULT_REVISION)
        }

        pub fn new_header(hdr:HeaderT) -> Self {
            Self{mid:MidT::new(hdr)}
        }

        pub fn new_rev(revision:i32) -> Self {
            let hdr1 = HeaderT{mid:Self::MID, revision:revision, ..Default::default()};
            Self::new_header(hdr1)
        }

        pub fn pack(&mut self)->String {
            self.mid.pack()
        }

        pub fn process_header(&mut self, package:String)->HeaderT {
            self.mid.process_header(package)
        }

    }