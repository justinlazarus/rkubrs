use rand::Rng;
use rand::thread_rng;

pub const MAX_TILE_NUMBER: u8 = 13;
pub const JOKER_COUNT: u8 = 2;


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

pub struct Rack {
    tiles: Vec<Tile>,
}

pub struct Pool {
    tiles: Vec<Tile>,
}

impl Pool {
    fn new() -> Pool {
        let mut pool = Pool { tiles: Vec::new() };

        // Add 2 tiles for each number/color

        for tile_number in 1..(MAX_TILE_NUMBER+1) {
            pool.tiles.push(Tile{ color: TileColor::Black, number: tile_number, });
            pool.tiles.push(Tile{ color: TileColor::Black, number: tile_number, });
            pool.tiles.push(Tile{ color: TileColor::Red, number: tile_number, });
            pool.tiles.push(Tile{ color: TileColor::Red, number: tile_number, });
            pool.tiles.push(Tile{ color: TileColor::Yellow, number: tile_number, });
            pool.tiles.push(Tile{ color: TileColor::Yellow, number: tile_number, });
            pool.tiles.push(Tile{ color: TileColor::Blue, number: tile_number, });
            pool.tiles.push(Tile{ color: TileColor::Blue, number: tile_number, });
        }

        pool
    }

    fn choose(mut self) -> Tile {
        self.tiles.swap_remove(
            thread_rng().gen_range(1, self.tiles.len())
        )
    }

    fn draw(mut self) -> Option<Tile> {
        self.tiles.pop()
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

    #[test]
    fn check_pool_size() {
        assert_eq!(Pool::new().tiles.len(), 104)
    }


}