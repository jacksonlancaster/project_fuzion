use std::{any::TypeId, collections::HashMap};

use crate::OpenProtocolInterpreter::Interfaces::MidGeneric;

/// <summary>
/// Templates for parsing packages and validating Mid assignability
/// </summary>

pub trait IMessagesTemplateI {
    fn add_or_update_template<T: MidGeneric + 'static>(&mut self, types:HashMap<i32, TypeId>);
    fn process_package(&self, mid:i32, package:String)->Box<dyn MidGeneric>;
    fn process_package2(&self, mid:i32, package:Vec<u8>)->Box<dyn MidGeneric>;
    fn is_assignable_to(&self, mid:i32)->bool;
}