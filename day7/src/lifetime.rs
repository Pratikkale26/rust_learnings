// Example demonstrating Rust lifetimes
// Lifetimes ensure that references are valid for as long as we need them

// This function takes two string slices and returns the longer one
// The lifetime parameter 'a ensures both input references and the output reference
// live for the same duration
fn longest_str<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Struct with a lifetime parameter
// This tells Rust that the struct cannot outlive the reference it holds
struct Excerpt<'a> {
    part: &'a str,
}

fn main() {
    // Example 1: Basic lifetime usage
    let str1 = String::from("Pratik");
    let str2 = String::from("Kale");
    
    let result = longest_str(&str1, &str2);
    println!("The longer string is: {}", result);

    // Example 2: Struct with lifetime
    let novel = String::from("Call me Pratik. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    
    let excerpt = Excerpt {
        part: first_sentence,
    };
    println!("Excerpt: {}", excerpt.part);

    // Example 3: Multiple lifetimes
    let string1 = String::from("long string is long");
    let result2;
    {
        let string2 = String::from("xyz");
        result2 = longest_str(&string1, &string2);
        println!("The longer string is: {}", result2);
    }
    // This would not compile because string2's lifetime ends here
    // println!("The longer string is: {}", result2);
}

// Example of a function with multiple lifetime parameters
fn multiple_lifetimes<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    x
}