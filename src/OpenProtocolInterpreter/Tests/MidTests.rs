use crate::OpenProtocolInterpreter::Communication::Mid0001::Mid0001T;
use crate::OpenProtocolInterpreter::Communication::Mid0002::Mid0002T;
use crate::OpenProtocolInterpreter::Communication::Mid0004::Mid0004T;
use crate::OpenProtocolInterpreter::Communication::Mid0005::Mid0005T;
use crate::OpenProtocolInterpreter::Communication::Mid0008::Mid0008T;
use crate::OpenProtocolInterpreter::ParameterSet::Mid0010::Mid0010T;
use crate::OpenProtocolInterpreter::ParameterSet::Mid0011::Mid0011T;
use crate::OpenProtocolInterpreter::ParameterSet::Mid0014::Mid0014T;
use crate::OpenProtocolInterpreter::ParameterSet::Mid0015::Mid0015T;
use crate::OpenProtocolInterpreter::ParameterSet::Mid0018::Mid0018T;
use crate::OpenProtocolInterpreter::Job::Mid0031::Mid0031T;
use crate::OpenProtocolInterpreter::Tool::Mid0042::Mid0042T;
use crate::OpenProtocolInterpreter::Tool::Mid0043::Mid0043T;
use crate::OpenProtocolInterpreter::Vin::Mid0050::Mid0050T;
use crate::OpenProtocolInterpreter::Vin::Mid0051::Mid0051T;
use crate::OpenProtocolInterpreter::Vin::Mid0052::Mid0052T;
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
  
    println!("Mid0004 Test 1 = {}", mid0004.clone().pack());
}

pub fn test_mid0004_2() {

    //Application Communication negative acknowledge
    let mut mid0004:Mid0004T = Mid0004T::new();

    mid0004.mid.header = mid0004.mid.process_header("00260004001 001802".to_string());
    println!("Mid0004 Test 2 = {}", mid0004.clone().pack());
}

pub fn test_mid0004_all() {
    test_mid0004_1();
    test_mid0004_2();
}

//Mid0005 Tests
pub fn test_mid0005_1() {
    
     //Application Communication positive acknowledge
    let mut mid0005:Mid0005T = Mid0005T::new();
    mid0005.set_mid_accepted(18);
  
    println!("Mid0005 = {}", mid0005.clone().pack());
}

pub fn test_mid0005_2() {

    //Application Communication positive acknowledge
    let mut mid0005:Mid0005T = Mid0005T::new();

    mid0005.mid.header = mid0005.mid.process_header("00240005001 0018".to_string());
    println!("Mid0005 Test 2 = {}", mid0005.clone().pack());
}

pub fn test_mid0005_all() {
    test_mid0005_1();
    test_mid0005_2();
}

//Mid0008 Tests
pub fn test_mid0008_1() {
    
    //Application data message subscription
   let mut mid0008:Mid0008T = Mid0008T::new();
   mid0008.set_subscription_mid("1202".to_string());
 
   println!("Mid0008 = {}", mid0008.clone().pack());
}

pub fn test_mid0008_2() {

    //Application data message subscription
    let mut mid0008:Mid0008T = Mid0008T::new();

    mid0008.mid.header = mid0008.mid.process_header("00290008001 1202".to_string());
    println!("Mid0008 Test 2 = {}", mid0008.clone().pack());
}

pub fn test_mid0008_all() {
    test_mid0008_1();
    test_mid0008_2();
}

//Mid0010 Tests
pub fn test_mid0010_1() {
    
    //Parameter set ID upload request
   let mid0010:Mid0010T = Mid0010T::new();
 
   println!("Mid0010 = {}", mid0010.clone().pack());
}

pub fn test_mid0010_2() {

    //Parameter set ID upload request
    let mut mid0010 = Mid0010T::new();
    mid0010.mid.header = mid0010.mid.process_header("00200010001".to_string());
    println!("Mid0010 Test 2 = {}", mid0010.clone().pack());
}

pub fn test_mid0010_all() {
    test_mid0010_1();
    test_mid0010_2();
}

//Mid0011 Tests
pub fn test_mid0011_1() {
     //Parameter set ID upload reply
     let mut mid0011 = Mid0011T::new();
     mid0011.parameter_sets = vec![1, 2];

     println!("Mid0011 Test 1 = {}", mid0011.clone().pack());
}

pub fn test_mid0011_2() {

    //Parameter set ID upload reply
    let mut mid0011 = Mid0011T::new();
    mid0011.mid.header = mid0011.mid.process_header("00290011001 002001002".to_string());
    println!("Mid0011 Test 2 = {}", mid0011.clone().pack());
}

pub fn test_mid0011_all() {
    test_mid0011_1();
    test_mid0011_2();
}


//Mid0014 Tests
pub fn test_mid0014_1() {
    //Parameter set selected subscribe
    let mid0014 = Mid0014T::new();

    println!("Mid0014 Test 1 = {}", mid0014.clone().pack());
}

pub fn test_mid0014_2() {

   //Parameter set selected subscribe
   let mut mid0014 = Mid0014T::new();
   mid0014.mid.header = mid0014.mid.process_header("00200014001".to_string());
   println!("Mid0014 Test 2 = {}", mid0014.clone().pack());
}

pub fn test_mid0014_all() {
   test_mid0014_1();
   test_mid0014_2();
}

