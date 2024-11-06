use std::any::type_name;
use std::hash::{DefaultHasher, Hash, Hasher};

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