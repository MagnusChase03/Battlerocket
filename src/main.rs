mod rockets;

// Creates a 3d player gameboard
fn make_board() -> [[[i32; 10]; 10]; 10] {
    let board: [[[i32; 10]; 10]; 10] = [[[0; 10]; 10]; 10];

    board
}

// Places a rocket with only one node
fn place_simple_rocket(game_board: &mut [[[i32; 10]; 10]; 10]) {

    println!("Enter x y z: ");

    let mut position: String = String::new();
    std::io::stdin().read_line(&mut position).unwrap();

    let cords = position.split(" ").collect::<Vec<&str>>();
    let x = cords[0].parse::<usize>().unwrap();
    let y = cords[0].parse::<usize>().unwrap();
    let z = cords[0].parse::<usize>().unwrap();

    game_board[x][y][z] = 1;

}

// Place rockets on board
fn place_rockets(game_board: &mut [[[i32; 10]; 10]; 10]) {
    
    place_simple_rocket(game_board);

}

fn main() {
    let mut game_board1 = make_board();
    //let mut game_board2 = make_board();

    place_rockets(&mut game_board1);
    //place_rockets(&mut game_board2);

    print!("{:?}", game_board1);
}
