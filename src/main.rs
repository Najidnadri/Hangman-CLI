
use std::io::{self, Read};
use std::fs::File;
use rand::{thread_rng, Rng};
use std::{thread, time};
use colour::{self, green_ln, red_ln};

struct Game {
    words: String,
    words_char: Vec<char>,
    answered_char: Vec<char>,
    
}

fn into_dash(s: &String) -> String {
    s
    .chars()
    .map(|x| match x {
        ' ' => return x,
        _ => return '-',
    })
    .collect()
}

fn show_char(char: &Vec<char>, string: String) -> String {
    string
    .chars()
    .map(|x| if char.contains(&x) {
        return x
    } else {
        return '-'
    })
    .collect()
}

fn random_word() -> String {
    let mut f  = File::open("words.txt").expect("file words.txt not found"); 
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).expect("cannot read to string");
    let vec_words: Vec<&str> = buffer.split(' ').collect();
    let index = thread_rng().gen_range(0 .. vec_words.len());
    vec_words[index].to_string()
}

fn setup() {
    println!("lets play hangman game!");
    thread::sleep(time::Duration::from_secs(1));
    println!("the words consist only lowerletter case");
    thread::sleep(time::Duration::from_secs(1));
    println!("please input only one char at a time");
    thread::sleep(time::Duration::from_secs(1));
    println!("type 'quit' to quit the game pussy");
    thread::sleep(time::Duration::from_secs(1));
    println!("Ready?");
    thread::sleep(time::Duration::from_secs(1));
    for i in (1 .. 4).rev() {
        println!("The game will start in {}", i);
        thread::sleep(time::Duration::from_secs(1));
    }
    red_ln!("GO!");
}

fn main() {
    setup();

    let string = random_word();
    let mut all_char: Vec<char> = string.chars().collect();
    all_char.dedup();
    let mut answered_char: Vec<char> = vec![];
    //let mut answered = String::new();
    let mut showed_string = into_dash(&string);
    //println!("{}", showed_string);
    let mut chances = 5;

    loop {
        println!("{}", showed_string);
        //check if win 
        if answered_char.len() == all_char.len() {
            green_ln!("YOU HAVE WON THE GAME. CONGRATS");
            break;
        }
        if chances == 0 {
            red_ln!("YOU LOSE!");
            break;
        }
        println!("You got {} chances left", chances);
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();

        //check quit
        if input == "quit".to_string() {
            println!("bye bye");
            break;
        }

        //check input is legit
        
        let vec: Vec<char> = input.chars().collect();
        if vec.len() > 1 || vec.len() < 1 || vec[0].is_ascii_uppercase() || vec[0].is_ascii_whitespace() {
            red_ln!("Please input only 1 lowercase character");
            continue;
        }
        if answered_char.contains(&vec[0]) {
            red_ln!("It is a repitition!");
            chances -= 1;
            continue;
        }
        if string.contains(vec[0]) {
            green_ln!("Correct!");
            answered_char.push(vec[0]);
            showed_string = show_char(&answered_char, string.clone());
            continue;
        } else {
            red_ln!("False!");
            chances -= 1;
        }
        

    }
}