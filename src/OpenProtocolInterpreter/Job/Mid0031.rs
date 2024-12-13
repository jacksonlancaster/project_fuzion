    /// <summary>
    /// Job ID upload reply
    /// <para>
    ///     The transmission of all the valid Job IDs of the controller. 
    ///     The data field contains the number of valid Jobs currently present in the controller, and the ID of each Job.
    /// </para>    
    /// <para>Message sent by: Controller</para>
    /// <para>Answer: None</para>
    /// </summary>
    

    use std::collections::HashMap;
    use substring::Substring;
    
    use crate::OpenProtocolInterpreter::DataField::DataFieldT;
    
    use crate::OpenProtocolInterpreter::Header::{self, HeaderT};
    use crate::OpenProtocolInterpreter::OpenProtocolConvert::OpenProtocolConvertT;
    use crate::OpenProtocolInterpreter::MID::MidT;
    use crate::OpenProtocolInterpreter::{Enums, Utils};
    
    
    pub(crate) enum DataFields
    {
        NumberOfJobs,
        EachJobId
    }
    
    #[derive(Default, Clone)]
    pub struct Mid0031T { //:Mid, IJob, IController
        pub mid:MidT,
        pub job_ids:Vec<i32>,
    }
    
    impl Mid0031T {
    
            pub const MID:i32 = 31;
    
            fn job_size(&self) -> i32 {
                if self.mid.header.revision > 1 {
                    4
                } else {
                    2
                }
            }

            pub fn total_jobs(&mut self) ->i32 {
                self.mid.get_field(1, DataFields::NumberOfJobs as i32).get_value(OpenProtocolConvertT::string_to_int32)
            }
        
            pub fn set_total_jobs(&mut self, value:i32) {
                self.mid.get_field(1, DataFields::NumberOfJobs as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value);
            }
    
            /*The following 4 methods are Common Methods to all MIDs  */
            pub fn new() -> Self {
                Self::new_rev(Header::DEFAULT_REVISION)
            }
    
            pub fn new_header(hdr:HeaderT) -> Self {

                let mut me = Self{mid:MidT::new(hdr), job_ids:Vec::new()};
                
                me.handle_revisions();

                me
            }
    
            fn new_rev(revision:i32) -> Self {
                let hdr1 = HeaderT{mid:Self::MID, revision:revision, ..Default::default()};
                Self::new_header(hdr1)
            }
    
            pub fn set_header(&mut self, hdr:HeaderT) {
                self.mid.header = hdr
            }
    
            pub fn process_header(&mut self, package:String)->HeaderT {
                self.mid.process_header(package)
            }

            pub fn pack(&mut self) ->String
            {
                self.set_total_jobs(self.job_ids.len() as i32);

                let mut each_job_field = self.mid.get_field(1, DataFields::EachJobId as i32);
                each_job_field.size = self.job_size() * self.total_jobs();
                each_job_field.value = self.pack_job_id_list();

                self.mid.pack()
            }
            /* Common methods end here */
        
            pub fn parse(&mut self, package:String)->Self {
                self.set_header(self.clone().process_header(package.clone()));
                self.handle_revisions();

                let mut each_job_field = self.mid.get_field(1, DataFields::EachJobId as i32);
                each_job_field.size = self.mid.header.length - each_job_field.index;
                self.mid.parse2(package);
                self.job_ids = self.parse_job_id_list(each_job_field.value);

                return self.clone();
            }
    
            pub(crate) fn pack_job_id_list(&mut self)->String 
            {
                let mut builder = String::new();
                for v in self.clone().job_ids {
                    builder.push_str(OpenProtocolConvertT::tp_i32_to_string('0', self.job_size(), Enums::PaddingOrientation::LeftPadded, v).as_str());
                }
                return builder;
            }


            pub(crate)  fn parse_job_id_list(&mut self, section:String)->Vec<i32> {
                let mut list:Vec<i32> = Vec::new();

                if Utils::is_null_or_white_space(section.clone()) {
                    return list;
                }

                let mut i:usize =0;
                while  i < section.len() {
                    list.push(OpenProtocolConvertT::string_to_int32(section.substring(i, i+self.job_size() as usize).to_string()));
                    i += self.job_size() as usize;
                }
    
                list
            }
    
            pub(crate) fn register_datafields(&mut self)->HashMap<i32, Vec<DataFieldT>> {
                let mut hmp:HashMap<i32, Vec<DataFieldT>> = HashMap::new();

                let mut v1:Vec<DataFieldT>= Vec::new();
                v1.push(DataFieldT::number(DataFields::NumberOfJobs as i32, 20, 2, Some(false)));
                v1.push(DataFieldT::number(DataFields::EachJobId as i32, 22, 2, Some(false)));
                hmp.insert(1, v1);
    
                hmp
            }

            fn handle_revisions(&mut self) {
                if self.mid.header.revision > 1 {
                    self.mid.get_field(1, DataFields::NumberOfJobs as i32).size = 4;
                    self.mid.get_field(1, DataFields::EachJobId as i32).index = 24;
                }
                else {
                    self.mid.get_field(1, DataFields::NumberOfJobs as i32).size = 2;
                    self.mid.get_field(1, DataFields::EachJobId as i32).size = 2;
                }
            }
    
    }