
// pub extern "C" fn get_day_collection(base: String, code: String) -> String {
//     let slice: &[u8] = code.as_ref();
//     let mut index: u8 = 0;
//     if slice.len() >= 1 {
//         index = slice[slice.len() - 1] % 10;
//     }
//     format!("{}{}", base, index)
// }

#[no_mangle]
pub extern "C" fn test(){
    println!("test")
}ew