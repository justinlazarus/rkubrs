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

/// Each Rummikub ® tile has only 2 identifying characteristics, a number and
/// color. A standard game has 104 of these tiles plus an additional pair of
/// jokers. Though the jokers in the standard game are usually colored black and
/// red, these colors are not meaningful as jokers can be used to substitute any
/// tile in the game.
pub struct Tile {
    color: TileColor,
    number: u8,
}

/// Each player has a rack to hold tiles. When a player draws tiles from the
/// pool at the beginning of the game and on each following turn, the tiles are
/// placed on the rack. Each tile on the rack can be played individually or as
/// part of a tile set, either a run or a group.
pub struct Rack {
    tiles: Vec<Tile>,
}

/// The pool is usually a bag or some container to house tiles that aren't in
/// play. Players take tiles out of the pool at the beginning of the game and
/// during each round of play.
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

/// When players build sets of tiles (runs and groups), they are played on the
/// table. Once tiles are on the table as runs or groups, they are open for
/// modification by all players.
pub struct Table {
    runs: Vec<Run>,
    groups: Vec<Group>,
}

/// A run is composed of three or more same-colored tiles in consecutive
/// number order. (A 1 may not follow a 13.) For example: red7, red8, red9.
/// A run has a length of minimum 3 and maximum 13 tiles.
pub struct Run {
    color: TileColor,
    tiles: Vec<Tile>,
}

impl Run {
    fn len(&self) -> usize {
        self.tiles.len()
    }
}

/// A group is made from three or four same-value tiles in distinct colors. For
/// example: red3, blue3, black3, yellow3. A group has a length of minimum3 and
/// maximum 4 tiles.
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