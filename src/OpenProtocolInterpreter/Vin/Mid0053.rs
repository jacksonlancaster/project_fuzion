/// <summary>
/// Vehicle ID Number acknowledge
/// <para>Message sent by: Integrator</para>
/// <para>Answer: None</para>
/// </summary>

use crate::OpenProtocolInterpreter::MID::MidT;
use crate::OpenProtocolInterpreter::Header::{self, HeaderT};

#[derive(Clone)]
pub struct Mid0053T { //:Mid, IVin, IIntegrator, IAcknowledge
    pub mid:MidT,
}

impl Mid0053T {
    pub const MID:i32 = 53;

    pub fn new()->Self
    {
        Self::new_rev(Header::DEFAULT_REVISION)
    }

    pub fn new_header(header:HeaderT)->Self
    {
        Mid0053T { mid: MidT::new(header) }
    }

    pub fn new_rev(revision:i32) ->Self {
        let mut h:HeaderT = HeaderT::default();
        h.mid = Self::MID;
        h.revision = revision;
        Self::new_header(h)
    }
}