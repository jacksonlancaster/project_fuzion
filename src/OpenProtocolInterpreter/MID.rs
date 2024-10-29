use crate::OpenProtocolInterpreter::Header::Header_t;
use crate::OpenProtocolInterpreter::DataField::DataField_t;
use std::collections::HashMap;

#[derive(Default)]
pub struct Mid_t {
    pub Header:Header_t,
    /*Dictionary<int, List<DataField>>*/ RevisionsByFields:HashMap<i32, Vec<DataField_t>>
}

impl Mid_t {

        pub fn new(hdr:Header_t) -> Self {
            //Header = hdr;
            //RevisionsByFields = new SafeAccessRevisionsFields(RegisterDatafields());
            Self{Header:hdr, ..Default::default()}
        }

        pub fn new2(mid:i32, revision:i32, noAckFlag:Option<bool>) -> Self /* : this(new Header() */ {
            let mut hdr :Header_t = Default::default();
            hdr.Mid = mid;
            hdr.Revision = revision;
            hdr.NoAckFlag = noAckFlag.unwrap_or(false);
            Self{Header:hdr, ..Default::default()}
        }

        fn BuildHeader(&mut self) -> String {
            if !self.RevisionsByFields.is_empty() {
                self.Header.Length = 20;
                let mut i:i32 = 1;
                while i <= if self.Header.Revision > 0  {self.Header.Revision} else {1} {
                    if self.RevisionsByFields.contains_key(&i) {
                        let dataFields = self.RevisionsByFields.get(&i);
                        for  dataField in dataFields.unwrap() {
                            self.Header.Length += if dataField.HasPrefix  {2 } else {0} + dataField.Size;
                        }
                    }
                    i += 1;
                }
            }
            return self.Header.ToString();
        }

        pub fn Pack(&mut self) -> String {
            let header = self.BuildHeader();
            if self.RevisionsByFields.is_empty() {
                return header
            }

            let mut builder = String::new();
            let mut prefixIndex:i32 = 1;
            let revision = if self.Header.Revision > 0  {self.Header.Revision} else {1};
            let mut i:i32 = 1;
            while i <= revision {
                builder.push_str(&self.Pack2(i, &mut prefixIndex));
                i +=1;
            }

            return builder;
        }

        fn Pack2(&mut self, revision:i32, prefixIndex:&mut i32) -> String {
            if !self.RevisionsByFields.contains_key(&revision) {
                return "".to_string(); // string.Empty;
            }

            let dataFields = self.RevisionsByFields.get(&revision).unwrap();

            return self.Pack3(dataFields, prefixIndex);
        }

        fn Pack3<T>(self, dataFields:&Vec<DataField_t<T>>, prefixIndex:&mut i32) -> String {
            let mut builder = String::new();
            for dataField in dataFields {
                if dataField.HasPrefix {
                    builder.push_str(&format!("{:02}", prefixIndex)[..]); //("D2"));
                    *prefixIndex += 1;
                }

                builder.push_str(&dataField.Value[..]);
            }

            return builder;
        }

        fn ProcessHeader(package:String) -> Header_t {
            Header_t::ProcessHeader(package)
        }
}