// WDGTR
// NCWH
// Christmas 2021
use std::io;
use std::process;
use rand::Rng;

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
    game_state();
}

fn game_state() {
    let mut game_board = ["1","2","3","4","5","6","7","8","9"];
    let mut planets = ["","","","","","","","",""];
    let mut i = 0;
    while i < game_board.len() / 3 {
        let mut rng = rand::thread_rng();
        let mut space = rng.gen_range(0..game_board.len());
        if game_board[space] != "X" {
            planets[space] = "X";
            i += 1;
        } else if  space < game_board.len() && game_board[space] != "X" {
            game_board[space + 1] = "X";
            i += 1;
        } else if  space > game_board.len() && game_board[space] != "X" {
            game_board[space + 1] = "X";
            i += 1;
        } else {
            i += 1;
        }
    }

    let mut n = 1;
    while n <= game_board.len() {
        if n % 3 != 0 {
            print!("[{}]", game_board[n - 1]);
            n += 1;
        } else {
            println!("[{}]", game_board[n - 1]);
            n += 1;
        }
    }
    space_selection();
}

fn space_selection() -> String {
    println!("Select a spot on the grid:");

    let mut selection = String::new();

    io::stdin()
        .read_line(&mut selection)
        .expect("Failed to process selection");

    return selection;
}

