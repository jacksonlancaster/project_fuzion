
use std::borrow::{Borrow, BorrowMut};
use std::default;
//use std::hash::{DefaultHasher, Hash, Hasher};

use crate::OpenProtocolInterpreter::Enums::PaddingOrientation;
use crate::OpenProtocolInterpreter::Utils;
use crate::OpenProtocolInterpreter::OpenProtocolConvert::OpenProtocolConvert_t;
use std::any::type_name;
use std::clone::Clone;

use super::Utils::type_of;

//#[derive(Default)]
#[derive(Copy, Clone)]
pub struct DataField_t<T> where T: Clone {
    _paddingChar:char,
    _paddingOrientation:PaddingOrientation,

    /* TBD */
    //CachedValue:object,
    CachedValue:Option<T>,
    //Default:Box<DataField_t>, //= new(-1, -1, -1)

    pub HasPrefix:bool,
    pub Field:i32,
    pub Index:i32,
    pub Size:i32,
    pub Value:String,
    pub RawValue:Vec<u8>,
    pub TotalSize:i32
}

impl<T: std::clone::Clone> DataField_t<T> {

    fn setDefault(&mut self) {
        //let mut df:DataField_t;
        /*self.Default = Box::new(Self {
            //Default:Box::from(None),
            _paddingChar:' ',
            _paddingOrientation:PaddingOrientation::RightPadded,
            HasPrefix:true,
            Field : -1,
            Index : -1,
            Size:-1,
            Value : "".to_string(),
            RawValue:vec![],
            TotalSize:0
        });*/

        //self.Default.calTotalSize();
        

        /*df._paddingChar = ' ';
        df._paddingOrientation = PaddingOrientation::RightPadded;
        df.HasPrefix = true;
        df.Field = -1;
        df.Index = -1;
        df.Size = -1;
        df.Value = "".to_string();
        df.RawValue = vec![];
        df.calTotalSize();*/

        //self.Default = Box::new(df);
    }

    fn calTotalSize(&mut self) {
        self.TotalSize = if self.HasPrefix {2 + self.Size} else {self.Size};
    }

    pub fn new(field:i32, index:i32, size:i32, hasPrefix:Option<bool>) -> Self {
            let mut df = Self {
                //Default:Box::from(None),
                _paddingChar:' ',
                _paddingOrientation:PaddingOrientation::RightPadded,
                CachedValue:None,
                HasPrefix:hasPrefix.unwrap_or(true),
                Field : field,
                Index : index,
                Size: size,
                Value : "".to_string(),
                RawValue:vec![],
                TotalSize:0
            };
            df.calTotalSize();
            df.setDefault();

            df

    }

    pub fn new2(&mut self, field:i32 /*enum*/, index:i32, size:i32, hasPrefix:Option<bool>) -> Self {

        let mut df = Self {
            //Default:Box::from(None),
            _paddingChar:' ',
            _paddingOrientation:PaddingOrientation::RightPadded,
            CachedValue:None,
            HasPrefix:hasPrefix.unwrap_or(true),
            Field : field,//TBD:field.GetHashCode(), field.hash()?
            Index : index,
            Size: size,
            Value : "".to_string(),
            RawValue:vec![],
            TotalSize:0
        };
        df.calTotalSize();
        df.setDefault();

        df
    }

        pub fn new3(field:i32, index:i32, size:i32, paddingChar:char, paddingOrientation:Option<PaddingOrientation>,  hasPrefix:Option<bool>) -> Self {

            let mut df = Self {
                //Default:Box::from(None),
                _paddingChar:paddingChar,
                _paddingOrientation:paddingOrientation.unwrap_or(PaddingOrientation::RightPadded),
                CachedValue:None,
                HasPrefix:hasPrefix.unwrap_or(true),
                Field : field, 
                Index : index,
                Size: size,
                Value : "".to_string(),
                RawValue:vec![],
                TotalSize:0
            };
            df.calTotalSize();
            df.setDefault();
    
            df
        }

        pub fn new4(field:i32 /*enum*/, index:i32, size:i32, paddingChar:char, paddingOrientation:Option<PaddingOrientation>,  hasPrefix:Option<bool>) -> Self {

            let mut df = Self {
                //Default:Box::from(None),
                _paddingChar:paddingChar,
                _paddingOrientation:paddingOrientation.unwrap_or(PaddingOrientation::RightPadded),
                CachedValue:None,
                HasPrefix:hasPrefix.unwrap_or(true),
                Field : field, //TBD:field.GetHashCode(), field.hash()?
                Index : index,
                Size: size,
                Value : "".to_string(),
                RawValue:vec![],
                TotalSize:0
            };
            df.calTotalSize();
            df.setDefault();
    
            df
        }

        pub fn GetValue(mut self, converter:fn(String)->T) -> T {
            //let flag = self.IsValueNotCached();
            let slf2 = self;
            let v = &slf2.Value;
            if Utils::IsNullOrWhiteSpace(v.to_string()) {
                self.CachedValue = None; //TBD- T::default() ?
            } else {
                if self.clone().IsValueNotCached() {
                  //  if flag {
                    self.CachedValue = Some(converter(v.to_string()));
                }
            }

            self.CachedValue.unwrap()
        }

        pub fn GetValue2(mut self, converter:fn(Vec<u8>)->T) -> T {
            if self.RawValue.is_empty() || self.RawValue.is_empty() { //TBD for 1st one- == self.default ?
                self.CachedValue = None; //TBD-  T::default() ?
            } else {
                if self.clone().IsValueNotCached() {
                    self.CachedValue = Some(converter(self.RawValue));
                }
            }

            self.CachedValue.unwrap()
        }

        pub fn SetValue2<U>(mut self, converter:fn(char, i32, PaddingOrientation, U)->String, value:U) {
            self.CachedValue = None;
            self.Value = converter(self._paddingChar, self.Size, self._paddingOrientation, value);
            self.Size = self.Value.len() as i32;
        }

        pub fn SetRawValue(mut self, converter:fn(char, i32, PaddingOrientation, T)->Vec<u8>, value:T) {
            self.CachedValue = None;
            self.RawValue = converter(self._paddingChar, self.Size, self._paddingOrientation, value);
            self.Size = self.RawValue.len() as i32;
        }

        pub fn SetValue(mut self, value:String)
        {
            self.CachedValue = None;
            self.SetValue2(OpenProtocolConvert_t::TruncatePadded, value);
        }

        fn IsValueNotCached(self) -> bool {
            self.CachedValue.is_none() || self.IsNotTypeOf()
        }

        fn IsNotTypeOf(self) -> bool {

            Utils::type_of(self.CachedValue) != type_name::<T>() //TBD:will this work?
        }

}