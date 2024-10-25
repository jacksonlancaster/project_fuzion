use crate::OpenProtocolInterpreter::Header::{self, Header_t};
use crate::OpenProtocolInterpreter::DataField::{self, DataField_t};
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
                        //TryGetValue(i, out var dataFields) {
                        //foreach (var dataField in dataFields)
                        for dataField in self.RevisionsByFields.get(&i) {
                            self.Header.Length += (dataField.HasPrefix ? 2 : 0) + dataField.Size;
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
            
            var builder = new StringBuilder(header);
            int prefixIndex = 1;
            var revision = (Header.Revision > 0 ? Header.Revision : 1);
            for (int i = 1; i <= revision; i++)
            {
                builder.Append(Pack(i, ref prefixIndex));
            }

            return builder.ToString();
        }
}