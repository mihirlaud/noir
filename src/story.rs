use rltk::{RandomNumberGenerator, RGB};
use specs::prelude::*;
use specs_derive::Component;

use crate::constants::*;

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

#[derive(Component, Debug, Clone, Default)]
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
            color: if is_killer {
                RGB::named(rltk::RED)
            } else {
                RGB::named(rltk::YELLOW)
            },
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

#[derive(Component, Clone, Debug)]
pub struct Clue {
    pub name: String,
    pub color: RGB,
    pub is_murder_weapon: bool,
    pub display: Vec<String>,
    pub tags: Vec<String>,
    pub markers: Vec<(i32, i32, String)>,
}

impl Clue {
    fn gen_rand(victim: &Victim, is_murder_weapon: bool) -> Self {
        let mut rand = RandomNumberGenerator::new();

        let names = ["knife", "gun", "wrench", "poison"];
        let mut name = names[rand.range(0, names.len())].to_string();

        if is_murder_weapon {
            name = victim.weapon_used.to_string();
        }

        let display = match name.as_str() {
            "knife" => {
                vec![
                    "___________________________________ ______________________  ".to_string(),
                    "\\                                  | (_)     (_)    (_)   \\ ".to_string(),
                    " `.                                |  __________________   }".to_string(),
                    "   `-..........................____|_(                  )_/ ".to_string(),
                ]
            }
            "gun" => {
                vec![
                    " _ ________,".to_string(),
                    " >`(==(----'".to_string(),
                    "(__/~~`     ".to_string(),
                ]
            }
            "poison" => {
                vec![
                    "     o=o     ".to_string(),
                    "     | |     ".to_string(),
                    "  ___| |___  ".to_string(),
                    " /         \\ ".to_string(),
                    "|    .-.    |".to_string(),
                    "|   (0.0)   |".to_string(),
                    "| '=.|m|.=' |".to_string(),
                    "| .='`\"``=. |".to_string(),
                    " \\_________/ ".to_string(),
                ]
            }
            "wrench" => {
                vec![
                    "-------".to_string(),
                    "|     |".to_string(),
                    "|     |".to_string(),
                    "|     |".to_string(),
                    "|     |".to_string(),
                    "|     |".to_string(),
                    "-------".to_string(),
                ]
            }
            _ => vec!["".to_string()],
        };

        let tags = vec![
            "Killer left a partial fingerprint.".to_string(),
            "Killer's hair is on the weapon.".to_string(),
        ];

        let mut markers = vec![];

        for tag in tags.clone() {
            let mut rand = rltk::RandomNumberGenerator::new();

            let w = display[0].len() as i32;
            let h = display.len() as i32;

            loop {
                let x: i32 = EXAM_PANEL_WIDTH / 2 - w / 2 + rand.range(0, w);
                let y: i32 = EXAM_PANEL_HEIGHT / 2 - h / 2 + rand.range(0, h);

                let mut available = true;

                for (marker_x, marker_y, _) in markers.clone() {
                    let marker_x: i32 = marker_x;
                    let marker_y: i32 = marker_y;
                    if (x - marker_x).abs() <= 2 && (y - marker_y).abs() <= 2 {
                        available = false;
                        break;
                    }
                }

                if available {
                    markers.push((x, y, tag));
                    break;
                }
            }
        }

        Clue {
            name,
            color: if is_murder_weapon {
                RGB::named(rltk::PURPLE)
            } else {
                RGB::named(rltk::LIMEGREEN)
            },
            is_murder_weapon,
            display,
            tags,
            markers,
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
