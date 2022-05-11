#[derive(Debug,Clone,Copy,Eq,PartialEq)]

//Corners are defined in terms of the directions which are open
//Threeways are defined in terms of the single direction which is NOT open
pub enum Tile{
    Blank,
    CorridorNS,
    CorridorEW,
    CornerNW,
    CornerNE,
    CornerSW,
    CornerSE,
    ThreewayN,
    ThreewayS,
    ThreewayE,
    ThreewayW,
    Fourway,
    Room,
    Room2,
    Water,
    Unknown,
}

//Rotates the tile clockwise
pub fn rotate_tile_cw(tile:Tile)->Tile{
    match tile{
        Tile::CorridorNS => Tile::CorridorEW,
        Tile::CorridorEW => Tile::CorridorNS,
        Tile::CornerNW => Tile::CornerNE,
        Tile::CornerNE => Tile::CornerSE,
        Tile::CornerSE => Tile::CornerSW,
        Tile::CornerSW => Tile::CornerNW,
        Tile::ThreewayN => Tile::ThreewayE,
        Tile::ThreewayE => Tile::ThreewayS,
        Tile::ThreewayS => Tile::ThreewayW,
        Tile::ThreewayW => Tile::ThreewayN,
        _ => tile,
    }
}

//Flips the tile vertically
pub fn flip_tile_v(tile:Tile)->Tile{
    match tile{
            Tile::CornerNW => Tile::CornerSW,
            Tile::CornerNE => Tile::CornerSE,
            Tile::CornerSW => Tile::CornerNW,
            Tile::CornerSE => Tile::CornerNE,
            Tile::ThreewayN => Tile::ThreewayS,
            Tile::ThreewayS => Tile::ThreewayN,
            _ => tile,
    }
}

//Flips the tile horizontally
pub fn flip_tile_h(tile:Tile)->Tile{
    match tile{
            Tile::CornerNW => Tile::CornerNE,
            Tile::CornerNE => Tile::CornerNW,
            Tile::CornerSW => Tile::CornerSE,
            Tile::CornerSE => Tile::CornerSW,
            Tile::ThreewayW => Tile::ThreewayE,
            Tile::ThreewayE => Tile::ThreewayW,
            _ => tile,
    }
}

//What a given tile will display as
pub fn print_tile(tile:&Tile)->char{
    match tile{
        Tile::Blank => '.',
        Tile::CorridorNS => '║',
        Tile::CorridorEW => '═',
        Tile::CornerNW => '╝',
        Tile::CornerNE => '╚',
        Tile::CornerSE => '╔',
        Tile::CornerSW => '╗',
        Tile::ThreewayN => '╦',
        Tile::ThreewayE => '╣',
        Tile::ThreewayS => '╩',
        Tile::ThreewayW => '╠',
        Tile::Fourway => '╬',
        Tile::Room => '█',
        Tile::Room2 => '█',
        Tile::Water => '~',
        Tile::Unknown => '?',
    }
}

//What inputs in a rule map to a given tile
pub fn import_tile(ch:char)->Tile{
    match ch{
        'c' => Tile::CorridorEW,
        'C' => Tile::CorridorNS,
        '7' => Tile::CornerSE,
        '8' => Tile::ThreewayN,
        '9' => Tile::CornerSW,
        '4' => Tile::ThreewayW,
        '5' => Tile::Fourway,
        '6' => Tile::ThreewayE,
        '1' => Tile::CornerNE,
        '2' => Tile::ThreewayS,
        '3' => Tile::CornerNW,
        'r' => Tile::Room,
        'R' => Tile::Room2,
        '~' => Tile::Water,
        '.' => Tile::Blank,
        '?' => Tile::Unknown,
        _ => Tile::Unknown,
    }
}
