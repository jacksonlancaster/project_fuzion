
use std::borrow::Borrow;
use std::default;
use std::hash::{DefaultHasher, Hash, Hasher};

use crate::OpenProtocolInterpreter::Enums::PaddingOrientation;
use crate::OpenProtocolInterpreter::Utils;
use crate::OpenProtocolInterpreter::OpenProtocolConvert::OpenProtocolConvertT;
use std::any::type_name;
use std::clone::Clone;
use std::any::Any;

#[derive(Default)]
pub struct DataFieldT {
    _padding_char:char,
    _padding_orientation:PaddingOrientation,
    cached_value:Option<Box<dyn Any>>,
    default:Box<DataFieldT>, 
    pub has_prefix:bool,
    pub field:i32,
    pub index:i32,
    pub size:i32,
    pub value: String,
    pub raw_value:Vec<u8>,
    pub total_size:i32
}

impl DataFieldT {
    const  DF_BOX_NONE: Option<Box<DataFieldT>> = None;

    fn set_default(&mut self) {
 
        self.default = Box::new(Self {
            _padding_char:' ',
            _padding_orientation:PaddingOrientation::RightPadded,
            cached_value:None,
            default:DataFieldT::DF_BOX_NONE.unwrap(),
            has_prefix:true,
            field : -1,
            index : -1,
            size:-1,
            value : "".to_string(),
            raw_value:vec![],
            total_size:0
        });

        self.default.cal_total_size();
    }

    fn cal_total_size(&mut self) {
        self.total_size = if self.has_prefix {2 + self.size} else {self.size};
    }

    pub fn new(field:i32, index:i32, size:i32, hasPrefix:Option<bool>) -> Self {
            let mut df = Self {
                _padding_char:' ',
                _padding_orientation:PaddingOrientation::RightPadded,
                cached_value:None,
                default:DataFieldT::DF_BOX_NONE.unwrap(),
                has_prefix:hasPrefix.unwrap_or(true),
                field,
                index,
                size,
                value : "".to_string(),
                raw_value:vec![],
                total_size:0
            };
            df.cal_total_size();
            df.set_default();

            df

    }

    pub fn new2(&mut self, field:i32 /*enum*/, index:i32, size:i32, hasPrefix:Option<bool>) -> Self {

        let mut df = Self {
            _padding_char:' ',
            _padding_orientation:PaddingOrientation::RightPadded,
            cached_value:None,
            default:DataFieldT::DF_BOX_NONE.unwrap(),
            has_prefix:hasPrefix.unwrap_or(true),
            field : Utils::get_hash_code(field),
            index,
            size,
            value : "".to_string(),
            raw_value:vec![],
            total_size:0
        };
        df.cal_total_size();
        df.set_default();

        df
    }

        pub fn new3(field:i32, index:i32, size:i32, paddingChar:char, paddingOrientation:Option<PaddingOrientation>,  hasPrefix:Option<bool>) -> Self {

            let mut df = Self {
                _padding_char:paddingChar,
                _padding_orientation:paddingOrientation.unwrap_or(PaddingOrientation::RightPadded),
                cached_value:None,
                default:DataFieldT::DF_BOX_NONE.unwrap(),
                has_prefix:hasPrefix.unwrap_or(true),
                field, 
                index,
                size,
                value : "".to_string(),
                raw_value:vec![],
                total_size:0
            };
            df.cal_total_size();
            df.set_default();
    
            df
        }

        pub fn new4(field:i32 /*enum*/, index:i32, size:i32, paddingChar:char, paddingOrientation:Option<PaddingOrientation>,  hasPrefix:Option<bool>) -> Self {

            let mut df = Self {
                _padding_char:paddingChar,
                _padding_orientation:paddingOrientation.unwrap_or(PaddingOrientation::RightPadded),
                cached_value:None,
                default:DataFieldT::DF_BOX_NONE.unwrap(),
                has_prefix:hasPrefix.unwrap_or(true),
                field : Utils::get_hash_code(field),
                index,
                size,
                value : "".to_string(),
                raw_value:vec![],
                total_size:0
            };
            df.cal_total_size();
            df.set_default();
    
            df
        }

        pub fn get_value<T: 'static + Clone>(&mut self, converter:fn(String)->T) -> T {

            if Utils::is_null_or_white_space(self.value.to_string()) {
                self.cached_value = None; //TBD- T::default() ?
            } else {
                if self.is_value_not_cached::<T>() {
                    self.cached_value = Some(Box::new(converter(self.value.to_string())));
                }
            }

