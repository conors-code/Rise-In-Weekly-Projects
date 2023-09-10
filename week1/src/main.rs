fn main() {
    let string1: String = String::from("start...");
    let string2: String = String::from("end!");
    let result_string:String = concatenate_strings(&string1, &string2);
    println!("Concatenated result is: {}", result_string);
}

fn concatenate_strings(start_str_slice: &str, end_str_slice: &str) -> String {
    let mut result: String = String::from("");
    result.push_str(start_str_slice);
    result.push_str(end_str_slice);
    result
}