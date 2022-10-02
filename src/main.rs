mod rockets;

// Creates a 3d player gameboard
fn make_board() -> [[[i32; 10]; 10]; 10] {
    let board: [[[i32; 10]; 10]; 10] = [[[0; 10]; 10]; 10];

    board
}

// TODO Make function to just create a cordinate from system input for code reuse

// Get the origin the ship should be placed at
fn get_orgin(rocket: &mut rockets::Rocket) {
    println!("Enter origin for {} node rocket (x y z): ", rocket.size);

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
    println!("Enter direction for {} node rocket (x y z): ", rocket.size);

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
        game_board[node.2][node.1][node.0] = 1;
    }
}

// Create and place rocket
fn create_rocket(game_board: &mut [[[i32; 10]; 10]; 10], size: i32) -> rockets::Rocket {
    let mut rocket = rockets::Rocket::new(0, 0, 0, size);
    get_orgin(&mut rocket);

    if size > 1 {
        get_direction(&mut rocket);
    }

    place_rocket(game_board, &mut rocket);

    rocket
}

// Place rockets on board
fn place_rockets(game_board: &mut [[[i32; 10]; 10]; 10]) -> Vec<rockets::Rocket> {
    let mut placed_rockets: Vec<rockets::Rocket> = Vec::new();

    placed_rockets.push(create_rocket(game_board, 1));
    placed_rockets.push(create_rocket(game_board, 2));
    placed_rockets.push(create_rocket(game_board, 3));
    placed_rockets.push(create_rocket(game_board, 4));
    placed_rockets.push(create_rocket(game_board, 5));

    placed_rockets
}

// Check if all player rockets are destroyed
fn player_dead(player_rockets: &Vec<rockets::Rocket>, game_board: &[[[i32; 10]; 10]; 10]) -> bool {
    for rocket in player_rockets {
        if !rocket.is_dead(game_board) {
            return false;
        }
    }

    true
}

// Let player attack spot on enemy board
fn player_attack(game_board: &mut [[[i32; 10]; 10]; 10]) -> bool {

    println!("Enter cords to attack (x y z): ");

    let mut position: String = String::new();
    std::io::stdin().read_line(&mut position).unwrap();

    // Remove new line and return carage
    position.pop();
    position.pop();

    let cords = position.split(" ").collect::<Vec<&str>>();
    let x = cords[0].parse::<usize>().unwrap();
    let y = cords[1].parse::<usize>().unwrap();
    let z = cords[2].parse::<usize>().unwrap();

    if game_board[z][y][x] == 1 {

        game_board[z][y][x] = 0;
        return true;

    }

    false

}

fn main() {
    let mut game_board1 = make_board();
    let mut game_board2 = make_board();

    println!("Player 1: Place rockets: ");
    let player1_rockets = place_rockets(&mut game_board1);

    println!("===================================");
    print!("Player 2: Place rockets: ");
    let player2_rockets = place_rockets(&mut game_board2);

    let mut player_turn = 1;
    while !player_dead(&player1_rockets, &game_board1)
    && !player_dead(&player2_rockets, &game_board2)
    {
        
        println!("===================================");
        print!("Player {}: ", player_turn);
        println!("{:?}", game_board1);
        if player_turn == 1 {

            player_attack(&mut game_board2);
            player_turn = 2;

        } else {

            player_attack(&mut game_board1);
            player_turn = 1;

        }

    }
}
