/// <summary>
/// Parameter set selected subscribe
/// <para>
///     A subscription for the parameter set selection. Each time a new parameter set is selected the <see cref="Mid0015"/>
///     Parameter set selected is sent to the integrator.
/// </para>
/// <para>
///     Note that the immediate response is <see cref="Communication.Mid0005"/> Command
///     accepted and <see cref="Mid0015"/> Parameter set selected with the current parameter set number selected.
/// </para>
/// <para>Message sent by: Integrator</para>
/// <para>Answer: <see cref="Communication.Mid0005"/> Command accepted and <see cref="Mid0015"/> Parameter set selected</para>
/// </summary>

use crate::OpenProtocolInterpreter::Header::{self, HeaderT};
use crate::OpenProtocolInterpreter::MID::MidT;

#[derive(Default, Clone)]
pub struct Mid0014T { //:Mid, IParameterSet, IController
    pub mid:MidT,
}

impl Mid0014T {

        pub const MID:i32 = 14;

        /*The following 4 methods are Common Methods to all MIDs  */
        pub fn new() -> Self {
            Self::new_no_ack_flag(false)
        }

        pub fn new_header(hdr:HeaderT) -> Self {
            Self{mid:MidT::new(hdr)}
        }

        fn new_no_ack_flag(p_no_ack_flag:bool /*= false*/) -> Self {
            let hdr1 = HeaderT{mid:Self::MID, revision:Header::DEFAULT_REVISION, 
                no_ack_flag:p_no_ack_flag, ..Default::default()};
            Self::new_header(hdr1)
        }

        pub fn pack(&mut self) ->String
        {
            self.mid.pack()
        }
        /* Common methods end here */

}