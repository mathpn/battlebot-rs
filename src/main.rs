use ndarray::{Array2, s};
use rand::Rng;


const SHIP_SIZES: [usize; 5] = [1, 3, 3, 4, 5];

pub fn main() -> () {
    print!("wellcome to battleship\n");
    let mut board = create_board(10);
    println!("{:?}", board);
    place_ships(&mut board, &SHIP_SIZES);
    println!("{:?}", board);
}

fn create_board(size: usize) -> Array2<u8> {
    let board = Array2::zeros((size, size));
    return board;
}

fn place_ships<'a>(board: &'a mut Array2<u8>, ship_sizes: &'a[usize]) -> &'a mut Array2<u8> {
    let board_size: usize = board.dim().0;
    for ship_size in ship_sizes {
        let mut x: usize = 0;
        let mut y: usize = 0;
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

fn validate_ship(board: &Array2<u8>, x: usize, y: usize, ship_size: &usize, orientation: u8) -> bool {
    let board_size: usize = board.dim().0;
    match orientation {
        0 if y + ship_size > board_size => false,
        1 if x + ship_size > board_size => false,
        0 if board.slice(s![x, y..y+ship_size]).iter().any(|x| x != &0) => false,
        1 if board.slice(s![x..x+ship_size, y]).iter().any(|x| x != &0) => false,
        _ => true,
    }
}

fn insert_ship(board: &mut Array2<u8>, x: usize, y: usize, size: usize, orientation: u8) -> &mut Array2<u8> {
    if orientation == 0 {
        // horizontal
        let mut slice = board.slice_mut(s![x, y..y+size]);
        slice += 1;
    } else {
        // vertical
        let mut slice = board.slice_mut(s![x..x + size, y]);
        slice += 1;
    };
    return board;
}
