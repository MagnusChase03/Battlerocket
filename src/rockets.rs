pub struct Rocket {

    pub x: usize,
    pub y: usize,
    pub z: usize

}

impl Rocket {

    pub fn next(&self) -> (usize, usize, usize) {

        (self.x + 1, self.y, self.z)

    }

}