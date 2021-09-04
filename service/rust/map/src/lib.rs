#[no_mangle]
pub extern "C" fn get_day_collection(base: &str, code: &str) -> String {
    let mut index: u8 = 0;
    for i in code.bytes() {
        index = i as u8 % 10;
    }
    format!("{}{}", base, index)
}

#[test]
fn test_get_day_collection() {
    assert_eq!(get_day_collection("codo_", "1"), "2".to_string());
}
