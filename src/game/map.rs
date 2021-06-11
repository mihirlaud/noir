use super::story::Story;

pub struct Map {
    map: Vec<Vec<Tile>>,
}

impl Map {
    pub fn from_story(story: Story) -> Self {
        Self { map: vec![] }
    }

    pub fn get_tile_at(&self, x: usize, y: usize) -> Tile {
        self.map.get(y).unwrap().get(x).unwrap().clone()
    }
}

#[derive(Clone, Debug)]
pub struct Tile {
    pub blocks_movement: bool,
    pub blocks_light: bool,
}
