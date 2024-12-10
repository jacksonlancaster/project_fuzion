    /// <summary>
    /// Keep alive message
    /// <para>
    ///   The integrator sends a keep alive to the controller. The controller should only mirror and return the 
    ///   received keep alive to the integrator.
    /// </para>
    /// <para>
    ///   The controller has a communication timeout equal to 15s. This means that if no message has been 
    ///   exchanged between the integrator and the controller for the last 15s, then the controller considers 
    ///   the connection lost and closes it;
    /// </para>
    /// <para>
    ///   In order to keep the communication alive the integrator must send a keep alive to the controller with a
    ///   time interval lower than 15s.
    /// </para>
    /// <para>
    ///   Note: An inactivity timeout is suggested to integrator i.e. if no message has been exchanged (sent or 
    ///   received) during the last 10s, send a keep alive.
    /// </para>  
    /// <para>Message sent by: Integrator</para>
    /// <para>Answer: The same message (<see cref="Mid9999"/>) mirrored by the controller.</para>
    /// </summary>

use crate::OpenProtocolInterpreter::Header::HeaderT;
use crate::OpenProtocolInterpreter::MID::MidT;

#[derive(Default, Clone)]
pub struct Mid9999T { //:Mid, IIntegrator, IController
    pub mid:MidT,
}

impl Mid9999T {

        pub(crate) const DEFAULT_REVISION:i32 = 0;
        pub const MID:i32 = 9999;

        /*The following 3 methods are Common Methods to all MIDs  */
        pub fn new() -> Self {
            Self::new_rev(Self::DEFAULT_REVISION)
        }

        pub fn new_header(hdr:HeaderT) -> Self {
            Self{mid:MidT::new(hdr)}
        }

        pub fn new_rev(revision:i32) ->Self {
            let mut h:HeaderT = HeaderT::default();
            h.mid = Self::MID;
            h.revision = revision;
            Self::new_header(h)
        }

        pub fn pack(&mut self)->String {
            self.mid.pack()
        }

        pub fn process_header(&mut self, package:String)->HeaderT {
            self.mid.process_header(package)
        }

    }