//Mid0015 Tests
pub fn test_mid0015_1() {

    let mut mid0015 = Mid0015T::new();
    mid0015.set_max_torque(2.3);
    mid0015.set_min_torque(1.0);
    mid0015.set_torque_final_target(2.2);
    mid0015.set_parameter_set_name("PSET1".to_string());
    mid0015.set_angle_final_target(500);
    mid0015.set_max_angle(700);
    mid0015.set_min_angle(400);
    mid0015.set_parameter_set_id(1);
    mid0015.set_rotation_direction(Enums::RotationDirection::Clockwise);
    println!("Mid0015 Test 1 = {}", mid0015.clone().pack());
}

pub fn test_mid0015_2() {

   let mut mid0015 = Mid0015T::new();
   mid0015.mid.header = mid0015.mid.process_header("00420015001 001".to_string());
   println!("Mid0015 Test 2 = {}", mid0015.clone().pack());
}

pub fn test_mid0015_all() {
   test_mid0015_1();
   test_mid0015_2();
}

//Mid0018 Tests
pub fn test_mid0018_1() {

    let mut mid0018 = Mid0018T::new();

    mid0018.set_parameter_set_id(1);

    println!("Mid0018 Test 1 = {}", mid0018.mid.pack());
}

pub fn test_mid0018_2() {

   let mut mid0018 = Mid0018T::new();
   mid0018.mid.header = mid0018.mid.process_header("00230018001 001".to_string());
   println!("Mid0018 Test 2 = {}", mid0018.mid.pack());
}

pub fn test_mid0018_all() {
   test_mid0018_1();
   test_mid0018_2();
}

//Mid0031 Tests
pub fn test_mid0031_1() {

    let mut mid0031 = Mid0031T::new();

    println!("Mid0031 Test 1 = {}", mid0031.mid.pack());
}

pub fn test_mid0031_2() {

   let mut mid0031 = Mid0031T::new();
   mid0031.mid.header = mid0031.mid.process_header("00220031001 00".to_string());
   println!("Mid0031 Test 2 = {}", mid0031.mid.pack());
}

pub fn test_mid0031_all() {
   test_mid0031_1();
   test_mid0031_2();
}

//Mid0042 Tests
pub fn test_mid0042_1() {
    //disable tool
    let mut mid0042 = Mid0042T::new();
    mid0042.set_tool_number(1);
    mid0042.set_disable_type(Enums::DisableType::Disable);
    println!("Mid0042 Test 1 = {}", mid0042.mid.pack());
}

pub fn test_mid0042_2() {
    // disable tool
   let mut mid0042 = Mid0042T::new();
   mid0042.mid.header = mid0042.mid.process_header("00200042001".to_string());
   println!("Mid0042 Test 2 = {}", mid0042.mid.pack());
}

pub fn test_mid0042_all() {
   test_mid0042_1();
   test_mid0042_2();
}


//Mid0043 Tests
pub fn test_mid0043_1() {
    //disable tool
    let mut mid0043 = Mid0043T::new();
    mid0043.set_tool_number(1);
    println!("Mid0043 Test 1 = {}", mid0043.mid.pack());
}

pub fn test_mid0043_2() {
    // enable tool
   let mut mid0043 = Mid0043T::new();
   mid0043.mid.header = mid0043.mid.process_header("00200043001".to_string());
   println!("Mid0043 Test 2 = {}", mid0043.mid.pack());
}

pub fn test_mid0043_all() {
   test_mid0043_1();
   test_mid0043_2();
}

//Mid0050 Tests
pub fn test_mid0050_1() {
    //MID 0050 Vehicle ID Number download request
    let mut mid0050 = Mid0050T::new();
    mid0050.set_vin_number("123456".to_string());
    println!("Mid0050 Test 1 = {}", mid0050.mid.pack());
}

pub fn test_mid0050_2() {
    //MID 0050 Vehicle ID Number download request
   let mut mid0050 = Mid0050T::new();
   mid0050.mid.header = mid0050.mid.process_header("00260050001 123456".to_string());
   println!("Mid0050 Test 2 = {}", mid0050.mid.pack());
}

pub fn test_mid0050_all() {
   test_mid0050_1();
   test_mid0050_2();
}

//Mid0051 Tests
pub fn test_mid0051_1() {
    //MID 0050 Vehicle ID Number download request
    let mut mid0051 = Mid0051T::new();
    println!("Mid0051 Test 1 = {}", mid0051.mid.pack());
}

pub fn test_mid0051_2() {
    //MID 0050 Vehicle ID Number download request
   let mut mid0051 = Mid0051T::new();
   mid0051.mid.header = mid0051.mid.process_header("00200051001".to_string());
   println!("Mid0051 Test 2 = {}", mid0051.mid.pack());
}

pub fn test_mid0051_all() {
   test_mid0051_1();
   test_mid0051_2();
}

//Mid0052 Tests
pub fn test_mid0052_1() {
    //MID 0050 Vehicle ID Number download request
    let mut mid0052 = Mid0052T::new();
    mid0052.set_vin_number("123456".to_string());
    println!("Mid0052 Test 1 = {}", mid0052.mid.pack());
}

pub fn test_mid0052_2() {
    //MID 0050 Vehicle ID Number download request
   let mut mid0052 = Mid0052T::new();
   mid0052.mid.header = mid0052.mid.process_header("00450052001 123456".to_string());
   println!("Mid0052 Test 2 = {}", mid0052.mid.pack());
}

pub fn test_mid0052_all() {
   test_mid0052_1();
   test_mid0052_2();
}