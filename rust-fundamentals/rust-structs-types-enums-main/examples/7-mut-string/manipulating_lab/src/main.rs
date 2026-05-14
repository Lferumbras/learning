/*
Challenge Questions:
    Modify the code to count the number of occurrences of each vowel (a, e, i, o, u) in the sentence. Print the count for each vowel individually.
    Extend the program by implementing a function that takes a sentence as input and returns the longest word in the sentence. Invoke this function with the `sentence` variable and print the longest word.
*/

use std::collections::HashMap;

fn count(rust_map: &mut HashMap<char, i32>, c: char){
    let contador = rust_map.entry(c).or_insert(0);
    *contador += 1;
}

fn find_longest_word(sentence : &str) -> String{
    let words: Vec<&str> = sentence.split_whitespace().collect();
    let mut longest_word = String::new();

    for word in words{
        if word.len() > longest_word.len(){
            longest_word = word.to_string();
        }
    }
    
    longest_word
    
}

fn main() {
    let mut rust_map: HashMap<char,i32> = HashMap::new();

    let sentence = "the quick brown fox jumps over the lazy dog".to_string();

    for c in sentence.chars() {
        match c {
                'a' | 'e' | 'i' | 'o' | 'u' => count(&mut rust_map, c),// println!("Got a vowel{:?}",c),
                _ => continue,
        }
    }

    println!("Vowel counts:");

    for vowel in ['a', 'e', 'i', 'o', 'u'] {
        let count = rust_map.get(&vowel).unwrap_or(&0);
        println!("{}: {}", vowel, count);
    }

    let longest_word = find_longest_word(&sentence);

    println!("The longest word into sentence '{}' is '{}'",sentence, longest_word);

}
