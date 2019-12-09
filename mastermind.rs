use std::collections::HashMap;
use std::io;

fn main() {
    println!("Hello, Mastermind!");

    let mut player = Player::new();

    loop {
        let guess = player.guess();

        println!("Hmm...Is {} the answer?", guess);
        println!("Please input a reply");

        let mut reply = String::new();
        io::stdin()
            .read_line(&mut reply)
            .expect("Failed to read line");

        if validate_reply(&reply).is_none() {
            continue;
        }

        // if reply == "60 { ideally I want to write this, but not knowing why even when reply is "60" false is returned..."
        if reply.contains("60") {
            println!("Yay! I guessed it!");
            break;
        }

        player.eliminate_candidates(&guess, &reply);

        println!("{}", player.candidates.len());
    }
}

fn validate_reply(reply: &String) -> Option<()> {
    let reply = reply.trim();
    if reply.chars().filter(|c| '0' <= *c && *c <= '6').count() != 2 {
        println!("Each digit in the reply must be in between 1 and 6");
        None
    } else if reply.chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>() > 6 {
        println!("Sum of 2 digits must be 6 or less than 6");
        None
    } else {
        Some(())
    }
}

struct Player {
    candidates: Vec<String>,
}

impl Player {
    fn new() -> Self {
        let mut candidates = Vec::new();
        for i in 100000..1000000 {
            candidates.push(i.to_string());
        }

        Self { candidates }
    }

    fn guess(&self) -> String {
        use rand::Rng;
        
        self.candidates[rand::thread_rng().gen_range(0, self.candidates.len())].clone()
    }

    fn eliminate_candidates(&mut self, guess: &String, reply: &String) {
        self.candidates = self
            .candidates
            .clone()
            .into_iter()
            .filter(|candidate| {
                right_num_and_right_position(guess, candidate)
                    == reply.chars().nth(0).unwrap().to_digit(10).unwrap() as isize
                && right_num_but_wrong_position(guess, candidate)
                    == reply.chars().nth(1).unwrap().to_digit(10).unwrap() as isize
            })
            .collect::<Vec<_>>();
    }
}

fn right_num_and_right_position(guess: &String, answer: &String) -> isize {
    answer
        .chars()
        .zip(guess.chars())
        .filter(|(answer, guess)| answer == guess)
        .count() as isize
}

fn right_num_but_wrong_position(guess: &String, answer: &String) -> isize {
    let mut guess_counter = HashMap::new();
    guess.chars().for_each(|c| {
        guess_counter.insert(c, guess.chars().filter(|c_inner| *c_inner == c).count());
    });

    let mut answer_counter = HashMap::new();
    answer.chars().for_each(|c| {
        answer_counter.insert(c, answer.chars().filter(|c_inner| *c_inner == c).count());
    });

    guess_counter
        .iter()
        .map(|(key, value)| std::cmp::min(value, answer_counter.get(key).unwrap_or(&0)))
        .sum::<usize>() as isize
        - right_num_and_right_position(&guess, &answer)
}

#[test]
fn test_right_num_and_right_position() {
    let pair = ("636144".into(), "600000".into());

    assert_eq!(1, right_num_and_right_position(&pair.0, &pair.1));
    assert_eq!(0, right_num_but_wrong_position(&pair.0, &pair.1));

    let pair = ("100000".into(), "100000".into());
    assert_eq!(6, right_num_and_right_position(&pair.0, &pair.1));
    assert_eq!(0, right_num_but_wrong_position(&pair.0, &pair.1));

    let pair = ("123456".into(), "654321".into());
    assert_eq!(0, right_num_and_right_position(&pair.0, &pair.1));
    assert_eq!(6, right_num_but_wrong_position(&pair.0, &pair.1));

    let pair = ("900000".into(), "999999".into());
    assert_eq!(1, right_num_and_right_position(&pair.0, &pair.1));
    assert_eq!(0, right_num_but_wrong_position(&pair.0, &pair.1));
}
