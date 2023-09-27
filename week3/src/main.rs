//use std::fmt;

//Create a test collection and a filter condition instance.
//Filter the collection and print the result.
fn main() {
    let items_to_check: Vec<bool> = vec![true, true, false, true, false];
    let my_filter_cond :FilterCondition = FilterCondition { pickme: true };

    let my_filtered_items: Vec<bool> = custom_filter(
      my_filter_cond, items_to_check
    );
    println!("My filtered items are {:?}", my_filtered_items);

}
//define a rust struct called FilterCondition
struct FilterCondition {
    pickme : bool,
}

//is_match method on the FilterCondition takes a reference to an
//item of the same type as the filter condition and returns a boolean 
// indicating whether the item matches the condition or not.
impl FilterCondition {
    fn is_match(&self, check_me: bool) -> bool {
        self.pickme == check_me
    }
}
//custom_filter function takes a collection (e.g., a vector) and a 
//FilterCondition reference, iterates over the collection and returns a
//new collection of elements matching the condition.
fn custom_filter(condition: FilterCondition, items: Vec<bool>) -> Vec<bool> {
    items.into_iter().filter(|&item| condition.is_match(item)).collect()
}
