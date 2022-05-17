use std::{collections::HashMap, fs, hash::Hash, io, time::Instant, sync::{Mutex, Arc}};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

// Use threads to process multiple instances of Get_best_guess() concurrently.

fn main() {
    let start = Instant::now();
    // Enable to run some diagnostic tests
    // compute_avg_tries_to_solve();
    run_game();
    // println!("{}", get_best_guess_par(&get_all_possible_words()));
    // compute_avg_tries_to_solve();
    
    println!("{}s", start.elapsed().as_secs_f64());
}

fn get_all_possible_words() -> Vec<String> {
    let contents: String = fs::read_to_string("src/words.txt").expect("Could not read file");
    let word_list: Vec<String> = contents.split("\n").map(|s| s.trim().to_string()).collect();
    return word_list;
}

fn compare_guess(guess: &String, word: String) -> Vec<u8> {
    let mut response: Vec<u8> = vec![3u8, 3u8, 3u8, 3u8, 3u8];
    let mut char_used: HashMap<char, u8> = HashMap::new();

    for (i, el) in guess.chars().enumerate() {
        if &guess[i..i + 1] == &word[i..i + 1] {
            response[i] = 2u8;

            if char_used.contains_key(&el) {
                char_used.insert(el, char_used.get_key_value(&el).unwrap().1 + 1);
            } else {
                char_used.insert(el, 1);
            }
        } else if !word.contains(el) {
            response[i] = 0u8;
        }
    }
    for (i, el) in guess.chars().enumerate() {
        if response[i] == 3u8 {
            let occurance_in_word = word.chars().filter(|s| s.to_owned() == el).count();
            if !char_used.contains_key(&el)
                || occurance_in_word < char_used.get(&el).unwrap().to_owned().into()
            {
                response[i] = 1u8;

                if char_used.contains_key(&el) {
                    char_used.insert(el, char_used.get_key_value(&el).unwrap().1 + 1);
                } else {
                    char_used.insert(el, 1);
                }
            } else {
                response[i] = 0u8;
            }
        }
    }

    return response;
}

fn filter(word_list: &Vec<String>, guess: String, feedback: &Vec<u8>) -> Vec<String> {
    let word_arr = word_list
        .to_owned()
        .into_iter()
        .filter(|word| -> bool {
            let mut yellow_pos: HashMap<char, usize> = HashMap::new();
            let word_bytes = word.as_bytes();

            for (i, el) in guess.chars().enumerate() {
                if feedback[i] == 0 && el == word_bytes[i] as char {
                    return false;
                }

                if feedback[i] == 2 && el != word_bytes[i] as char {
                    return false;
                }

                if feedback[i] == 1 && el == word_bytes[i] as char {
                    return false;
                }

                if feedback[i] == 1 {
                    if yellow_pos.contains_key(&el) {
                        yellow_pos.insert(el, yellow_pos.get(&el).unwrap().to_owned() + 1);
                    } else {
                        yellow_pos.insert(el, 1);
                    }
                }
            }

            for el in yellow_pos.iter() {
                let count = word.chars().filter(|c| c == el.0).count();
                if el.1 > &count {
                    return false;
                }
            }

            true
        })
        .collect();

    word_arr
}

fn get_best_guess(word_list: &Vec<String>) -> String {
    let mut best_guess = String::new();
    let mut best_score: usize = 0;

    if word_list.len() == 1 {
        return String::from(&word_list[0]);
    }

    for (i, el) in word_list.iter().enumerate() {
        let mut buckets: HashMap<Vec<u8>, u64> = HashMap::new();
        for (n, el2) in word_list.iter().enumerate() {
            if n != i {
                let res = compare_guess(el2, el.to_string());
                *buckets.entry(res).or_insert(0) += 1;
            }
        }

        if buckets.keys().count() > best_score {
            best_score = buckets.keys().count();
            best_guess = el.to_owned();
        }
    }

    best_guess

}

