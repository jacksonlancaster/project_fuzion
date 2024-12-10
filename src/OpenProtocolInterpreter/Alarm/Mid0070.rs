/// <summary>
/// Alarm subscribe
/// <para>A subscription for the alarms that can appear in the controller.</para>
/// <para>Message sent by Integrator</para>
/// <para>Answers: <see cref="Communication.Mid0005"/> Command accepted or <see cref="Communication.Mid0004"/> Command error, Alarm subscription already exists</para>
/// </summary>

use crate::OpenProtocolInterpreter::{Enums, Interfaces};
use crate::OpenProtocolInterpreter::Header::{self, HeaderT};
use crate::OpenProtocolInterpreter::MID::MidT;

#[derive(Default, Clone)]
pub struct Mid0070T { //:Mid, IAlarm, IIntegrator, ISubscription, IAcceptableCommand, IDeclinableCommand
    pub mid:MidT,
}

impl Interfaces::IDeclinableCommand for Mid0070T {
    fn documented_possible_errors(&self) -> Box<dyn Iterator<Item = Enums::Error> + '_> {
        Box::new([Enums::Error::AlarmSubscriptionAlreadyExists].into_iter())
    }
}

impl Mid0070T {

        pub const MID:i32 = 70;

        /*The following 3 methods are Common Methods to all MIDs  */
        pub fn new() -> Self {
            Self::new_rev_no_ack_flag(Header::DEFAULT_REVISION, None)
        }

        pub fn new_header(hdr:HeaderT) -> Self {
            Self{mid:MidT::new(hdr)}
        }

        pub fn new_rev_no_ack_flag(revision:i32, no_ack_flag:Option<bool>) ->Self {
            let mut h:HeaderT = HeaderT::default();
            h.mid = Self::MID;
            h.revision = revision;
            h.no_ack_flag = no_ack_flag.unwrap_or(false);
            Self::new_header(h)
        }

        pub fn pack(&mut self)->String {
            self.mid.pack()
        }

        pub fn process_header(&mut self, package:String)->HeaderT {
            self.mid.process_header(package)
        }

    }