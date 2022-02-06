pub enum TileColor {
    Blue,
    Red,
    Yellow,
    Black,
}

pub enum TileNumber {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Joker,
}

pub struct Tile {
    color: TileColor,
    number: TileNumber,
}

pub struct Rack {
    tiles: Vec<Tile>,

}

pub struct Pool {
    tiles: Vec<Tile>,
}

pub struct Table {
    sets: Vec<Set>,
}

pub struct Set {
    tiles: Vec<Tile>,
}