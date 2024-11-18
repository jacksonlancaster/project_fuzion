use crate::OpenProtocolInterpreter::Communication::Mid0001::Mid0001T; 
use crate::OpenProtocolInterpreter::Communication::Mid0002::Mid0002T; 
use crate::OpenProtocolInterpreter::Communication::Mid0004::Mid0004T; 
use crate::OpenProtocolInterpreter::Enums;

//Mid0001 Tests
pub fn test_mid0001_1() {
    //MID 0001 Application Communication start
    let mut mid0001 = Mid0001T::new();
    mid0001.set_optional_keep_alive(true);
    println!("Mid0001 = {}", mid0001.clone().mid.pack());
}

//Mid0002 Tests
pub fn test_mid0002_1() {
    
    //MID 0002 Application Communication start acknowledge
    let mut mid0002 = Mid0002T::new();
    mid0002.set_channel_id(1);
    mid0002.set_cell_id(1);
    mid0002.set_controller_name("Tool1".to_string());
    println!("Mid0002 = {}", mid0002.clone().mid.pack());
}

//Mid0004 Tests
pub fn test_mid0004_1() {
    
    //Application Communication negative acknowledge
    let mut mid0004:Mid0004T = Mid0004T::new();
    mid0004.set_failed_mid(18);
    mid0004.set_error_code(Enums::Error::ParameterSetIdNotPresent);
  
    println!("Mid0004 = {}", mid0004.clone().pack());
}