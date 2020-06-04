// iterators2.rs
// In this module, you'll learn some of unique advantages that iterators can offer
// Step 1. Complete the `capitalize_first` function to pass the first two cases
// Step 2. Apply the `capitalize_first` function to a vector of strings, ensuring that it returns a vector of strings as well
// Step 3. Apply the `capitalize_first` function again to a list, but try and ensure it returns a single string
// As always, there are hints if you execute `rustlings hint iterators2`!


pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        // First convert the char to uppercase
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
    }
}

// Here the input is Vec<&str>
pub fn capitalized_words(input: Vec<&str>) -> Vec<String> {
    let mut iter = input.iter().map(|x| capitalize_first(x).to_string());
    iter.collect::<Vec<_>>()

}

#[cfg(test)]
mod tests {
    use super::*;

    // Step 1.
    // Tests that verify your `capitalize_first` function implementation
    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    // Step 2.
    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        let capitalized_words: Vec<String> = capitalized_words(words);
        assert_eq!(capitalized_words, ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        let capitalized_words = capitalized_words(words).iter().fold(String::from(""), |a, b| (a.to_owned() + b));
        assert_eq!(capitalized_words, "Hello World");
    }
}
