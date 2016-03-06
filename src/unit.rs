pub struct Unit {
    pub tile: (i32, i32),
}

impl Unit {
    pub fn new(tile: (i32, i32)) -> Unit {
        Unit {
            tile: tile,
        }
    }
}
