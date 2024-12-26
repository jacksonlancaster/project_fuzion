use std::any::{Any, TypeId, type_name};
use crate::OpenProtocolInterpreter::Enums::Error;
use crate::OpenProtocolInterpreter::MID::MidT;

use super::KeepAlive::Mid9999::Mid9999T;

pub trait MidGeneric: Any {
    fn new() -> Self
    where
        Self: Sized;
    fn parse2(&mut self, package: String) -> Self
    where
        Self: Sized;
    
    fn parse(&mut self, package:&[u8]) -> Self
    where
        Self: Sized;
    fn is_default(&self)->bool;
    fn get_type(&self)->TypeId;
    fn get_type_name(&self)->String;
    /*fn downcast<T>(&self)->T
    where 
        T:Sized;*/
    fn transform(&self) -> Box<dyn MidGeneric>;
}

/*
impl<T: Any> MidGeneric for T {
    fn get_type(&self) -> TypeId {
        TypeId::of::<T>()
    }

    fn get_type_name(&self) -> String {
        type_name::<T>().to_string()
    }
}*/

impl Default for Box<dyn MidGeneric> {
    fn default() -> Self {
        Box::new(Mid9999T::new()) // Default to Mid9999
    }
}

/// Contract which every integrator <see cref="Mid"/> message implements.
pub trait IIntegrator {
    
}

/// <summary>
/// Contract which every controller <see cref="Mid"/> message implements.
/// </summary>
pub trait IController
{
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