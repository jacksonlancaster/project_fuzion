use std::any::type_name;

pub fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

pub fn IsNullOrWhiteSpace(pstr:String) -> bool {
    pstr.trim().is_empty() /* Is Null Or White Space */
}
