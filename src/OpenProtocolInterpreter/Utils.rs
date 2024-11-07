use std::any::{type_name, TypeId};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::any::Any;
use std::fmt::Debug;

pub fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

pub fn is_null_or_white_space(pstr:String) -> bool {
    pstr.trim().is_empty() /* Is Null Or White Space */
}

pub fn get_hash_code<T>(obj:T) -> i32 
where
    T: Hash,
{

    let mut hasher = DefaultHasher::new();
    obj.hash(&mut hasher);
    hasher.finish() as i32
}

pub fn to_ascii(bytes:&[u8])-> String {
    let s = match std::str::from_utf8(bytes) {
        Ok(v)=> v.to_string(),
        Err(_er) => "".to_string(),
    };

    s
}

#[macro_export]
    macro_rules! ToBytes {
        ($($x:expr),* ) => {
                $value.as_bytes()
        }
    }

// Define a trait `ClnAny` (Clonable Any) for cloning boxed trait objects
pub trait ClnAny: Any + Debug {
    fn clone_box(&self) -> Box<dyn ClnAny>;
}

// Implement `ClnAny` for any type that implements `Clone + Any`
impl<T> ClnAny for T
where
    T: Any + Clone + Debug + 'static,
{
    fn clone_box(&self) -> Box<dyn ClnAny> {
        Box::new(self.clone())
    }
}

// Define a wrapper struct for `Box<dyn ClnAny>` (Cloanble Box)
#[derive(Debug)]
pub struct ClnBox {
    inner: Box<dyn ClnAny>,
}

// Implement `Clone` for `ClnBox`
impl Clone for ClnBox {
    fn clone(&self) -> Self {
        ClnBox {
            inner: self.inner.clone_box(),
        }
    }
}

// Implement `Default` for `ClnBox`
impl Default for ClnBox {
    fn default() -> Self {
        ClnBox {
            inner: Box::new(String::new()), // Choose a default inner value, e.g., an empty String
        }
    }
}

// Constructor for `ClnBox`
impl ClnBox {
    pub fn new<T>(value: T) -> Self
    where
        T: ClnAny,
    {
        ClnBox {
            inner: Box::new(value),
        }
    }

    pub fn as_any(&self) -> &dyn Any {
        &self.inner
    }

    // Method to check the type of the inner value
    pub fn is<T: Any>(&self) -> bool {
        self.inner.as_ref().type_id() == TypeId::of::<T>()
    }
    
    pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }
}