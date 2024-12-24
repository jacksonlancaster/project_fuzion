/// <summary>
/// Controller reboot request 
/// <para>This message causes the controller to reboot after it has accepted the command.
///     <list type="bullet">
///         <item>Warning 1: this MID requires programming control (see 4.4 Programming control).</item>
///         <item>Warning 2: the connection will be lost and will need to be reestablished after controller reboot!</item>
///     </list>
/// </para>    
/// <para>Message sent by: Integrator</para>
/// <para>Answer: <see cref="Communication.Mid0005"/> Command accepted or <see cref="Communication.Mid0004"/> Command error, Programming control not granted</para>
/// </summary>    

use crate::OpenProtocolInterpreter::MID::MidT;
use crate::OpenProtocolInterpreter::Enums;
use crate::OpenProtocolInterpreter::Header::{self, HeaderT};
use crate::OpenProtocolInterpreter::Interfaces::{self, MidGeneric};

#[derive(Clone)]
pub struct Mid0270T { //Mid, IApplicationController, IIntegrator, IAcceptableCommand, IDeclinableCommand 
    pub mid:MidT,
}

impl MidGeneric for Mid0270T {
    fn transform(&self) -> Box<dyn MidGeneric> {
        Box::new(Mid0270T::new()) // Return a new instance
    }
}

impl Interfaces::IDeclinableCommand for Mid0270T {
    fn documented_possible_errors(&self) -> Box<dyn Iterator<Item = Enums::Error> + '_> {
        Box::new([Enums::Error::ProgrammingControlNotGranted].into_iter())
    }
}

impl Mid0270T {
    pub const MID:i32 = 270;

    pub fn new()->Self {
        let hdr1 = HeaderT{mid:Self::MID, revision:Header::DEFAULT_REVISION, ..Default::default()};
        Self::new_header(hdr1)
    }

    pub fn new_header(header:HeaderT)->Self {
        Mid0270T { mid: MidT::new(header) }
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