// Really cpu-expensive but fast
fn get_best_guess_par(word_list: &Vec<String>) -> String {
    let mut best_guess = Arc::new(Mutex::new(String::new()));
    let mut best_score = Arc::new(Mutex::new(usize::from(0u8))); 

    if word_list.len() == 1 {
        return String::from(&word_list[0]);
    }

    word_list.into_par_iter().for_each(|x| {
        // let mut buckets: HashMap<Vec<u8>, u64> = HashMap::new();
        let mut buckets: Arc<Mutex<HashMap<Vec<u8>, u64>>> = Arc::new(Mutex::new(HashMap::new()));

        word_list.into_par_iter().for_each(|y| {
            let res = compare_guess(y, x.to_string());
            *buckets.lock().unwrap().entry(res).or_insert(0) += 1;
        });

        if buckets.lock().unwrap().keys().count() > *best_score.lock().unwrap()  {
            *best_guess.lock().unwrap() = x.to_owned() ;
            *best_score.lock().unwrap() = buckets.lock().unwrap().keys().count();
        }

    });

    let x = &*best_guess.lock().unwrap(); 
    x.to_string()
}


//TODO fix this before git upload and add some tests

fn run_game() {
    let mut word_list = get_all_possible_words();
    // *Precomputed* most performant word in terms of getting the most possible data
    let mut current_guess = String::from("tares");
    let mut counter = 1;
    let mut feedback: Vec<u8> = Vec::new();

    loop {
        if feedback == vec![2u8, 2u8, 2u8, 2u8, 2u8] {
            break;
        }
        println!("Guess the word {}", current_guess);
        println!("Enter feedback (2=Green 1=Yellow 0=Red)");

        let mut response = String::new();
        io::stdin()
            .read_line(&mut response)
            .expect("Did not understand your response, exiting program");

        feedback = response
            .trim()
            .replace(" ", "")
            .chars()
            .map(|c| c as u8 - 48)
            .collect();
        println!("{:?}", feedback);
        word_list = filter(&word_list, current_guess, &feedback);
        println!("List Size: {}", word_list.len());
        // print!("{:?}", word_list);
        current_guess = get_best_guess_par(&word_list);
        println!("current guess is {}", current_guess);
        counter += 1;
    }

    println!(
        "Congrats the correct word was {}, which we got in {} tries.",
        current_guess, counter
    );
}

fn compute_avg_tries_to_solve() {
    let mut avg_time: u128 = 0;
    let mut avg_attempts: usize = 0;
    let number_of_attempts: usize = 100;
    let word_list = get_all_possible_words()[0..number_of_attempts].to_vec();
    let mut counter = 0;

    for el in word_list {
        let mut inner_word_list = get_all_possible_words();
        let mut current_guess = String::from("tares");
        let mut attempts: usize = 0;
        let mut feedback: Vec<u8> = Vec::new();
        let start_time = Instant::now();
        println!("Using {} as the correct word", el);
        loop {
            if feedback == vec![2u8, 2u8, 2u8, 2u8, 2u8] {
                break;
            }

            feedback = compare_guess(&current_guess, el.to_string());
            inner_word_list = filter(&inner_word_list, current_guess, &feedback);
            current_guess = get_best_guess(&inner_word_list);
            attempts += 1;

            if attempts >= 10 || current_guess == String::from("") {
                println!(
                    "Error handling attempt #{} word: {}",
                    attempts,
                    el.to_string()
                );
            }
        }
        let time_elapsed = start_time.elapsed().as_millis();
        avg_time += time_elapsed;
        avg_attempts += attempts;

        counter += 1;

        println!(
            "Finished attempt #{}:{} with {} tries, using {}ms",
            counter, current_guess, attempts, time_elapsed
        );
    }

    avg_time = avg_time / number_of_attempts as u128;
    avg_attempts = avg_attempts / number_of_attempts;

    println!("Average time per word {}ms", avg_time);
    println!("Average # of attempts per word {}", avg_attempts);
}

#[test]
fn check_compare_guess() {
    assert_eq!(
        compare_guess(&String::from("title"), String::from("tares")),
        vec![2, 0, 0, 0, 1]
    )
    //TODO setup more compare guess tests
}

#[test]
fn check_filtering() {
    // TODO setup similar filter tests
    assert_eq!(
        filter(
            &get_all_possible_words(),
            String::from("tares"),
            &vec![2u8, 0u8, 1u8, 0u8, 2u8]
        )
        .len(),
        39
    )
}
