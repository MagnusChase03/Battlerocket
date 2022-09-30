mod rockets;

// Creates a 3d player gameboard
fn make_board() -> [[[i32; 10]; 10]; 10] {
    let board: [[[i32; 10]; 10]; 10] = [[[0; 10]; 10]; 10];

    board
}

// Get the origin the ship should be placed at
fn get_orgin(rocket: &mut rockets::Rocket) {

    println!("Enter cords for single node rocket (x y z): ");

    let mut position: String = String::new();
    std::io::stdin().read_line(&mut position).unwrap();

    // Remove new line and return carage
    position.pop();
    position.pop();

    let cords = position.split(" ").collect::<Vec<&str>>();
    let x = cords[0].parse::<usize>().unwrap();
    let y = cords[1].parse::<usize>().unwrap();
    let z = cords[2].parse::<usize>().unwrap();

    rocket.origin = (x, y, z);

}

// Gets the direction the ship should be placed in
fn get_direction(rocket: &mut rockets::Rocket) {

    println!("Enter direction for rocket (x y z): ");

    let mut position: String = String::new();
    std::io::stdin().read_line(&mut position).unwrap();

    // Remove new line and return carage
    position.pop();
    position.pop();

    let cords = position.split(" ").collect::<Vec<&str>>();
    let x = cords[0].parse::<usize>().unwrap();
    let y = cords[1].parse::<usize>().unwrap();
    let z = cords[2].parse::<usize>().unwrap();

    rocket.direction = Some((x, y, z));

}

// Places rocket on game board
fn place_rocket(game_board: &mut [[[i32; 10]; 10]; 10], rocket: &mut rockets::Rocket) {

    rocket.calculate_nodes();

    for node in &rocket.nodes {

        game_board[node.0][node.1][node.2] = 1;

    }

}

// Create and place rocket
fn create_rocket(game_board: &mut [[[i32; 10]; 10]; 10], size: i32) {

    let mut rocket = rockets::Rocket::new(0, 0, 0, size);
    get_orgin(&mut rocket);

    if size > 1 {

        get_direction(&mut rocket);

    }
    
    place_rocket(game_board, &mut rocket);

}

// Place rockets on board
fn place_rockets(game_board: &mut [[[i32; 10]; 10]; 10]) {
    
    create_rocket(game_board, 1);
    create_rocket(game_board, 2);
    create_rocket(game_board, 3);

}

fn main() {
    let mut game_board1 = make_board();
    //let mut game_board2 = make_board();

    place_rockets(&mut game_board1);
    //place_rockets(&mut game_board2);

    print!("{:?}", game_board1);
}
