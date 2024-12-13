/// <summary>
/// Vehicle ID Number subscribe
/// <para>
///     This message is used by the integrator to set a subscription for the current 
///     identifiers of the tightening result.
/// </para>    
/// <para>The tightening result can be stamped with up to four identifiers:</para>
/// <list type="bullet">
///     <item>VIN number (identifier result part 1)</item>
///     <item>Identifier result part 2</item>
///     <item>Identifier result part 3</item>
///     <item>Identifier result part 4</item>
/// </list>
/// <para>
///     The identifiers are received by the controller from several input sources, 
///     for example serial, Ethernet, or field bus.
/// </para>
/// <para>In revision 1 of the <see cref="Mid0052"/> Vehicle ID Number, only the VIN number is transmitted.</para>
/// <para>In revision 2, all four possible identifiers are transmitted.</para>
/// <para>Message sent by: Integrator</para>
/// <para>
///     Answer: <see cref="Communication.Mid0005"/> Command accepted or 
///             <see cref="Communication.Mid0004"/> Command error, VIN subscription already exists
/// </para>
/// </summary>

  
use crate::OpenProtocolInterpreter::MID::MidT;
use crate::OpenProtocolInterpreter::Enums;
use crate::OpenProtocolInterpreter::Header::{self, HeaderT};
use crate::OpenProtocolInterpreter::Interfaces;

#[derive(Clone)]
pub struct Mid0051T { //:Mid, IVin, IIntegrator, ISubscription, IAcceptableCommand, IDeclinableCommand
    pub mid:MidT,
}

impl Interfaces::IDeclinableCommand for Mid0051T {
    fn documented_possible_errors(&self) -> Box<dyn Iterator<Item = Enums::Error> + '_> {
        Box::new([Enums::Error::VINUploadSubscriptionAlreadyExists].into_iter())
    }
}

impl Mid0051T {
    pub const MID:i32 = 51;

        pub fn new()->Self
        {
            Self::new_rev_no_ack_flag(Header::DEFAULT_REVISION, None)
        }
    
        pub fn new_header(header:HeaderT)->Self
        {
            Mid0051T{ mid: MidT::new(header) }
        }
    
        pub fn new_rev_no_ack_flag(revision:i32, no_ack_flag:Option<bool>) ->Self {
            let mut h:HeaderT = HeaderT::default();
            h.mid = Self::MID;
            h.revision = revision;
            h.no_ack_flag = no_ack_flag.unwrap_or(false);
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