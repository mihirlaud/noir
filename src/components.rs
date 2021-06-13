pub struct Position {
    x: i32,
    y: i32,
}

pub struct Glyph<'a> {
    char: Character,
    color: &'a str,
}
