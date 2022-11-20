use ndarray::{s, Array2};
use rand::Rng;
use std::io::stdin;

const SHIP_SIZES: [usize; 5] = [1, 3, 3, 4, 5];

pub fn main() -> () {
    print!("wellcome to battleship\n");
    let mut board = create_board(10);
    let mut public_board = create_board(10);
    println!("{:?}", board);
    place_ships(&mut board, &SHIP_SIZES);
    println!("{:?}", board);
    println!("public board: \n{:?}", public_board);
    update_public_board(&board, &mut public_board, 0, 0);
    update_public_board(&board, &mut public_board, 5, 4);
    update_public_board(&board, &mut public_board, 9, 0);
    println!("public board: \n{:?}", public_board);
    print_board(&public_board);
    loop {
        let x: usize;
        let y: usize;
        (x, y) = get_valid_position(&public_board);
        let hit: u8 = update_public_board(&board, &mut public_board, x, y);
        if hit == 1 {
            print!("\nYou hit one of the computer's ships!\n")
        } else {
            print!("\nIt's a miss, try again\n")
        }
        print_board(&public_board);
    }
}

fn get_input(board_size: usize, row: bool) -> usize {
    let row_number: usize;
    let prompt: &str = if row { "row" } else { "column" };
    loop {
        let mut row = String::new();
        println!("\nChoose one valid {}: ", prompt);
        stdin().read_line(&mut row).unwrap();
        let parsed_row = row.trim().parse::<usize>();
        match parsed_row {
            Result::Ok(n) if n <= board_size => {
                row_number = n;
                break;
            }
            _ => continue,
        };
    }
    return row_number - 1;
}
fn get_valid_position(public_board: &Array2<i8>) -> (usize, usize) {
    let mut row_number: usize;
    let mut col_number: usize;
    let board_size: usize = public_board.shape()[0];
    loop {
        row_number = get_input(board_size, true);
        col_number = get_input(board_size, false);
        if public_board[[row_number, col_number]] == 0 {
            break
        } else {
            print!("position was already guessed, pick another one...\n")
        }
    }
    return (row_number, col_number);
}

fn print_board(public_board: &Array2<i8>) -> () {
    let mut rows: Vec<String> = vec!["   1  2  3  4  5  6  7  8  9  10 ".to_string()];
    for (i, row) in public_board.outer_iter().enumerate() {
        let str_row: String = row
            .iter()
            .map(|x| match x {
                1 => " S ",
                -1 => " X ",
                _ => "   ",
            })
            .collect::<Vec<&str>>()
            .join("");
        let r_number = if i <= 8 {
            format!("{} ", i + 1)
        } else {
            format!("{}", i + 1)
        };
        let r_numbered = vec![r_number, str_row].join("");
        rows.push(r_numbered);
    }
    print!("\n{}\n", rows.join("\n"));
}

fn create_board(size: usize) -> Array2<i8> {
    let board = Array2::zeros((size, size));
    return board;
}

fn update_public_board(
    board: &Array2<i8>,
    public_board: &mut Array2<i8>,
    x: usize,
    y: usize,
) -> u8 {
    if board[[x, y]] == 1 {
        public_board[[x, y]] = 1;
        return 1;
    } else {
        public_board[[x, y]] = -1;
        return 0;
    }
}

fn place_ships<'a>(board: &'a mut Array2<i8>, ship_sizes: &'a [usize]) -> &'a mut Array2<i8> {
    let board_size: usize = board.dim().0;
    for ship_size in ship_sizes {
        let mut x: usize = board_size + 1;
        let mut y: usize = board_size + 1;
        let mut ori: u8 = 0;
        while !validate_ship(board, x, y, &ship_size, ori) {
            ori = rand::thread_rng().gen_range(0..2);
            x = rand::thread_rng().gen_range(0..board_size);
            y = rand::thread_rng().gen_range(0..board_size);
        }
        insert_ship(board, x, y, *ship_size, ori);
    }
    return board;
}

fn validate_ship(
    board: &Array2<i8>,
    x: usize,
    y: usize,
    ship_size: &usize,
    orientation: u8,
) -> bool {
    let board_size: usize = board.dim().0;
    match orientation {
        0 if y + ship_size > board_size => false,
        1 if x + ship_size > board_size => false,
        0 if board.slice(s![x, y..y + ship_size]).iter().any(|x| x != &0) => false,
        1 if board.slice(s![x..x + ship_size, y]).iter().any(|x| x != &0) => false,
        _ => true,
    }
}

fn insert_ship(
    board: &mut Array2<i8>,
    x: usize,
    y: usize,
    size: usize,
    orientation: u8,
) -> &mut Array2<i8> {
    if orientation == 0 {
        // horizontal
        let mut slice = board.slice_mut(s![x, y..y + size]);
        slice += 1;
    } else {
        // vertical
        let mut slice = board.slice_mut(s![x..x + size, y]);
        slice += 1;
    };
    return board;
}
