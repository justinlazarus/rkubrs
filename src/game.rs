use rand::Rng;
use rand::thread_rng;

pub const MAX_TILE_NUMBER: u8 = 13;
pub const JOKER_COUNT: u8 = 2;

pub enum TileColor {
    Blue,
    Red,
    Yellow,
    Black,
}

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

    fn draw(&mut self) -> Tile {
        self.tiles.swap_remove(
            thread_rng().gen_range(1, self.tiles.len())
        )
    }
}

pub struct Table {
    runs: Vec<Run>,
    groups: Vec<Group>,
}

pub struct Run {
    color: TileColor,
    tiles: Vec<Tile>,
}

impl Run {
    fn len(&self) -> usize {
        self.tiles.len()
    }


}

pub struct Group {
    number: u8,
    tiles: Vec<Tile>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pool_new() {
        assert_eq!(Pool::new().tiles.len(), 104)
    }

    #[test]
    fn pool_draw() {
        let mut pool = Pool::new();
        let tile = pool.draw();

        assert_eq!(pool.tiles.len(), 103);
    }

}