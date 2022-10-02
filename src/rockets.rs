pub struct Rocket {
    pub origin: (usize, usize, usize),
    pub size: i32,
    pub direction: Option<(usize, usize, usize)>,
    pub nodes: Vec<(usize, usize, usize)>,
}

impl Rocket {
    pub fn new(x: usize, y: usize, z: usize, size: i32) -> Self {
        Self {
            origin: (x, y, z),
            size: size,
            direction: None,
            nodes: Vec::new(),
        }
    }

    pub fn calculate_nodes(&mut self) {
        self.nodes.push(self.origin);
        if self.size > 1 {
            let mut previous_node = self.origin;
            for _i in 0..(self.size - 1) {
                let next_node = (
                    previous_node.0 + self.direction.unwrap().0,
                    previous_node.1 + self.direction.unwrap().1,
                    previous_node.2 + self.direction.unwrap().2,
                );

                self.nodes.push(next_node);
                previous_node = next_node;
            }
        }
    }

    pub fn is_dead(&self, game_board: &[[[i32; 10]; 10]; 10]) -> bool {

        for node in &self.nodes {

            if game_board[node.2][node.1][node.0] == 1 {

                return false;

            }

        }

        true

    }
}
