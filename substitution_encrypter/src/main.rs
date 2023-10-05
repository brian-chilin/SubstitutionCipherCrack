use std::env;
use std::collections::HashMap;

fn main() {
    println!("Usage: substitution_encrypter [26capital_letter_key] \"[plaintext_phrase]\"");
    let mut iter = env::args();
    iter.next(); //throw away garbage: the first substitution_encrypter.exe in the command
    //Store key and phrase in separate String variables
    let key = iter.next().unwrap();
    let phrase = iter.next().unwrap().to_uppercase(); //println!("{} {}", key, phrase);
    //Store alphabet to map from
    let alpha = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    //Prepare to generate mapping by getting iterator of key string
    let mut k = key.chars().into_iter();
    let mut map: HashMap<char, char> = HashMap::new();
    //Use hashmap to map from original character to substitute character
    for c in alpha.chars() {
        map.insert(c, k.next().unwrap());
    }
    //debug
    // for c in alpha.chars() {
    //     println!("{}: {}", c, map.get(&c).unwrap());
    // }
    let mut result = String::new();
    //Use the map to make plaintext into ciphertext
    for c in phrase.chars() {
        if map.contains_key(&c) {
            result.push(map.get(&c).unwrap().clone())
        } else {
            //println!("missing character '{}'", c);
            result.push(c)
        }
    }
    print!("Encryption:\n{}", result);
}
