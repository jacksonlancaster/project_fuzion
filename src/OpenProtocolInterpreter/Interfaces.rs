//use std::error::Error;
use crate::OpenProtocolInterpreter::Enums::Error;
use crate::OpenProtocolInterpreter::MID::MidT;

pub trait MidGeneric {
    
}

/// Contract which every integrator <see cref="Mid"/> message implements.
pub trait IIntegrator {
    
}

/// Contract of every <see cref="Mid"/> message that can be answered by another mid which is not classified as an acknowledge.
// Define the IAnswerableBy trait with a generic type that must implement Mid
pub trait IAnswerableBy<TAnswerMid: MidGeneric> {
    // Define any methods or associated types for IAnswerableBy here
    fn get_answer_mid(&self) -> TAnswerMid;
}

/// Communication message category. Every communication mid must implement <see cref="ICommunication"/>.
pub trait ICommunication
{

}

/// Contract which every <see cref="Mid"/> message that can be declined with <see cref="Communication.Mid0004"/> implements.
pub trait IDeclinableCommand {
    fn documented_possible_errors(&self) -> Box<dyn Iterator<Item = Error> + '_>;
}

impl MidGeneric for MidT {

}