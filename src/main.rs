use std::{ self, io::Write };
use std::fmt;
use rand::Rng;
use clearscreen::clear;

const VERSION: &str = "1.0.0";
const DEBUG_BUILD: bool = cfg!(debug_assertions);

#[derive(Clone, Copy, PartialEq)]
enum RolesAvailable {
    X,
    O
}

impl std::fmt::Display for RolesAvailable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RolesAvailable::X => write!(f, "X"),
            RolesAvailable::O => write!(f, "O"),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum SquareState {
    Empty,
    FulfilledBy(RolesAvailable)
}

#[derive(Clone, Copy)]
struct Square {
    current_state: SquareState
}

impl std::fmt::Display for SquareState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SquareState::Empty => write!(f, " "),
            SquareState::FulfilledBy(role) => write!(f, "{}", role),
        }
    }
}

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.current_state)
    }
}

fn show_round(r: &Round) {

    clear().unwrap();
    debug_println("Executing show_round function");

    print!(
r#"Tic tac toe
made by whitesu!

-> Game round: {}
-> Next turn: {}
     _________
    |_|_1_2_3_|
    |A| {} {} {} |
    |B| {} {} {} |
    |C| {} {} {} |
     ^^^^^^^^^
"#,
        r.rows,
        r.next_turn,
        r.round_table[0][0],
        r.round_table[0][1],
        r.round_table[0][2],
        r.round_table[1][0],
        r.round_table[1][1],
        r.round_table[1][2],
        r.round_table[2][0],
        r.round_table[2][1],
        r.round_table[2][2],
    );

    debug_println("Finished Executing show_round");
}

enum InputError<'a> {
    XNotValid(&'a str),
    YNotValid(&'a str)
}

impl<'a> fmt::Display for InputError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InputError::YNotValid(msg) => write!(f, "{}", msg),
            InputError::XNotValid(msg) => write!(f, "{}", msg)
        }
    }
}

fn new_round(rows: u32, first: RolesAvailable) -> Round {
    let new_round_table = [[Square { current_state: SquareState::Empty }; 3]; 3];
    Round {
        rows: rows + 1,
        next_turn: first,
        round_table: new_round_table
    }
}

fn update_round<'a>(x: u32, y: &str, player: RolesAvailable, round: &mut Round) -> core::result::Result<(), InputError<'a>> {

    debug_println("Executing update_round function");
    if x > 3 {
        return Err(InputError::XNotValid("X must not be higher than 3"));
    }
    if x == 0 {
        return Err(InputError::XNotValid("X must not be 0"));
    }

    match y {
        "A" => round.round_table[0][(x - 1) as usize].current_state = SquareState::FulfilledBy(player),
        "B" => round.round_table[1][(x - 1) as usize].current_state = SquareState::FulfilledBy(player),
        "C" => round.round_table[2][(x - 1) as usize].current_state = SquareState::FulfilledBy(player),
        _ => return Err(InputError::YNotValid("Y must be A, B, or C")),
    }

    match player {
        RolesAvailable::X => round.next_turn = RolesAvailable::O,
        RolesAvailable::O => round.next_turn = RolesAvailable::X
    }
 
    debug_println("Finished execution of update_round");
    Ok(())
}

#[derive(Clone, Copy)]
struct Round {
    rows: u32,
    next_turn: RolesAvailable,
    round_table: [[Square; 3]; 3]
}

fn input(question: &str) -> String {
    print!("{}", question);
    std::io::stdout().flush().unwrap();

    let mut user_input = String::new();
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Error trying to read standard input");

    user_input.trim().to_string()
}

fn ask_next<'a>(round: &mut Round) -> core::result::Result<(), InputError<'a>> {

    debug_println("Executing ask_next function");

    match round.next_turn {
        RolesAvailable::X => {
            let x = input("X coordinate: ");
            let y_input = input("Y coordinate: ");
            let y = y_input.as_str();

            let x_u32 = match x.parse::<u32>() {
                Ok(num) => num,
                Err(_) => return Err(InputError::XNotValid("X is not valid, it should be a number between 1 and 3"))
            };
            update_round(x_u32, y, RolesAvailable::X, round)?;
            show_round(round);
            debug_println("Finished executing ask_next function");
            Ok(())
        },
        RolesAvailable::O => {
            let x = input("X coordinate: ");
            let y_input = input("Y coordinate: ");
            let y = y_input.as_str();

            let x_u32 = match x.parse::<u32>() {
                Ok(num) => num,
                Err(_) => return Err(InputError::XNotValid("X is not valid, it should be a number between 1 and 3"))
            };
            update_round(x_u32, y, RolesAvailable::O, round)?;
            
            show_round(round);
            debug_println("Finished executing ask_next function");
            Ok(())
        }
    }
}

fn debug_println(msg: impl AsRef<str>) {
    if DEBUG_BUILD {
        println!("{}", msg.as_ref());
    }
}

fn main_loop(round: &mut Round) {
    
    debug_println("Executing main loop");
    show_round(round);

    let mut iterations = 0;
    loop {
        if iterations >= 9 {
            println!("Draw!");
            break;
        }
        match ask_next(round) {
            Ok(_) => {
                iterations += 1;
                if let Some(player) = check_winner(round.round_table) {
                    println!("We have a winner!: {}", player);
                    break;
                }
                debug_println(format!("Finished {} iteration of main loop", iterations));
                continue;
            },
            Err(msg) => {
                iterations += 1;
                debug_println(format!("Error at {} iteration of main loop", iterations));
                println!("Error: {}", msg);
                continue;
            }
        }
    }
}

fn check_winner(board: [[Square; 3]; 3]) -> Option<RolesAvailable> {

    for row in board {
        if let SquareState::FulfilledBy(player) = row[0].current_state {
            if row[0].current_state == row[1].current_state && row[1].current_state == row[2].current_state {
                return Some(player);
            }
        }
    }

    for col in 0..3 {
        if let SquareState::FulfilledBy(player) = board[0][col].current_state {
            if board[0][col].current_state == board[1][col].current_state && board[1][col].current_state == board[2][col].current_state {
                return Some(player);
            }
        }
    }

    if let SquareState::FulfilledBy(player) = board[0][0].current_state {
        if board[0][0].current_state == board[1][1].current_state && board[1][1].current_state == board[2][2].current_state {
            return Some(player);
        }
    }

    if let SquareState::FulfilledBy(player) = board[0][2].current_state {
        if board[0][2].current_state == board[1][1].current_state && board[1][1].current_state == board[2][0].current_state {
            return Some(player);
        }
    }

    None

}

fn main() {

    debug_println(format!("Ver: {}", VERSION));
    debug_println("Started execution of main");

    let turn = if rand::thread_rng().gen_range(1..=2) == 1 {
        RolesAvailable::X 
    } else {
        RolesAvailable::O
    };

    loop {

        let mut round = new_round(0, turn);
        main_loop(&mut round);
        if input("Want to start again?\n")
            .starts_with(['y', 'Y']) {
            continue;
        } else {
            break;
        }
    }
    debug_println("Finished execution of main");
}
