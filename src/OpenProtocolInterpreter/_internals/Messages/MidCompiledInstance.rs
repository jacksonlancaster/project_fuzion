use std::{any::TypeId, sync::Arc};
use crate::OpenProtocolInterpreter::Interfaces::MidGeneric;

#[derive(Clone)]
pub struct MidCompiledInstanceT {
    pub mci_type:TypeId,
    pub compiled_constructor: Arc<dyn Fn() -> Box<dyn MidGeneric>>,
}

impl PartialEq for MidCompiledInstanceT {
    fn eq(&self, other: &Self) -> bool {
        self.mci_type == other.mci_type // && self.compiled_constructor == other.compiled_constructor
    }
}

impl MidCompiledInstanceT {

    //set
    pub fn set_constructor<F>(&mut self, constructor: F)
    where
        F: Fn() -> Box<dyn MidGeneric> + 'static,
    {
        self.compiled_constructor = Arc::new(constructor);
    }

    //get
    pub fn get_compiled_constructor(&self) -> Arc<dyn Fn() -> Box<dyn MidGeneric>> {
        self.compiled_constructor.clone()
    }

    pub fn invoke_compiled_constructor(&self) -> Box<dyn MidGeneric> {
            (self.compiled_constructor)()
    }

    pub fn new<T: MidGeneric + 'static>()->Self { 
    //pub fn new(type_id:TypeId)->Self { //(type_id:TypeId)->Self {

        let constructor = Arc::new(|| Box::new(T::new()) as Box<dyn MidGeneric>);
        MidCompiledInstanceT{mci_type:TypeId::of::<T>(), compiled_constructor:constructor }
    }

}