use std::{collections::HashMap, fs, io, time::Instant};

// Use threads to process multiple instances of Get_best_guess() concurrently.

fn main() {
    // Enable to run some diagnostic tests
    // compute_avg_tries_to_solve(); 
    run_game();
}

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
        current_guess = get_best_guess(&word_list);
        println!("current guess is {}", current_guess);
        counter += 1;
    }

    println!(
        "Congrats the correct word was {}, which we got in {} tries.",
        current_guess, counter
    );
}

fn compare_guess(guess: &String, word: String) -> Vec<String> {
    let mut response: Vec<String> = vec![
        String::from(""),
        String::from(""),
        String::from(""),
        String::from(""),
        String::from(""),
    ];

    let mut char_used: HashMap<char, u8> = HashMap::new();

    guess.chars().enumerate().for_each(|(i, el)| {
        if &guess[i..i + 1] == &word[i..i + 1] {
            response[i] = String::from("2");

            if char_used.contains_key(&el) {
                char_used.insert(el, char_used.get_key_value(&el).unwrap().1 + 1);
            } else {
                char_used.insert(el, 1);
            }
        } else if !word.contains(el) {
            response[i] = String::from("0");
        }
    });

    for (i, el) in guess.chars().enumerate() {
        if response[i] == String::from("") {
            let occurance_in_word = word.chars().filter(|s| s.to_owned() == el).count();
            if !char_used.contains_key(&el)
                || occurance_in_word < char_used.get(&el).unwrap().to_owned().into()
            {
                response[i] = String::from("1");

                if char_used.contains_key(&el) {
                    char_used.insert(el, char_used.get_key_value(&el).unwrap().1 + 1);
                } else {
                    char_used.insert(el, 1);
                }
            } else {
                response[i] = String::from("0");
            }
        }
    }

    return response;
}

fn get_all_possible_words() -> Vec<String> {
    let contents: String = fs::read_to_string("src/words.txt").expect("Could not read file");
    let word_list: Vec<String> = contents.split("\n").map(|s| s.trim().to_string()).collect();
    return word_list;
}

fn get_best_guess(word_list: &Vec<String>) -> String {
    if word_list.len() == 1 {
        return word_list.get(0).unwrap().to_owned();
    }

    let mut buckets: HashMap<String, usize> = HashMap::new();

    // println!("{}", word_list.len());

    for el in word_list.iter() {
        let assumed_correct = el;
        let mut temp_diff_scores: Vec<Vec<String>> = Vec::new();

        for el2 in word_list.iter() {
            let other_word = el2;

            if assumed_correct != other_word {
                let temp_score: Vec<String> =
                    compare_guess(other_word, assumed_correct.to_string());
                temp_diff_scores.push(temp_score);
            }
        }

        buckets.insert(assumed_correct.to_string(), temp_diff_scores.len());
    }

    let mut best_word = String::from("");
    let mut best_score: usize = 0;

    for el in word_list.iter() {
        let temp_score = buckets.get(el).unwrap();
        if temp_score > &best_score {
            best_word = el.to_string();
            best_score = *temp_score;
        }
    }
    best_word
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

fn process_guess_response(guess_resp: Vec<String>) -> Vec<u8> {
    let mut processed_resp: Vec<u8> = Vec::new();

    for el in guess_resp {
        // 48: 0, 49: 1, 50: 2
        processed_resp.push(el.parse().unwrap());
    }

    processed_resp
}

fn compute_avg_tries_to_solve() {
    let mut avg_time: u128 = 0;
    let mut avg_attempts: usize = 0;
    let number_of_attempts: usize = 10;
    let word_list = get_all_possible_words()[0..number_of_attempts].to_vec();
    let mut counter = 0;

    for el in word_list {
        let mut inner_word_list = get_all_possible_words();
        let mut current_guess = String::from("tares");
        let mut attempts: usize = 0;
        let mut feedback: Vec<u8> = Vec::new();
        let start_time = Instant::now();
        loop {
            if feedback == vec![2u8, 2u8, 2u8, 2u8, 2u8] {
                break;
            }

            feedback = process_guess_response(compare_guess(&current_guess, el.to_string()));
            inner_word_list = filter(&inner_word_list, current_guess, &feedback);
            current_guess = get_best_guess(&inner_word_list);
            attempts += 1;

            if attempts >=  10 || current_guess == String::from("") {
                println!("Error handling attempt #{} word: {}", attempts,el.to_string());
            }
        }
        let time_elapsed = start_time.elapsed().as_millis();
        avg_time += time_elapsed;
        avg_attempts += attempts;

        counter += 1;

        println!(
            "Finished attempt #{} with {} tries, using {}ms",
            counter, attempts, time_elapsed
        );
    }

    avg_time = avg_time / number_of_attempts as u128;
    avg_attempts = avg_attempts / number_of_attempts;

    println!("Average time per word {}", avg_time);
    println!("Average # of attempts per word {}", avg_attempts);
}
