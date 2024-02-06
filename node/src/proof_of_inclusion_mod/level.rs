/// Represents a level or row in a Merkle tree.
#[derive(Debug)]
pub struct Level {
    left: Option<Vec<u8>>,
    right: Option<Vec<u8>>,
}

impl Level {
    pub fn new(left: Option<Vec<u8>>, right: Option<Vec<u8>>) -> Level {
        Level { left, right }
    }

    pub fn get_left(&self) -> &Option<Vec<u8>> {
        &self.left
    }

    pub fn get_right(&self) -> &Option<Vec<u8>> {
        &self.right
    }

    pub fn set_left(&mut self, left: Option<Vec<u8>>) {
        self.left = left
    }

    pub fn set_right(&mut self, right: Option<Vec<u8>>) {
        self.right = right
    }
}
