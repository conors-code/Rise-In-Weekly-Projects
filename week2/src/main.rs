use  std::io;

fn main() {

    //Declare arg holders instances
    let mut arg1_as_string:String = String::new();
    let mut arg2_as_string:String = String::new();


    println!("Please enter your first number, decimals allowed");
    io::stdin().read_line(&mut arg1_as_string).expect("Failed to read input");
    let arg1:f64 = arg1_as_string.trim().parse().unwrap();
    println!("Please enter your second number, decimals allowed");
    io::stdin().read_line(&mut arg2_as_string).expect("Failed to read input");
    let arg2:f64= arg2_as_string.trim().parse().unwrap();

    println!("Please enter your operation: choose one of:");
    println!("add, subtract, divide or multiply");
    println!("Please enter first number");
    
    let mut input_op_string = String::new();
    io::stdin().read_line(&mut input_op_string).expect("Failed to read input");

    //holder for the operation's result
    //let mut input_operation: Operation; 
    let input_op:Operation = match input_op_string.trim() {
        "add" => {
            Operation::Add(arg1, arg2)
        },
        "subtract" => {
            Operation::Subtract(arg1, arg2)
        },
        "multiply" => {
            Operation::Multiply(arg1, arg2)
        },
        "divide" => {
            Operation::Divide(arg1, arg2)
        },
        _ => {
            println!("{} is invalid input", input_op_string.trim());
            return ();
        }
    };

    let input_result = calculate(input_op);

    match input_result {
       Ok(result_num) => println!("{} with {} and {} gives: {}", 
         input_op_string, arg1_as_string, arg2_as_string, result_num),
       Err(result_err_msg) => println!("{}", result_err_msg),
    }
}

//Declare an enum for each of the four mathematical operations
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(op: Operation) -> Result<f64, String> {
    match op {
        Operation::Divide(arg1, arg2) => { 
            if arg2 == 0.0 {
                Err(String::from("Cannot divide by 0."))
            } else {
                Ok(arg1 / arg2) 
            }
        },
        Operation::Add(arg1, arg2) => { 
            Ok(arg1 + arg2) 
        },
        Operation::Subtract(arg1, arg2) => { 
            Ok(arg1 - arg2) 
        },
        Operation::Multiply(arg1, arg2) => { 
            Ok(arg1 * arg2) 
        },
    }
}

//Test: Show that the String returned by calling concatenate_strings on two
//string slices is the same as creating a String using the text in the slices.
#[cfg(test)]
mod tests {
    use crate::calculate;
    use crate::Operation;

    #[test]
    fn calculate_works() {
        //set up test values and expected result
        let test_num_1_7: f64 = 1.7;
        let test_num_1_2: f64 = 1.2;
        let test_num_2_0: f64 = 2.0;
        let test_add_operation = Operation::Add(test_num_1_7, test_num_1_2);
        let expected_add_result_num: f64 = 2.9;
        let test_subtract_operation = Operation::Subtract(test_num_1_7, test_num_1_2);
        let expected_subtract_result_num: f64 = 0.5;
        let test_divide_operation = Operation::Divide(test_num_1_2, test_num_2_0);
        let expected_divide_result_num: f64 = 0.6;
        let test_multiply_operation = Operation::Multiply(test_num_1_2, test_num_2_0);
        let expected_multiply_result_num: f64 = 2.4;
        //call function under test with test values...
        let test_add_result = calculate(test_add_operation);
        let test_subtract_result = calculate(test_subtract_operation);
        let test_divide_result = calculate(test_divide_operation);
        let test_multiply_result = calculate(test_multiply_operation);
        
        //assert correct match, or print error
        match test_add_result {
            Ok(actual_add_result_num) => 
              assert_eq!(expected_add_result_num, actual_add_result_num),
            Err(result_err_msg) => println!("{}", result_err_msg),
        } 
        match test_subtract_result {
            Ok(actual_subtract_result_num) => assert_eq!(
                expected_subtract_result_num, 
                actual_subtract_result_num
            ),
            Err(result_err_msg) => println!("{}", result_err_msg),
        }
        match test_divide_result {
            Ok(actual_divide_result_num) => assert_eq!(
                expected_divide_result_num,
                actual_divide_result_num
            ),
            Err(result_err_msg) => println!("{}", result_err_msg),
        }
        match test_multiply_result {
            Ok(actual_multiply_result_num) => assert_eq!(
                expected_multiply_result_num,
                actual_multiply_result_num
            ),
            Err(result_err_msg) => println!("{}", result_err_msg),
        }        
    }
}