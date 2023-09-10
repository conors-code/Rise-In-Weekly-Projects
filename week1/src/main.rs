fn main() {
    //Declare two Strings
    let string1: String = String::from("start...");
    let string2: String = String::from("end!");
    //Call the function with string slices (references) of the created strings
    let result_string:String = concatenate_strings(&string1, &string2);
    //Print the result of the call
    println!("Concatenated result is: {}", result_string);
}

//Create a function to concatenate two String slices, returning the resulting
//concatenated String.
fn concatenate_strings(start_str_slice: &str, end_str_slice: &str) -> String {
    let mut result: String = String::from("");
    result.push_str(start_str_slice);
    result.push_str(end_str_slice);
    result
}

//Test: Show that the String returned by calling concatenate_strings on two
//string slices is the same as creating a String using the text in the slices.
#[cfg(test)]
mod tests {
    use crate::concatenate_strings;

    #[test]
    fn concatenate_strings_works() {
        //set up test values and expected result
        let test_str1: &str = "First";
        let test_str2: &str = "Second";
        let expected_result_string: String = String::from("FirstSecond");
        //call function under test with test values...
        let concat_result = concatenate_strings(test_str1, test_str2);
        //...and verify that result is as expected.
        assert_eq!(expected_result_string, concat_result);
    }
}