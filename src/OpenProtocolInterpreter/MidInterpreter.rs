use std::any::TypeId;
use std::collections::{BTreeMap, HashMap};
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;
use substring::Substring;
use crate::OpenProtocolInterpreter::MID::MidT;
use super::Interfaces::MidGeneric;
use super::KeepAlive::Mid9999::Mid9999T;
use super::_internals::Messages::IMessagesTemplate::IMessagesTemplateI;
use super::_internals::Messages::MessagesTemplate::MessagesTemplateT;
use super::{ApplicationController, KeepAlive};

/// <summary>
/// Responsible for building and parsing any incoming Mid. 
/// Message templates initialization must be done with <see cref="MidInterpreterMessagesExtensions"/> methods.
/// </summary>

pub struct MidInterpreterT {
    //messages_templates: HashMap<TypeId, Lazy<Arc<dyn IMessagesTemplateI>>>,
    messages_templates: HashMap<TypeId, MessagesTemplateT>,
    //mid_templates: BTreeMap<i32, Arc<dyn IMessagesTemplateI>>,
    mid_templates: BTreeMap<i32, MessagesTemplateT>,
}

impl MidInterpreterT {

    pub fn new()->Self
    {
        MidInterpreterT {
            messages_templates:HashMap::new(),
            mid_templates:BTreeMap::new(),
        }
    }

    pub fn pack(mid:&mut MidT)->String {
        mid.pack()
    }

    pub fn pack_bytes(mid:&mut MidT)->Vec<u8> {
        mid.pack_bytes()
    }

    pub fn parse(&mut self, package:String)->MidT
    {
        let mid:i32 = package.substring(4, 8).parse::<i32>().unwrap();
        let instance = Self::TryParseStandaloneMid(mid);
        if instance != Box::new(Mid9999T::new())  { //Default::default()
            return instance;
        }

        let template = self.get_message_template(mid);
        return template.process_package(mid, package);
    }

    fn get_message_template(&mut self, mid:i32)->MessagesTemplateT {
        let template :&MessagesTemplateT;

        if !self.mid_templates.contains_key(&mid) {
        //.TryGetValue(mid, out IMessagesTemplate template) {
            let lazy = self.messages_templates.values().filter(|x|x.is_assignable_to(mid));
            if lazy == None {
                panic!("Could not found a message parser for mid {}, please register it before using", mid);
            }

            template = lazy.Value;
            self.mid_templates.insert(mid, template);
        } else {
            template = self.mid_templates.get(&mid).unwrap();
        }

        *template
    }

    fn TryParseStandaloneMid(mid:i32)->Box<dyn MidGeneric> {
        match mid {
            KeepAlive::Mid9999::Mid9999T::MID => Box::new(KeepAlive::Mid9999::Mid9999T::new()),
            ApplicationController::Mid0270::Mid0270T::MID => Box::new(ApplicationController::Mid0270::Mid0270T::new()),
            _ => Default::default(), //Right now we are defaulting to Mid9999, is this okay?
        }
    }
}