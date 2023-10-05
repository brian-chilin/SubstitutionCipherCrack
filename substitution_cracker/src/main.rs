use std::env;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, Write};
use rand::{Rng, seq::SliceRandom};

fn get_dict() -> HashMap<String, u16> {
    let file = File::open("trigrams.csv").unwrap();
    let reader = io::BufReader::new(file);

    let mut result: HashMap<String, u16> = HashMap::new();

    //each line of the csv is a frequency,trigram
    for line in reader.lines() {
        let line_s = line.unwrap();
        let data: Vec<&str> = line_s.split(",").collect();

        let frequency: u32 = data[0].parse().unwrap();
        let weight = f64::log(frequency as f64, 3.31) * 10.0; // I wanted 0..=~10. Since we are using unsigned int instead of floats I'll do 0..=100

        //println!("{} >> {}", frequency, weight);
        result.insert(String::from(data[1]), weight as u16);
    }

     // println!("THE:{}", result.get(&String::from("THE")).unwrap());
     // println!("AND:{}", result.get(&String::from("AND")).unwrap());
     // println!("YOU:{}", result.get(&String::from("YOU")).unwrap());

    result
}

fn main() {
    println!("Usage: substitution_cracker \"[ciphertext_phrase]\"*\nMay break with some characters like '-'\nX)INPUT\n  confidence:key >> plaintext\n\n");
    let input: Vec<String> = env::args().collect();
    let mut cipher_texts: Vec<String> = vec!();

    //use 4 from lab instructions if no input was given
    if input.len() < 2 {
        let tests: [&str; 4] = [
            "fqjcb rwjwj vnjax bnkhj whxcq nawjv nfxdu mbvnu ujbbf nnc",
            "oczmz vmzor jocdi bnojv dhvod igdaz admno ojbzo rcvot jprvi oviyv aozmo cvooj ziejt dojig toczr dnzno jahvi fdiyv xcdzq zoczn zxjiy",
            "ejitp spawa qleji taiul rtwll rflrl laoat wsqqj atgac kthls iraoa twlpl qjatw jufrh lhuts qataq itats aittk stqfj cae",
            "iyhqz ewqin azqej shayz niqbe aheum hnmnj jaqii yuexq ayqkn jbeuq iihed yzhni ifnun sayiz yudhe sqshu qesqa iluym qkque aqaqm oejjs hqzyu jdzqa diesh niznj jayzy uiqhq vayzq shsnj jejjz nshna hnmyt isnae sqfun dqzew qiead zevqi zhnjq shqze udqai jrmtq uishq ifnun siiqa suoij qqfni syyle iszhn bhmei squih nimnx hsead shqmr udquq uaqeu iisqe jshnj oihyy snaxs hqihe lsilu ymhni tyz",
            //"COUNTERREVOLUTIONARIESCOUNTERREVOLUTIONARIES COUNTERREVOLUTIONARIESCOUNTERREVOLUTIONARIES COUNTERREVOLUTIONARIESCOUNTERREVOLUTIONARIES"
        ];
        for t in tests {
            cipher_texts.push(String::from(t));
        }
    } else {
        cipher_texts = input.clone();
        cipher_texts.remove(0);
    }

    //clean input to remove spaces and make all uppercase
    for i in 0..cipher_texts.len() {
        cipher_texts[i] = cipher_texts[i].to_uppercase().replace(" ", "");
    }


    let dictionary: HashMap<String, u16> = get_dict();
    //println!("{:#?}", &dictionary);
    //main driver code
    let mut counter = 0;
    for ct in cipher_texts {
        counter += 1;
        println!("{}){}", counter, ct);
        let results: Vec<(u32, String, String)> = crack(&ct, &dictionary);
        println!();
        for r in results {
            println!("    {}:{} >> {:?}", r.0, r.1, r.2);
        }
        println!();
    }
}