            self.cached_value
                .as_ref()
                .and_then(|boxed| boxed.downcast_ref::<T>())
                .cloned()
                .expect("Failed to retrieve Cached Value")

        }

        pub fn get_value2<T: 'static + Clone>(&mut self, converter:fn(Vec<u8>)->T) -> T {

            if self.raw_value.is_empty() || self.raw_value.is_empty() { //TBD for 1st one- == self.default ?
                self.cached_value = None; //TBD-  T::default() ?
            } else {
                if self.is_value_not_cached::<T>() {
                    self.cached_value = Some(Box::new(converter(self.raw_value.clone())));
                }
            }

            self.cached_value
            .as_ref()
            .and_then(|boxed| boxed.downcast_ref::<T>())
            .cloned()
            .expect("Failed to retrieve Cached Value")
        }

        pub fn set_value2<U>(&mut self, converter:fn(char, i32, PaddingOrientation, U)->String, value:U) {
            self.cached_value = None;
            self.value = converter(self._padding_char, self.size, self._padding_orientation, value);
            self.size = self.value.len() as i32;
        }

        pub fn set_raw_value<T>(&mut self, converter: fn(char, i32, PaddingOrientation, T) -> Vec<u8>, value: T) {
            self.cached_value = None;
        
            self.raw_value  = converter(self._padding_char, self.size, self._padding_orientation, value);
        
            self.size = self.raw_value.len() as i32;
        }
        
        pub fn string4(field:i32, index:i32, size:i32, hasPrefix:Option<bool>) -> Self {
            DataFieldT::new3(field, index, size, ' ', Some(PaddingOrientation::RightPadded), hasPrefix)
        }

        pub fn string3(field:i32, index:i32, size:i32, paddingOrientation:PaddingOrientation, hasPrefix:Option<bool>)->Self {
            DataFieldT::new3(field, index, size, ' ', Some(paddingOrientation), hasPrefix)
        }
        pub fn string2(field:i32 /*Enum*/, index:i32, size:i32, hasPrefix:Option<bool>)  -> Self {
            DataFieldT::new3(field, index, size, ' ', Some(PaddingOrientation::RightPadded), hasPrefix)
        }
        pub fn string(field:i32 /*Enum*/, index:i32, size:i32, paddingOrientation:PaddingOrientation, hasPrefix:Option<bool>) -> Self {
            DataFieldT::new3(field, index, size, ' ', Some(paddingOrientation), hasPrefix)
        }

        pub fn boolean2(field:i32, index:i32, hasPrefix:Option<bool>) -> Self {
            DataFieldT::new(field, index, 1, hasPrefix)
        }

        pub fn boolean(field:i32 /*Enum */, index:i32, hasPrefix:Option<bool>) -> Self {
            DataFieldT::new(field, index, 1, hasPrefix)
        }

        pub fn timestamp2(field:i32, index:i32, hasPrefix:Option<bool>) -> Self {
            DataFieldT::new(field, index, 19, hasPrefix)
        }

        pub fn timestamp(field:i32 /* Enum */, index:i32, hasPrefix:Option<bool>) -> Self {
            DataFieldT::new(field, index, 19, hasPrefix)
        }

        pub fn number2(field:i32, index:i32, size:i32, hasPrefix:Option<bool>) -> Self {
            DataFieldT::new4(field, index, size, '0', Some(PaddingOrientation::LeftPadded), hasPrefix)
        }

        pub fn number(field:i32 /*Enum */, index:i32, size:i32, hasPrefix:Option<bool>) -> Self {
            DataFieldT::new4(field, index, size, '0', Some(PaddingOrientation::LeftPadded), hasPrefix)
        }

        pub fn volatile4(field:i32, index:i32, hasPrefix: Option<bool> ) -> Self {
            DataFieldT::new(field, index, 0, hasPrefix)
        }

        pub fn volatile3(field: i32 /*Enum */, index: i32, hasPrefix :Option<bool>) -> Self {
            DataFieldT::new(field, index, 0, hasPrefix)
        }

        pub fn volatile2(field: i32, hasPrefix:Option<bool>) -> Self {
            DataFieldT::new(field, 0, 0, hasPrefix)
        }
        pub fn volatile(field: i32 /*Enum */, hasPrefix:Option<bool>) -> Self {
            DataFieldT::new(field, 0, 0, hasPrefix)
        }

        pub fn set_value(mut self, value:String)
        {
            self.cached_value = None;
            self.set_value2(OpenProtocolConvertT::truncate_padded, value);
        }

        fn is_value_not_cached<T: 'static>(&self) -> bool {
            self.cached_value.is_none() || self.is_not_type_of::<T>()
        }

        fn is_not_type_of<T: 'static>(&self) -> bool {

            let cv = self
            .cached_value
            .as_ref()
            .map(|boxed| boxed.is::<T>())
            .unwrap_or(false);

            Utils::type_of(cv) != type_name::<T>()
        }

}