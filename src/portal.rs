pub struct Portal {
    pub tile: (i32, i32),
    pub destination: (i32, i32),
}

impl Portal {
    pub fn new(tile: (i32, i32)) -> Portal {
        Portal {
            tile: tile,
            destination: (0, 0),
        }
    }

    pub fn connect(&mut self, destination: (i32, i32)) {
        self.destination = destination;
    }
}
