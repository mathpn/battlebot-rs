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
    let row_number: usize = get_input(10, true);
    println!("row: {}", row_number);
    let col_number: usize = get_input(10, false);
    println!("column: {}", col_number);
    // TODO validate if coordinate is available
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
    return row_number;
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
    print!("{}", rows.join("\n"));
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
) -> () {
    if board[[x, y]] == 1 {
        public_board[[x, y]] = 1
    } else {
        public_board[[x, y]] = -1
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
