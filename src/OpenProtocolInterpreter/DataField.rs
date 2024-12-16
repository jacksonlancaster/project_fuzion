use crate::OpenProtocolInterpreter::Enums::PaddingOrientation;
use crate::OpenProtocolInterpreter::Utils;
use crate::OpenProtocolInterpreter::OpenProtocolConvert::OpenProtocolConvertT;
use std::any::type_name;
use std::clone::Clone;

#[derive(Clone)]
pub struct DataFieldT {
    _padding_char:char,
    _padding_orientation:PaddingOrientation,
    cached_value:Option<Utils::ClnBox>,
    default:Option<Box<DataFieldT>>, 
    pub has_prefix:bool,
    pub field:i32,
    pub index:i32,
    pub size:i32,
    pub value: String,
    pub raw_value:Vec<u8>,
    pub total_size:i32
}

impl Default for DataFieldT {
    fn default() -> Self {
        DataFieldT {
            _padding_char:' ',
            _padding_orientation:PaddingOrientation::RightPadded,
            cached_value:None,
            default:None,
            has_prefix:true,
            field : -1,
            index : -1,
            size:-1,
            value : "".to_string(),
            raw_value:vec![],
            total_size:0
        }
    }
}

impl DataFieldT {
    const  DF_BOX_NONE: Option<Box<DataFieldT>> = None;

    fn set_default(&mut self) {
 
        let mut bx_df = Box::new(Self {
            _padding_char:' ',
            _padding_orientation:PaddingOrientation::RightPadded,
            cached_value:None,
            default:None,
            has_prefix:true,
            field : -1,
            index : -1,
            size:-1,
            value : "".to_string(),
            raw_value:vec![],
            total_size:0
        });

        bx_df.cal_total_size();
        self.default = Some(bx_df);
    }

    fn cal_total_size(&mut self) {
        self.total_size = if self.has_prefix {2 + self.size} else {self.size};
    }

    pub fn new(field:i32, index:i32, size:i32, has_prefix:Option<bool>) -> Self {
            let mut df = Self {
                _padding_char:' ',
                _padding_orientation:PaddingOrientation::RightPadded,
                cached_value:None,
                default:None,
                has_prefix:has_prefix.unwrap_or(true),
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

    pub fn new2(&mut self, field:i32 /*enum*/, index:i32, size:i32, has_prefix:Option<bool>) -> Self {

        let mut df = Self {
            _padding_char:' ',
            _padding_orientation:PaddingOrientation::RightPadded,
            cached_value:None,
            default:None,
            has_prefix:has_prefix.unwrap_or(true),
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

        pub fn new3(field:i32, index:i32, size:i32, padding_char:char, padding_orientation:Option<PaddingOrientation>,  has_prefix:Option<bool>) -> Self {

            let mut df = Self {
                _padding_char:padding_char,
                _padding_orientation:padding_orientation.unwrap_or(PaddingOrientation::RightPadded),
                cached_value:None,
                default:None,
                has_prefix:has_prefix.unwrap_or(true),
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

        pub fn new4(field:i32 /*enum*/, index:i32, size:i32, padding_char:char, padding_orientation:Option<PaddingOrientation>,  has_prefix:Option<bool>) -> Self {

            let mut df = Self {
                _padding_char:padding_char,
                _padding_orientation:padding_orientation.unwrap_or(PaddingOrientation::RightPadded),
                cached_value:None,
                default:None,
                has_prefix:has_prefix.unwrap_or(true),
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

        pub fn get_value<T: 'static + Clone>(&mut self, converter:fn(String)->T) -> T 
        where 
            T:'static + Utils::ClnAny + Clone + Default,
        
        {

            if Utils::is_null_or_white_space(self.value.to_string()) {
                self.cached_value = None;
            } else {
                if self.is_value_not_cached::<T>() {
                    self.cached_value = Some(Utils::ClnBox::new(converter(self.value.to_string())));
                }
            }

            let ret_val = self.cached_value
                .as_ref()
                .and_then(|boxed| boxed.downcast_ref::<T>())
                .cloned();

            
            ret_val.unwrap_or_default()
        }

        pub fn get_value2<T: 'static + Clone>(&mut self, converter:fn(Vec<u8>)->T) -> T 
        where 
        T:Utils::ClnAny,
        {

            if self.raw_value.is_empty() || self.raw_value.is_empty() { //TBD for 1st one- == self.default ?
                self.cached_value = None; //TBD-  T::default() ?
            } else {
                if self.is_value_not_cached::<T>() {
                    self.cached_value = Some(Utils::ClnBox::new(converter(self.raw_value.clone())));
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
        
        pub fn string4(field:i32, index:i32, size:i32, has_prefix:Option<bool>) -> Self {
            DataFieldT::new3(field, index, size, ' ', Some(PaddingOrientation::RightPadded), has_prefix)
        }

        pub fn string3(field:i32, index:i32, size:i32, padding_orientation:PaddingOrientation, has_prefix:Option<bool>)->Self {
            DataFieldT::new3(field, index, size, ' ', Some(padding_orientation), has_prefix)
        }
        pub fn string2(field:i32 /*Enum*/, index:i32, size:i32, has_prefix:Option<bool>)  -> Self {
            DataFieldT::new3(field, index, size, ' ', Some(PaddingOrientation::RightPadded), has_prefix)
        }
        pub fn string(field:i32 /*Enum*/, index:i32, size:i32, padding_orientation:PaddingOrientation, has_prefix:Option<bool>) -> Self {
            DataFieldT::new3(field, index, size, ' ', Some(padding_orientation), has_prefix)
        }

        pub fn boolean2(field:i32, index:i32, has_prefix:Option<bool>) -> Self {
            DataFieldT::new(field, index, 1, has_prefix)
        }

        pub fn boolean(field:i32 /*Enum */, index:i32, has_prefix:Option<bool>) -> Self {
            DataFieldT::new(field, index, 1, has_prefix)
        }

        pub fn timestamp2(field:i32, index:i32, has_prefix:Option<bool>) -> Self {
            DataFieldT::new(field, index, 19, has_prefix)
        }

        pub fn timestamp(field:i32 /* Enum */, index:i32, has_prefix:Option<bool>) -> Self {
            DataFieldT::new(field, index, 19, has_prefix)
        }

        pub fn number2(field:i32, index:i32, size:i32, has_prefix:Option<bool>) -> Self {
            DataFieldT::new4(field, index, size, '0', Some(PaddingOrientation::LeftPadded), has_prefix)
        }

        pub fn number(field:i32 /*Enum */, index:i32, size:i32, has_prefix:Option<bool>) -> Self {
            DataFieldT::new4(field, index, size, '0', Some(PaddingOrientation::LeftPadded), has_prefix)
        }

        pub fn volatile4(field:i32, index:i32, has_prefix: Option<bool> ) -> Self {
            DataFieldT::new(field, index, 0, has_prefix)
        }

        pub fn volatile3(field: i32 /*Enum */, index: i32, has_prefix :Option<bool>) -> Self {
            DataFieldT::new(field, index, 0, has_prefix)
        }

        pub fn volatile2(field: i32, has_prefix:Option<bool>) -> Self {
            DataFieldT::new(field, 0, 0, has_prefix)
        }
        pub fn volatile(field: i32 /*Enum */, has_prefix:Option<bool>) -> Self {
            DataFieldT::new(field, 0, 0, has_prefix)
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