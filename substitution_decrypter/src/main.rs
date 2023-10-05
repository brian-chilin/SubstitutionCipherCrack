use std::env;
use std::collections::HashMap;

fn main() {
    println!("Usage: substitution_decrypter [26capital_letter_key] \"[ciphertext_phrase]\"\n*May break with some characters like '-'\n\n");
    let mut iter = env::args();
    iter.next(); //throw away garbage: the first substitution_encrypter.exe in the command
    //Store key and phrase in separate String variables
    let key = iter.next().unwrap();
    let phrase = iter.next().unwrap().to_uppercase(); //println!("{}\n{}", key, phrase);
    //Prepare the original alphabet, key, and map to populate the map
    let alpha = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut k = key.chars().into_iter();
    let mut map: HashMap<char, char> = HashMap::new();
    //Make the mapping
    for c in alpha.chars() {
        map.insert(k.next().unwrap(), c);
    }
    //debug
    // for a in alpha.chars() {
    //     print!("{}:{}   ", a, map.get(&a).unwrap());
    // }
    let mut result = String::new();
    for c in phrase.chars() {
        if map.contains_key(&c) {
            result.push(map.get(&c).unwrap().clone())
        } else {
            result.push(c)
        }
    }
    print!("Decrypted message:\n{}", result);
}
