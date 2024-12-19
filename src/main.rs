use std::default::Default;
use Fusion::OpenProtocolInterpreter::Header::HeaderT;
use Fusion::OpenProtocolInterpreter::Tests::MidTests;


fn main() {

    let mut hdr1:HeaderT = Default::default();

    hdr1.length = 45;
    let mut hdr2 = HeaderT::process_header("00200014001".to_string());
    println!("Header 1 = {}", hdr1.to_string());
    println!("Header 2 = {}", hdr2.to_string());
    println!("Header 3 = {}", HeaderT::process_header("00570002 010001020103Airbag1 NUL".to_string()).to_string());

    MidTests::test_mid0001_all();
    MidTests::test_mid0002_all();
    MidTests::test_mid0004_all();
    MidTests::test_mid0005_all();
    MidTests::test_mid0008_all();
    MidTests::test_mid0010_all();
    MidTests::test_mid0011_all();
    MidTests::test_mid0012_all();
    MidTests::test_mid0013_all();
    MidTests::test_mid0014_all();
    MidTests::test_mid0015_all();
    MidTests::test_mid0018_all();
    MidTests::test_mid0031_all();
    MidTests::test_mid0042_all();
    MidTests::test_mid0043_all();
    MidTests::test_mid0050_all();
    MidTests::test_mid0051_all();
    MidTests::test_mid0052_all();
    MidTests::test_mid0053_all();
    MidTests::test_mid0060_all();
    MidTests::test_mid0061_all();
    MidTests::test_mid0062_all();
    MidTests::test_mid0070_all();
    MidTests::test_mid0071_all();
    MidTests::test_mid9999_all();
}
