/// <summary>
/// Last tightening result data subscribe
/// <para>
///     Set the subscription for the result tightenings. The result of this command will be the transmission of
///     the tightening result after the tightening is performed(push function). The MID revision in the header
///     is used to subscribe to different revisions of <see cref="Mid0061"/> Last tightening result data upload reply.
/// </para>
/// <para>Message sent by: Integrator</para>
/// <para>
///     Answer: <see cref="Communication.Mid0005"/> Command accepted or 
///             <see cref="Communication.Mid0004"/> Command error, Last tightening subscription already exists or MID revision not supported
/// </para>
/// </summary>

use crate::OpenProtocolInterpreter::MID::MidT;
use crate::OpenProtocolInterpreter::Enums;
use crate::OpenProtocolInterpreter::Header::{self, HeaderT};
use crate::OpenProtocolInterpreter::Interfaces;

#[derive(Clone)]
pub struct Mid0060T { //:Mid, ITightening, IIntegrator, ISubscription, IAcceptableCommand, IDeclinableCommand
    pub mid:MidT,
}

impl Interfaces::IDeclinableCommand for Mid0060T {
    fn documented_possible_errors(&self) -> Box<dyn Iterator<Item = Enums::Error> + '_> {
        Box::new([Enums::Error::LastTighteningResultSubscriptionAlreadyExists, Enums::Error::MidRevisionUnsupported].into_iter())
    }
}

impl Mid0060T {
    pub const MID:i32 = 60;

        pub fn new()->Self
        {
            Self::new_rev_no_ack_flag(Header::DEFAULT_REVISION, None)
        }
    
        pub fn new_header(header:HeaderT)->Self
        {
            Mid0060T{ mid: MidT::new(header) }
        }
    
        pub fn new_rev_no_ack_flag(revision:i32, no_ack_flag:Option<bool>) ->Self {
            let mut h:HeaderT = HeaderT::default();
            h.mid = Self::MID;
            h.revision = revision;
            h.no_ack_flag = no_ack_flag.unwrap_or(false);
            Self::new_header(h)
        }
}