use rltk::{RandomNumberGenerator, RGB};
use specs::prelude::*;
use specs_derive::Component;

#[derive(Debug)]
pub struct Story {
    pub victim: Victim,
    pub suspects: Vec<Suspect>,
    pub clues: Vec<Clue>,
}

impl Story {
    pub fn gen_rand() -> Self {
        let victim = Victim::gen_rand();
        let suspects = Suspect::gen_rand_suspects(&victim);
        let clues = Clue::gen_rand_clues(&victim);

        Story {
            victim,
            suspects,
            clues,
        }
    }
}

#[derive(Debug)]
pub struct Victim {
    name: String,
    age: i32,
    weapon_used: String,
    hair_found: String,
    shoe_print: String,
}

impl Victim {
    fn gen_rand() -> Self {
        let mut rand = RandomNumberGenerator::new();

        let names = ["Hercule", "Sherlock", "Johnny"];
        let name = names[rand.range(0, names.len())].to_string();

        let age = rand.range(22, 65);

        let weapons = ["knife", "gun", "wrench", "poison"];
        let weapon_used = weapons[rand.range(0, weapons.len())].to_string();

        let hair_colors = ["none", "black", "blonde", "red"];
        let hair_found = hair_colors[rand.range(0, hair_colors.len())].to_string();

        let shoe_prints = ["none", "small", "average", "large"];
        let shoe_print = shoe_prints[rand.range(0, shoe_prints.len())].to_string();

        Victim {
            name,
            age,
            weapon_used,
            hair_found,
            shoe_print,
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct Suspect {
    pub name: String,
    pub age: i32,
    pub color: RGB,
    pub is_killer: bool,
    pub hair_color: String,
    pub shoe_size: String,
}

impl Suspect {
    fn gen_rand(victim: &Victim, is_killer: bool) -> Self {
        let mut rand = RandomNumberGenerator::new();

        let names = ["Adam", "Barry", "Charles"];
        let name = names[rand.range(0, names.len())].to_string();

        let age = rand.range(22, 65);

        let hair_colors = ["black", "blonde", "red"];
        let mut hair_color = hair_colors[rand.range(0, hair_colors.len())].to_string();

        let shoe_sizes = ["small", "average", "large"];
        let mut shoe_size = shoe_sizes[rand.range(0, shoe_sizes.len())].to_string();

        if is_killer {
            if victim.hair_found != "none" {
                hair_color = victim.hair_found.to_string();
            }
            if victim.shoe_print != "none" {
                shoe_size = victim.shoe_print.to_string();
            }
        }

        Suspect {
            name,
            age,
            color: RGB::named(rltk::YELLOW),
            is_killer,
            hair_color,
            shoe_size,
        }
    }

    fn gen_rand_suspects(victim: &Victim) -> Vec<Suspect> {
        let mut suspects = vec![];

        for i in 0..3 {
            suspects.push(Suspect::gen_rand(victim, i == 0));
        }

        suspects
    }
}

#[derive(Debug)]
pub struct Clue {
    name: String,
    is_murder_weapon: bool,
}

impl Clue {
    fn gen_rand(victim: &Victim, is_murder_weapon: bool) -> Self {
        let mut rand = RandomNumberGenerator::new();

        let names = ["knife", "gun", "wrench", "poison"];
        let mut name = names[rand.range(0, names.len())].to_string();

        if is_murder_weapon {
            name = victim.weapon_used.to_string();
        }

        Clue {
            name,
            is_murder_weapon,
        }
    }

    fn gen_rand_clues(victim: &Victim) -> Vec<Clue> {
        let mut clues = vec![];

        for i in 0..3 {
            clues.push(Clue::gen_rand(victim, i == 0));
        }

        clues
    }
}
