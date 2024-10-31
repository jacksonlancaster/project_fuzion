use std::any::type_name;
use std::hash::{DefaultHasher, Hash, Hasher};

pub fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

pub fn IsNullOrWhiteSpace(pstr:String) -> bool {
    pstr.trim().is_empty() /* Is Null Or White Space */
}

pub fn GetHashCode<T>(obj:T) -> i32 
where
    T: Hash,
{

    let mut hasher = DefaultHasher::new();
    obj.hash(&mut hasher);
    hasher.finish() as i32
}