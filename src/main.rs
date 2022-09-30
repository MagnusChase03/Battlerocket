mod rockets;

// Creates a 3d player gameboard
fn make_board() -> [[[i32; 10]; 10]; 10] {
    let board: [[[i32; 10]; 10]; 10] = [[[0; 10]; 10]; 10];

    board
}

// Places a rocket with only one node
fn place_node_rocket(game_board: &mut [[[i32; 10]; 10]; 10]) {

    println!("Enter cords for single node rocket (x y z): ");

    let mut position: String = String::new();
    std::io::stdin().read_line(&mut position).unwrap();
    position.pop();
    position.pop();

    let cords = position.split(" ").collect::<Vec<&str>>();
    println!("{:?}",cords);
    let x = cords[0].parse::<usize>().unwrap();
    let y = cords[1].parse::<usize>().unwrap();
    let z = cords[2].parse::<usize>().unwrap();

    game_board[x][y][z] = 1;

}

// Place two node rocket
fn place_rocket(game_board: &mut [[[i32; 10]; 10]; 10]) {

    println!("Enter cords for two node rocket(x+1) (x y z): ");

    let mut position: String = String::new();
    std::io::stdin().read_line(&mut position).unwrap();
    position.pop();
    position.pop();

    let cords = position.split(" ").collect::<Vec<&str>>();
    let x = cords[0].parse::<usize>().unwrap();
    let y = cords[1].parse::<usize>().unwrap();
    let z = cords[2].parse::<usize>().unwrap();

    let rocket = rockets::Rocket {x: x, y: y, z: z};
    let second_node = rocket.next();

    game_board[x][y][z] = 1;
    game_board[second_node.0][second_node.1][second_node.2] = 1;

}

// Place rockets on board
fn place_rockets(game_board: &mut [[[i32; 10]; 10]; 10]) {
    
    place_node_rocket(game_board);
    place_rocket(game_board);

}

fn main() {
    let mut game_board1 = make_board();
    //let mut game_board2 = make_board();

    place_rockets(&mut game_board1);
    //place_rockets(&mut game_board2);

    print!("{:?}", game_board1);
}
