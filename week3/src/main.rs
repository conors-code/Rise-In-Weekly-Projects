//Create a test collection and a filter condition instance.
//Filter the collection and print the result.
//Pick days of the week containing the letter 'n'.
fn main() {
    let items_to_check: Vec<&str> = vec![
        "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"
    ];

    let my_filter_cond :FilterCondition = FilterCondition { 
        pickme: String::from("n") 
    };

    let my_filtered_items: Vec<&str> = custom_filter(
      my_filter_cond, items_to_check
    );
    println!("My filtered items are {:?}", my_filtered_items);

}
//define a rust struct called FilterCondition
struct FilterCondition {
    pickme : String
}

//is_match method on the FilterCondition takes a reference to an
//item of the same type as the filter condition and returns a boolean 
// indicating whether the item matches the condition or not.
impl FilterCondition {
    fn is_match(&self, check_me: &str) -> bool {
        check_me.contains(&self.pickme)
    }
}
//custom_filter function takes a collection (e.g., a vector) and a 
//FilterCondition reference, iterates over the collection and returns a
//new collection of elements matching the condition.
fn custom_filter(condition: FilterCondition, items: Vec<&str>) -> Vec<&str> {
    items.into_iter().filter(|&item| condition.is_match(item)).collect()
}
