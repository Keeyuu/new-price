pub fn get_day_collection(base: String, code: String, size: u8) -> String {
    let slice: &[u8] = code.as_ref();
    let mut index: u8 = 0;
    if slice.len() >= 1 {
        index = slice[slice.len() - 1] % size;
    }
    format!("{}{}", base, index)
}
