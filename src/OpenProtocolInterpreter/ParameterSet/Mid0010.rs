/// <summary>
/// Parameter set ID upload request
/// <para>A request to get the valid parameter set IDs from the controller.</para>
/// <para>Message sent by: Integrator</para>
/// <para>Answer: <see cref="Mid0011"/> Parameter set ID upload reply</para>
/// </summary>

use crate::OpenProtocolInterpreter::MID::MidT;
use crate::OpenProtocolInterpreter::Header::{self, HeaderT};

#[derive(Clone)]
pub struct Mid0010T {
    pub mid:MidT,
}

impl Mid0010T { /* Mid, IParameterSet, IIntegrator, IAnswerableBy<Mid0011> */
    pub const MID:i32 = 10;

    pub fn new()->Self
    {
        Self::new_rev(Header::DEFAULT_REVISION)
    }

    pub fn new_header(header:HeaderT)->Self
    {
        Mid0010T { mid: MidT::new(header) }
    }

    pub fn new_rev(revision:i32) ->Self {
        let mut h:HeaderT = HeaderT::default();
        h.mid = Self::MID;
        h.revision = revision;
        Self::new_header(h)
    }

    pub fn set_header(&mut self, hdr:HeaderT) {
        self.mid.header = hdr
    }
    
    pub fn pack(&mut self)->String {
        self.mid.pack()
    }

    pub fn process_header(&mut self, package:String)->HeaderT {
        self.mid.process_header(package)
    }

}