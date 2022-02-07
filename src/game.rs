pub const MAX_TILE_NUMBER: u8 = 14;
pub const JOKER_COUNT: u8 = 2;

//[u8; 15] = [1,2,3,4,5,6,7,8,9,10,11,12,13,14];


#[derive(Debug)]
pub enum TileColor {
    Blue,
    Red,
    Yellow,
    Black,
}

#[derive(Debug)]
pub struct Tile {
    color: TileColor,
    number: u8,
}

impl Tile {
}

pub struct Rack {
    tiles: Vec<Tile>,
}

pub struct Pool {
    tiles: Vec<Tile>,
}

impl Pool {
    fn new() -> Pool {
        let mut pool = Pool { tiles: Vec::new() };

        for tile_number in 1..MAX_TILE_NUMBER {
            pool.tiles.push(Tile{ color: TileColor::Black, number: tile_number, });
        }
        pool
    }
}

pub struct Table {
    sets: Vec<Set>,
}

pub struct Set {
    tiles: Vec<Tile>,
}

#[cfg(test)]
mod tests {
    use super::*;

}