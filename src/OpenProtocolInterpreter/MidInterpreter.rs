use std::any::{Any, TypeId, type_name};
use std::borrow::Borrow;
use std::collections::{BTreeMap, HashMap};
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;
use substring::Substring;
use crate::OpenProtocolInterpreter::MID::MidT;
use super::Interfaces::MidGeneric;
use super::KeepAlive::Mid9999::Mid9999T;
use super::Utils::type_of;
use super::_internals::Messages::IMessagesTemplate::{self, IMessagesTemplateI};
use super::_internals::Messages::MessagesTemplate::MessagesTemplateT;
use super::{ApplicationController, Enums, KeepAlive, Utils};

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

    pub fn parse(&mut self, package:String)->Box<dyn MidGeneric>
    {
        let mid:i32 = package.substring(4, 8).parse::<i32>().unwrap();
        let instance = Self::try_parse_standalone_mid(mid);
        if instance.is_default() {
            return instance;
        }

        let template = self.get_message_template(mid);
        template.process_package(mid, package)
    }

    pub fn parse_from_bytes(&mut self, package:Vec<u8>)->Box<dyn MidGeneric>
    {
        let mid = Utils::bytes_to_int(package[4..8].to_vec());
        let instance = Self::try_parse_standalone_mid(mid);
        if !instance.is_default() {
            return instance;
        }

        let template = self.get_message_template(mid);
        template.process_package2(mid, package)
    }

    pub fn  parse_type_from_string<ExpectedMid>(&mut self, package:String)->ExpectedMid
    where ExpectedMid : MidGeneric + 'static,
    {
        let mid: Box<dyn MidGeneric> = self.parse(package);
        /*if mid.get_type() == TypeId::of::<ExpectedMid>() {
            //let Ok(expected_mid) = mid;
            return *mid; // as ExpectedMid;
        }*/

       // panic!("Package is Mid {}, cannot be casted to {}", mid.get_type_name(), type_name::<ExpectedMid>());
        
        // Downcast to the expected type
        if let Ok(expected_mid) = mid.downcast::<ExpectedMid>() {
            *expected_mid
        } else {
            panic!(
                "Package is Mid {}, cannot be casted to {}",
                mid.get_type_name(),
                type_name::<ExpectedMid>()
            );
        }
    }
    
    pub fn parse_type_from_bytes<ExpectedMid>(&mut self, package:Vec<u8>)->ExpectedMid
    where ExpectedMid : MidGeneric
    {
        let mid = self.parse_from_bytes(package);
        /*if mid.get_type() == TypeId::of::<ExpectedMid>() {
            return mid as ExpectedMid;
        }

        panic!("Package is Mid {}, cannot be casted to {}", mid.get_type_name(), type_name::<ExpectedMid>());
        */
        // Downcast to the expected type
        if let Ok(expected_mid) = mid.downcast::<ExpectedMid>() {
            *expected_mid
        } else {
            panic!(
                "Package is Mid {}, cannot be casted to {}",
                mid.get_type_name(),
                type_name::<ExpectedMid>()
            );
        }
    }
    
    fn use_template(&mut self, template:MessagesTemplateT)
    {
        let t_type = template.get_type_id();
        if !self.messages_templates.contains_key(&t_type) {
            self.messages_templates.insert(t_type, template);
        }
    }

    fn use_template_with_type(&mut self, t_type:TypeId, template:MessagesTemplateT) {
        if !self.messages_templates.contains_key(&t_type) {
            self.messages_templates.insert(t_type, template);
        }
    }

    fn use_template_default<T>(&mut self) 
        where T : IMessagesTemplateI
    {
        self.use_template_with_mode::<T>(Enums::InterpreterMode::Both);
    }

    fn use_template_with_mode<T>(&mut self, mode:Enums::InterpreterMode) 
    where T : IMessagesTemplateI
    {
        let t_type = TypeId::of::<T>();
        let instance = new Lazy<IMessagesTemplate>(() => (IMessagesTemplate)Activator.CreateInstance(type, [mode]));
        self.use_template_with_type(t_type, instance);
    }

    fn use_template_with_type_list<T>(&mut self, types:Vec<TypeId>)
    where T : IMessagesTemplateI
    {
        if !types.is_empty() {
            let t_type = TypeId::of::<T>();
            let instance = new Lazy<IMessagesTemplate>(() => (IMessagesTemplate)Activator.CreateInstance(type, [types]));
            self.use_template_with_type(t_type, instance);
        }
    }

    pub fn use_temuse_template_with_type_hashmap<T, T2>(&mut self, types:HashMap<i32, TypeId>)
    where
        T: IMessagesTemplateI + Default + 'static,
        T2: MidGeneric,
    {
        if !types.is_empty() {
            let type_id = TypeId::of::<T>();
            let instance = self.messages_templates.get(&type_id);
            if instance.is_none() {
                //instance = new Lazy<IMessagesTemplate>(() => (IMessagesTemplate)Activator.CreateInstance(type, []));
                let instance2 = MessagesTemplateT::new();// Arc::new(T::default()) as MessagesTemplateT;
                //let instance_cloned = instance.clone();//instance2.borrow();
                //instance = Some(&instance2_cloned);
                self.use_template_with_type(type_id, instance2.clone());
                instance2.add_or_update_template::<T2>(types);
            } else {
            // Update the template with the provided types
                instance.unwrap().add_or_update_template::<T2>(types);
            }
        }
    }

    fn get_message_template(&mut self, mid:i32)->MessagesTemplateT {
        let mut template :&MessagesTemplateT = &MessagesTemplateT::new();

        if !self.mid_templates.contains_key(&mid) {
            let lazy = self.messages_templates.values().filter(|x|x.is_assignable_to(mid));
            if lazy.clone().count() == 0 {
                panic!("Could not found a message parser for mid {}, please register it before using", mid);
            }

            for temp in lazy {
                template = temp;
                self.mid_templates.insert(mid, template.clone());
            }
        } else {
            template = self.mid_templates.get(&mid).unwrap();
        }

        template.clone()
    }

    fn try_parse_standalone_mid(mid:i32)->Box<dyn MidGeneric> {
        match mid {
            KeepAlive::Mid9999::Mid9999T::MID => Box::new(KeepAlive::Mid9999::Mid9999T::new()),
            ApplicationController::Mid0270::Mid0270T::MID => Box::new(ApplicationController::Mid0270::Mid0270T::new()),
            _ => Default::default(), //Right now we are defaulting to Mid9999, is this okay?
        }
    }
}