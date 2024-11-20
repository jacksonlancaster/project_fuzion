use crate::OpenProtocolInterpreter::Communication::Mid0001::Mid0001T;
use crate::OpenProtocolInterpreter::Communication::Mid0002::Mid0002T;
use crate::OpenProtocolInterpreter::Communication::Mid0004::Mid0004T;
use crate::OpenProtocolInterpreter::Communication::Mid0005::Mid0005T;
use crate::OpenProtocolInterpreter::Communication::Mid0008::Mid0008T;
use crate::OpenProtocolInterpreter::Enums;

//Mid0001 Tests
pub fn test_mid0001_1() {
    //MID 0001 Application Communication start
    let mut mid0001 = Mid0001T::new();
    mid0001.set_optional_keep_alive(true);
    println!("Mid0001 Test 1 = {}", mid0001.clone().pack());
}

pub fn test_mid0001_2() {
    //MID 0001 Application Communication start
    let mut mid0001 = Mid0001T::new();
    mid0001.mid.header = mid0001.mid.process_header("00200001001".to_string());
    println!("Mid0001 Test 2 = {}", mid0001.clone().pack());
}

pub fn test_mid0001_all() {
    test_mid0001_1();
    test_mid0001_2();
}

//Mid0002 Tests
pub fn test_mid0002_1() {
    
    //MID 0002 Application Communication start acknowledge
    let mut mid0002 = Mid0002T::new();
    mid0002.set_channel_id(1);
    mid0002.set_cell_id(1);
    mid0002.set_controller_name("Tool1".to_string());
    println!("Mid0002 Test 1 = {}", mid0002.clone().pack());
}

pub fn test_mid0002_2() {

    //MID 0002 Application Communication start acknowledge
    let mut mid0002 = Mid0002T::new();
    mid0002.mid.header = mid0002.mid.process_header("00570002001 010001020103Tool1".to_string());
    println!("Mid0002 Test 2 = {}", mid0002.clone().pack());
}

pub fn test_mid0002_all() {
    test_mid0002_1();
    test_mid0002_2();
}

//Mid0004 Tests
pub fn test_mid0004_1() {
    
    //Application Communication negative acknowledge
    let mut mid0004:Mid0004T = Mid0004T::new();
    mid0004.set_failed_mid(18);
    mid0004.set_error_code(Enums::Error::ParameterSetIdNotPresent);
  
    println!("Mid0004 = {}", mid0004.clone().pack());
}

//Mid0005 Tests
pub fn test_mid0005_1() {
    
     //Application Communication positive acknowledge
    let mut mid0005:Mid0005T = Mid0005T::new();
    mid0005.set_mid_accepted(18);
  
    println!("Mid0005 = {}", mid0005.clone().pack());
}

//Mid0008 Tests
pub fn test_mid0008_1() {
    
    //Application Communication positive acknowledge
   let mut mid0008:Mid0008T = Mid0008T::new();
   mid0008.set_subscription_mid("1202".to_string());
 
   println!("Mid0008 = {}", mid0008.clone().pack());
}