fn crack(ciphertext: &String, dictionary: &HashMap<String, u16>) -> Vec<(u32, String, String)> {
    //returns (confidence, key, plaintext)
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let mut results: Vec<(u32, String, String)> = vec![];
    let mut rng = rand::thread_rng();   // for escaping local peaks during hill climbing
    let results_len: usize = ((ciphertext.len()-1) as f64/(7.0*f64::log(ciphertext.len() as f64, 10.0))) as usize;


    //begin caesar cipher
    let k = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    for i in 1..26 { //1-25 inclusive. this entire for loop is for caesar ciphers
        //this first nested loop just makes a shifted key
        let mut key = String::new();
        for j in k.chars() {
            let mut char = j as u8 + &i;
            if char > 90 {
                char -= 26
            }
            key.push(char as char);
        }
        let plaintext = decode(&ciphertext, &key);
        //println!("{}: ", plaintext);
        results.push(
            (
                           confidence(&plaintext, &dictionary),
                           key,
                           plaintext
            )
        );
    }
    //end caesar cipher


    //begin hill climb
    for _i in 0..8 { // find 8 keys to avoid getting stuck in local maxima
        // start with a random key
        let mut key: Vec<char> = k.chars().collect();
        key.shuffle(&mut rng);
        let mut score = confidence(&decode(&ciphertext, &key.iter().collect()), &dictionary);
        //do hill climbing loop here
        for _j in 0..8192 {
            let mut next_gen = key.clone();
            //generate 2 random indices
            let rand_a: usize = rng.gen_range(0..26);
            let rand_b: usize = rng.gen_range(0..26);
            //swap those 2 random parts of the key
            let temp = next_gen[rand_b];
            next_gen[rand_b] = next_gen[rand_a];
            next_gen[rand_a] = temp;
            //if this key is better keep the changes
            let next_gen_score = confidence(&decode(&ciphertext, &next_gen.iter().collect()), &dictionary);
            if next_gen_score > score {
                score = next_gen_score;
                key = next_gen;
            }
        }
        let plaintext = &decode(&ciphertext, &key.iter().collect());
        handle.write(b"+ ").unwrap();
        handle.flush().unwrap();
        //println!("dbg;{}:{}", &score, key.iter().collect::<String>());
        results.push((score, key.iter().collect::<String>(), plaintext.clone()));
    }
    //end hill climb

    //FREQUENCY ANALYSIS
    //println!("{:?}\n{:?}\n{:?}", alph, keys, scores);
    // let mut freq_analysis: HashMap<char, u16> = HashMap::new();
    // for c in alph {
    //     freq_analysis.insert(c, 0);
    // }
    // for c in &hc {
    //     freq_analysis.insert(
    //         c.clone(),
    //         freq_analysis.get(c).unwrap() + 1
    //     );
    // }
    // println!("{:?}", freq_analysis);


    //results.sort_by(|a, b| b.0.cmp(&a.0)); //sort by matching permutations descending
    //returning
    if results.len() <= results_len {
        return results;
    }
    //find the worst key to begin finding the best ones
    let mut min_i: usize = 0; //smallest index
    for (i, result) in results.clone().into_iter().enumerate() {
        if result.0 < results[min_i].0 {
            min_i = i;
        }
    }
    //find the best keys
    let mut limited: Vec<(u32, String, String)> = vec![];
    let mut x_largest: HashSet<usize> = HashSet::new();
    for _i in 0..=results_len {
        let mut largest = min_i;
        for (j, result) in results.clone().into_iter().enumerate() {
            if result.0 > results[largest].0 && !x_largest.contains(&j) {
                largest = j;
            }
        }
        x_largest.insert(largest);
    }
    //print!("{:#?}", x_largest);
    //return the best keys
    for i in x_largest {
        limited.push(results[i].clone())
    }
    limited.sort_by(|a, b| b.0.cmp(&a.0)); //sort by matching permutations descending
    limited
}


fn confidence(plaintext: &String, dictionary: &HashMap<String, u16>) -> u32 {
    //score a word by how summing its trigrams scores
    let mut result: u32 = 0;
    for i in 0..(plaintext.len()-2) {
        let trigram = &plaintext[i..i+3];
        //print!("{} ", &trigram);
        if dictionary.contains_key(&String::from(trigram)) {
            result += *dictionary.get(&String::from(trigram)).unwrap() as u32;
        }
    }
    result
}


fn decode(ciphertext: &String, key: &String) -> String {
    //simple decoding given ciphertext and key
    let default_string = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let mut map: HashMap<char, char> = HashMap::new();
    let default: Vec<char> = default_string.chars().collect();
    let shifted: Vec<char> = key.chars().collect();
    for i in 0..26 {
        map.insert(
            shifted[i as usize],
            default[i as usize]
        );
    }
    let cipher: Vec<char> = ciphertext.chars().collect();
    let mut result = String::new();
    for c in cipher {
        if map.contains_key(&c) {
            result.push(
                map.get(&c).unwrap().clone()
            );
        } else {
            println!("map {:?} [[{}]] doesnt contain: {}", map, map.len(), &c)
        }
    }
    result
}

// fn encode(ciphertext: &String, key: &Vec<char>) -> String {
//     let mut map: HashMap<char, char> = HashMap::new();
//     let alpha = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect::<Vec<char>>();
//     for c in 0..alpha.len() {
//         map.insert(
//             alpha[c].clone(),
//             key[c].clone()
//         );
//     }
//
//     let mut input: Vec<char> = ciphertext.chars().collect();
//     for i in 0..input.len() {
//         input[i] = map.get(&input[i]).unwrap().clone();
//     }
//
//     String::from_iter(input)
// }