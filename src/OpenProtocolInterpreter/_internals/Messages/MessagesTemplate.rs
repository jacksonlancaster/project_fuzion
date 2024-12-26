use std::{any::TypeId, cmp::Ordering, collections::HashMap};

use crate::OpenProtocolInterpreter::{Enums, Interfaces::{IController, IIntegrator, MidGeneric}, Utils};

use super::{IMessagesTemplate::IMessagesTemplateI, MidCompiledInstance::MidCompiledInstanceT};

/// <summary>
/// Base class for all <see cref="IMessagesTemplate"/> templates implementers
/// </summary>

#[derive(Clone)]
pub struct MessagesTemplateT { //: IMessagesTemplate
    pub(crate) templates:HashMap<i32, MidCompiledInstanceT>,
}

impl IMessagesTemplateI for MessagesTemplateT {
    fn add_or_update_template<T: MidGeneric + 'static>(&mut self, types:HashMap<i32, TypeId>) {
        for  (k, _v) in types {
            if self.templates.contains_key(&k) {
                self.templates.remove(&k);
            }
            self.templates.insert(k, MidCompiledInstanceT::new::<T>());
        }
    }

    /// <summary>
    /// Find out which Mid instance it should instantiate and parse all it's content.
    /// </summary>
    /// <param name="mid">Mid number.</param>
    /// <param name="package">Package in ASCII string.</param>
    /// <returns><see cref="Mid"/> instance.</returns>
    fn process_package(&self, mid:i32, package:String)->Box<dyn MidGeneric> {
        let compiled_instance = self.get_mid_type(mid);  
        let mid_obj =compiled_instance.invoke_compiled_constructor();
        
        mid_obj.parse2(package)
    }

    /// <summary>
    /// Find out which Mid instance it should instantiate and parse all it's content
    /// </summary>
    /// <param name="mid">Mid number</param>
    /// <param name="package">package in bytes</param>
    /// <returns><see cref="Mid"/> instance</returns>
    fn process_package2(&self, mid:i32, package:Vec<u8>)->Box<dyn MidGeneric> {
        let compiled_instance = self.get_mid_type(mid);  
        let mid_obj =compiled_instance.invoke_compiled_constructor();
        
        mid_obj.parse(package.as_slice())
    }

    fn is_assignable_to(&self, mid:i32)->bool {
        todo!()
    }
}

impl MessagesTemplateT {

        pub fn get_type_id(&self) -> TypeId {
            TypeId::of::<Self>()
        }
        /// <summary>
        /// Initializes a new instance of <see cref="MessagesTemplate"/> class.
        /// </summary>
        pub fn new()->Self
        {
            MessagesTemplateT{templates:HashMap::new()}
        }

        /// <summary>
        /// Filter dictionary to use only Mids from it's mode.
        /// </summary>
        /// <param name="mode">Current mode if <see cref="InterpreterMode.Controller"/>, <see cref="InterpreterMode.Integrator"/> or <see cref="InterpreterMode.Both"/>.</param>
        pub(crate) fn filter_selected_mids(&mut self, mode:Enums::InterpreterMode) {
            if mode == Enums::InterpreterMode::Both {
                return;
            }
            
            let mtype = if mode == Enums::InterpreterMode::Controller {TypeId::of::<dyn IIntegrator>} else {TypeId::of::<dyn IController>};
            let temp = self.templates.clone();
            let selected_mids : Vec<_> = temp
            .values()
            .filter(|x| Utils::is_assignable_from(mtype(), x.mci_type) /*mtype.is_assignable_from(&x.mci_type)*/)
            .collect();
            //self.templates.Values.Where(x => type.IsAssignableFrom(x.Type));
            self.filter_selected_mids_with_compiled_instances(selected_mids);
            
        }

        /// <summary>
        /// Filter dictionary to use only selected Mids.
        /// </summary>
        /// <param name="mids">Selected <see cref="Mid"/> types.</param>
        pub(crate) fn filter_selected_mids_with_types(&mut self, mids:Vec<TypeId>) {
            let temp = self.templates.clone();
            let ignored_mids = temp
            .values()
            .filter(|x| mids.contains(&x.mci_type));
            let mut values:Vec<&MidCompiledInstanceT>=Vec::new();

            for v in ignored_mids {
                values.push(v);
            }
            self.filter_selected_mids_with_compiled_instances(values);
        }

        /// <summary>
        /// Remove unused/ignored <see cref="MidCompiledInstance"/> from dictionary.
        /// </summary>
        /// <param name="mids">Ignored mid instances</param>
        pub(crate) fn filter_selected_mids_with_compiled_instances(&mut self, mids:Vec<&MidCompiledInstanceT>)
        {
            let temp = self.templates.clone();
            let ignored_mids = temp.iter().filter(|x| !mids.contains(&x.1));
            for (k, _) in ignored_mids {
                self.templates.remove(k);
            }
        }

        /// <summary>
        /// Get <see cref="MidCompiledInstance"/> from the dictionary based on mid number
        /// </summary>
        /// <param name="mid">Mid number</param>
        /// <returns>Compiled instance</returns>
        fn  get_mid_type(&mut self, mid:i32)->&MidCompiledInstanceT
        {
            let instance_compiler = self.templates.get(&mid);
            if instance_compiler.is_none() {
                panic!("MID {} was not implemented, please register it!", mid);
            }

            instance_compiler.unwrap()
        }
}