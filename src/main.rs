use rand::Rng;
use std::{thread, time};

fn main() {
    loop {
        let options: [&str; 3] = ["Kivi", "Sakset", "Paperi"];

        clearscreen::clear().expect("virhe");

        println!("Valitse: [k] Kivi [p] Paperi [s] Sakset");

        // get user inputt
        let mut mode_input: String = String::new();

        std::io::stdin().read_line(&mut mode_input).expect("virhe");
        mode_input = mode_input.to_lowercase();

        let char_vec: Vec<char> = mode_input.chars().collect();

        let mode: char = char_vec[0];

        if mode != 'k' && mode != 's' && mode != 'p' {
            continue;
        }

        // random thingf

        let random_number: usize = rand::thread_rng().gen_range(0..=2);

        clearscreen::clear().expect("virhe");

        let mode: usize = if mode == 'k' {
            0
        } else if mode == 's' {
            1
        } else {
            2
        };

        anim();

        if mode == random_number {
            println!("Tasapeli!");
        } else if mode == 0 && random_number == 1 {
            println!("Voitit!");
        } else if mode == 0 && random_number == 2 {
            println!("Hävisit!");
        } else if mode == 1 && random_number == 0 {
            println!("Hävisit!");
        } else if mode == 1 && random_number == 2 {
            println!("Voitit!");
        } else if mode == 2 && random_number == 0 {
            println!("Voitit!");
        } else if mode == 2 && random_number == 1{
            println!("Hävisit!");
        } else {
            println!("Tapahtui virhe");
        }

        println!(
            "Sinä valitsit {}, kone valitsi {}",
            options[mode], options[random_number]
        );

        thread::sleep(time::Duration::from_millis(1000));

        let mode: char = ask_for_input();
        if mode == 'y' {
            continue;
        } else {
            clearscreen::clear().expect("virhe");
            break;
        }
    }
}

fn ask_for_input() -> char {
    loop {
        println!("Haluatko pelata uudestaan [y] kyllä [n] ei");

        let mut mode: String = String::new();

        std::io::stdin().read_line(&mut mode).expect("virhe");
        mode = mode.to_lowercase();
        let char_vec: Vec<char> = mode.chars().collect();
        let mode: char = char_vec[0];
        if mode == 'y' {
            return 'y';
        } else if mode == 'n' {
            return 'n';
        } else {
            clearscreen::clear().expect("virhe");
            continue;
        }
    }
}

fn anim() {
    clearscreen::clear().expect("virhe");
    println!("K");
    thread::sleep(time::Duration::from_millis(200));

    clearscreen::clear().expect("virhe");
    println!("K .");
    thread::sleep(time::Duration::from_millis(200));

    clearscreen::clear().expect("virhe");
    println!("K . .");
    thread::sleep(time::Duration::from_millis(200));

    clearscreen::clear().expect("virhe");
    println!("K . . P");
    thread::sleep(time::Duration::from_millis(200));

    clearscreen::clear().expect("virhe");
    println!("K . . P .");
    thread::sleep(time::Duration::from_millis(200));

    clearscreen::clear().expect("virhe");
    println!("K . . P . .");
    thread::sleep(time::Duration::from_millis(200));

    clearscreen::clear().expect("virhe");
    println!("K . . P . . S");
    thread::sleep(time::Duration::from_millis(200));

    clearscreen::clear().expect("virhe");
}
