use crate::OpenProtocolInterpreter::Header::HeaderT;
use crate::OpenProtocolInterpreter::DataField::DataFieldT;
use crate::OpenProtocolInterpreter::Utils;
use std::collections::HashMap;
use std::hash::Hash;
use substring::Substring;

#[derive(Default, Clone /*, Copy*/)]
pub struct MidT {
    pub header:HeaderT,
    pub(crate) revisions_by_fields:HashMap<i32, Vec<DataFieldT>>
}

impl MidT {

        const DEFAULT_REVISION: i32 = 1;

        fn clone_revisions_by_fields(&self) -> HashMap<i32, Vec<DataFieldT>> {
            self.revisions_by_fields.clone()
        }

        pub fn new(hdr:HeaderT) -> Self {
            //RevisionsByFields = new SafeAccessRevisionsFields(RegisterDatafields());
            Self{header:hdr, revisions_by_fields:Self::register_datafields()}
        }

        pub fn new2(mid:i32, revision:i32, no_ack_flag:Option<bool>) -> Self /* : this(new Header() */ {
            let mut hdr :HeaderT = Default::default();
            hdr.mid = mid;
            hdr.revision = revision;
            hdr.no_ack_flag = no_ack_flag.unwrap_or(false);
            Self{header:hdr, ..Default::default()}
        }

        pub(crate) fn build_header(&mut self) -> String {
            if !self.revisions_by_fields.is_empty() {
                self.header.length = 20;
                let mut i:i32 = 1;
                while i <= if self.header.revision > 0  {self.header.revision} else {1} {
                    if self.revisions_by_fields.contains_key(&i) {
                        let data_fields = self.revisions_by_fields.get(&i);
                        for  data_field in data_fields.unwrap() {
                            self.header.length += if data_field.has_prefix  {2 } else {0} + data_field.size;
                        }
                    }
                    i += 1;
                }
            }
            return self.header.to_string();
        }

        pub(crate) fn pack(&mut self) -> String {
            let header = self.build_header();
            if self.revisions_by_fields.is_empty() {
                return header
            }

            let mut builder = String::new();
            let mut prefixIndex:i32 = 1;
            let revision = if self.header.revision > 0  {self.header.revision} else {1};
            let mut i:i32 = 1;
            while i <= revision {
                builder.push_str(&self.pack2(i, &mut prefixIndex));
                i +=1;
            }

            return builder;
        }

        pub(crate) fn pack2(&mut self, revision:i32, prefix_index:&mut i32) -> String {
            
            if !self.revisions_by_fields.contains_key(&revision) {
                return "".to_string();
            }

            let data_fields = self.revisions_by_fields.get(&revision).unwrap();
            MidT::pack3(data_fields, prefix_index)
        }

        pub(crate) fn pack3(data_fields:&Vec<DataFieldT>, prefix_index:&mut i32) -> String {
            let mut builder = String::new();
            for data_field in data_fields {
                if data_field.has_prefix {
                    builder.push_str(&format!("{:02}", prefix_index)[..]); //("D2"));
                    *prefix_index += 1;
                }

                builder.push_str(&data_field.value[..]);
            }

            return builder;
        }

        pub(crate) fn register_datafields() ->HashMap<i32, Vec<DataFieldT>> {
            let rvf:HashMap<i32, Vec<DataFieldT>> = HashMap::new();

            rvf
        }

        pub(crate) fn process_header(&mut self, package:String) -> HeaderT {
            HeaderT::process_header(package)
        }

        pub fn parse2(&mut self, package: String) -> Self {
            let pkg_dup = package.clone();
            self.header = self.clone().process_header(pkg_dup);
            self.process_data_fields(package);

            self.clone()
        }
        
        pub fn parse(&mut self, package:&[u8]) -> Self {
            let pack = Utils::to_ascii(package);
            self.parse2(pack)
        }

        pub(crate) fn process_data_fields(&mut self, package:String)
        {
            if self.revisions_by_fields.is_empty() {
                return;
            }

            let revision = if self.header.revision > 0  {self.header.revision} else {1};
            let mut i = 1;
            while i <= revision {
                self.process_data_fields2(i, package.clone());
                i += 1;
            }
        }

        pub(crate) fn process_data_fields2(&mut self, revision:i32, package:String) {
            if self.revisions_by_fields.contains_key(&revision) {

                let mut fields: Vec<DataFieldT> = self.revisions_by_fields.get(&revision).unwrap().to_vec();
                self.clone().process_data_fields3(&mut fields, package);
            }
        }

        pub(crate) fn process_data_fields3(self, data_fields:&mut Vec<DataFieldT>, package:String) {
            for dataField in data_fields {
                dataField.value = self.clone().get_value(dataField, package.clone());
            }
        }

        pub(crate) fn get_value(self, field:&DataFieldT, package:String) -> String {
                let res = if field.has_prefix {package.substring(2 + field.index as usize, field.size as usize)} else {package.substring(field.index as usize, field.size as usize)};
                res.to_string()
        }

        pub(crate) fn get_value2(self, field:DataFieldT, package:&[u8]) -> Vec<u8>
        {
            let mut bytes:Vec<u8> = vec![0;field.size as usize];
            let index = if field.has_prefix { 2 + field.index} else {field.index};
            let mut j = 0;
            let mut i = index as usize;
            while i < (index as usize + field.size as usize) {
                bytes[j] = package[i];
                j += 1;
                i += 1;
            }

            bytes
        }

        pub(crate) fn  get_field(&mut self, revision:i32, field:i32) ->DataFieldT
        {
            let result:DataFieldT;
            if !self.revisions_by_fields.contains_key(&revision) {
                result = DataFieldT::default();
            } else {
                let fields = self.revisions_by_fields.get(&revision).unwrap();

                result = fields.iter()
                                .find(|x| x.field == field)
                                .cloned()
                                .unwrap_or(DataFieldT::default());
            }
            result
        }

        pub(crate) fn get_field2<TEnum>(&mut self, revision:i32, field:TEnum) -> DataFieldT 
            where 
                TEnum: Hash
        {
            self.get_field(revision, Utils::get_hash_code(field))
        }

        pub(crate) fn to_ascii(bytes:&[u8])-> String {
            let s = match std::str::from_utf8(bytes) {
                Ok(v)=> v.to_string(),
                Err(_er) => "".to_string(),
            };

            s
        }

        pub(crate) fn to_bytes(value:String)->Vec<u8> {
             value.as_bytes().to_vec()
        }
    

    }