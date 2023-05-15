// iterators2.rs
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    let mut letter = match c.next() {
        None => String::new(),
        Some(first) => first.to_string().to_uppercase(),
    };

    let str_copy = c.clone();

    let res1 = format!("{}{}", letter, &c.collect::<String>());
    println!("res1 : {:?}", res1 );

    letter += &str_copy.collect::<String>();
    let res2 = letter.clone();
    println!("res2 : {:?}", res2 );

    let mut string = String::from(input);
    if letter!=""{ 
        string.replace_range(0..1,&letter);}
    string

}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    let mut vec = Vec::new();
    for i in words.into_iter(){
        vec.push(capitalize_first(i));
    }
    vec
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    let mut string = String::new();
    for i in words.into_iter(){
        string.push_str(&capitalize_first(i));
    }
    string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        //println!("capitalize_first('hello') : {:?}", capitalize_first("hello") );
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}

fn main() {
    println!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX" );
    println!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX" );
    println!("capitalize_first('hello')) : {:?}", capitalize_first("hello") );
    println!("capitalize_first('') : {:?}",  capitalize_first(""));
    println!("capitalize_words_vector(vec!['hello','world']) : {:?}", capitalize_words_vector(&vec!["hello","world"]));
    println!("capitalize_words_string(&words) : {:?}", capitalize_words_string(&vec!["hello"," ","world"])) ;
    println!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX" );
    println!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX" );
}
