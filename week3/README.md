# TASK: Create a filter using a struct, a vector and an iterator
Week 3 Task.
## Steps

-    Open the main.rs file in a text editor.
-    Define a struct called FilterCondition with a single field of the desired type for filtering.
-    Implement a method called is_match on the FilterCondition struct that takes a reference to an item of the same type as the filter condition and returns a boolean indicating whether the item matches the condition.
-    Define a function called custom_filter that takes a collection (e.g., a vector) and a reference to a FilterCondition object as arguments. The function should iterate over the elements in the collection and return a new collection containing only the elements that match the filter condition.
-    In the main function, create a collection (e.g., a vector) with some elements and initialize a FilterCondition object with the desired value.
-    Call the custom_filter function with the collection and the FilterCondition object, storing the result in a new variable.
-    Print the filtered result to the console.
-    Compile and run the program to test its functionality.

## Checklist ✅ 
-    Create a new Rust project and open the main.rs file.
-    Define the FilterCondition struct with the desired type for filtering.
-    Implement the is_match method on the FilterCondition struct.
-    Define the custom_filter function to filter elements based on the condition.
-    Create a collection and a FilterCondition object in the main function.
-    Call the custom_filter function and store the result.
-    Print the filtered result to the console.
-    Compile and run the program to test its functionality.