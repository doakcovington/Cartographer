// WDGTR
// NCWH
// Christmas 2021
use std::io;
use std::process;

fn main() {
    menu()
}

fn menu() {
    println!("CARTOGRAPHER");
    println!("> 1. New Game");
    println!("> 2. Exit");
    println!("> Please make a selection:");

    let mut selection = String::new();

    io::stdin()
        .read_line(&mut selection)
        .expect("Failed to process selection");
    let input: u32 = selection
        .trim()
        .parse()
        .expect("Wanted 1 or 2");
    if input == 1 {
        start_game();
    } else {
        process::exit(1);
    }
}

fn start_game() {
    game_board();
}

fn game_board() {
    let mut game_state = ["","","","","","","","",""];
    let mut top_row = println!("[ ][ ][ ]");
    let mut middle_row = println!("[ ][ ][ ]");
    let mut bottom_row = println!("[ ][ ][ ]");
}
