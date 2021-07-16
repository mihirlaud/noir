use std::collections::HashSet;

use rltk::{RandomNumberGenerator, RGB};
use specs::prelude::*;
use specs_derive::Component;

use crate::constants::*;

#[derive(Debug)]
pub struct Story {
    pub victim: Victim,
    pub suspects: Vec<Suspect>,
    pub clues: Vec<Clue>,
    pub connections: Vec<Connection>,
}

impl Story {
    pub fn gen_rand() -> Self {
        let victim = Victim::gen_rand();
        let suspects = Suspect::gen_rand_suspects(&victim);
        let clues = Clue::gen_rand_clues(&victim);
        let connections = Connection::gen_connections(&victim, suspects.clone(), clues.clone());

        Story {
            victim,
            suspects,
            clues,
            connections,
        }
    }
}

#[derive(Debug)]
pub struct Victim {
    pub name: String,
    pub age: i32,
    pub weapon_used: String,
    pub hair_found: String,
    pub shoe_print: String,
    pub clue: Clue,
}

impl Victim {
    fn gen_rand() -> Self {
        let mut rand = RandomNumberGenerator::new();

        let names = ["Hercule", "Sherlock", "Johnny"];
        let name = names[rand.range(0, names.len())].to_string();

        let age = rand.range(22, 65);

        let weapons = ["knife", "gun", "wrench", "poison"];
        let weapon_used = weapons[rand.range(0, weapons.len())].to_string();

        let hair_colors = ["black", "blonde", "red"];
        let hair_found = hair_colors[rand.range(0, hair_colors.len())].to_string();

        let shoe_prints = ["small", "average", "large"];
        let shoe_print = shoe_prints[rand.range(0, shoe_prints.len())].to_string();

        let color = rltk::RED;

        let display = vec![
            "      ___      ".to_string(),
            "     /   \\     ".to_string(),
            "     |   |     ".to_string(),
            "     \\   /     ".to_string(),
            "    __| |__    ".to_string(),
            "   /       \\   ".to_string(),
            "  / /|   |\\ \\  ".to_string(),
            " / / |   | \\ \\ ".to_string(),
            "/_/  |   |  \\_\\".to_string(),
            "     |   |     ".to_string(),
            "     / ^ \\     ".to_string(),
            "    / / \\ \\    ".to_string(),
            "   / /   \\ \\   ".to_string(),
            "  /_/     \\_\\  ".to_string(),
        ];

        let mut tags = vec![];

        match weapon_used.as_str() {
            "knife" => {
                tags.push(Note::new(
                    vec![
                        ("The".to_string(), rltk::WHITE, false),
                        ("victim".to_string(), color, false),
                        ("died from a".to_string(), rltk::WHITE, false),
                        ("stab wound".to_string(), color, true),
                    ],
                    Some(ConnectionType::MurderWeapon),
                ));
            }
            "gun" => {
                tags.push(Note::new(
                    vec![
                        ("The".to_string(), rltk::WHITE, false),
                        ("victim".to_string(), color, false),
                        ("died from a".to_string(), rltk::WHITE, false),
                        ("gunshot wound".to_string(), color, true),
                    ],
                    Some(ConnectionType::MurderWeapon),
                ));
            }
            "wrench" => {
                tags.push(Note::new(
                    vec![
                        ("The".to_string(), rltk::WHITE, false),
                        ("victim".to_string(), color, false),
                        ("died from".to_string(), rltk::WHITE, false),
                        ("blunt force trauma".to_string(), color, true),
                    ],
                    Some(ConnectionType::MurderWeapon),
                ));
            }
            "poison" => {
                tags.push(Note::new(
                    vec![
                        ("The".to_string(), rltk::WHITE, false),
                        ("victim".to_string(), color, false),
                        ("died from".to_string(), rltk::WHITE, false),
                        ("poisoning".to_string(), color, true),
                    ],
                    Some(ConnectionType::MurderWeapon),
                ));
            }
            _ => {}
        }

        tags.push(Note::new(
            vec![
                ("The killer left".to_string(), rltk::WHITE, false),
                (format!("{} footprints", shoe_print), color, true),
                ("next to the".to_string(), rltk::WHITE, false),
                ("victim".to_string(), color, false),
            ],
            Some(ConnectionType::EvidenceShoeSize),
        ));

        let mut markers = vec![];

        for tag in tags.clone() {
            let mut rand = rltk::RandomNumberGenerator::new();

            let w = display[0].len() as i32;
            let h = display.len() as i32;

            loop {
                let x: i32 = EXAM_PANEL_WIDTH / 2 - w / 2 + rand.range(0, w);
                let y: i32 = EXAM_PANEL_HEIGHT / 2 - h / 2 + rand.range(0, h);

                let mut available = true;

                for (marker_x, marker_y, _, _) in markers.clone() {
                    let marker_x: i32 = marker_x;
                    let marker_y: i32 = marker_y;
                    if (x - marker_x).abs() <= 2 && (y - marker_y).abs() <= 2 {
                        available = false;
                        break;
                    }
                }

                if available {
                    markers.push((x, y, tag, false));
                    break;
                }
            }
        }

        let clue = Clue {
            name: format!("Victim: {}", &name),
            color: RGB::named(rltk::RED),
            is_murder_weapon: false,
            display,
            markers,
        };

        Victim {
            name,
            age,
            weapon_used,
            hair_found,
            shoe_print,
            clue,
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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum ConnectionType {
    MurderWeapon,
    EvidenceHair,
    EvidenceShoeSize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Connection {
    pub ids: (u32, u32),
    pub cxn_type: ConnectionType,
    pub note: Note,
}

impl Connection {
    pub fn new(from: u32, to: u32, cxn_type: ConnectionType, note: Note) -> Self {
        Connection {
            ids: (from, to),
            cxn_type,
            note,
        }
    }

    pub fn gen_connections(
        victim: &Victim,
        suspects: Vec<Suspect>,
        clues: Vec<Clue>,
    ) -> Vec<Connection> {
        let mut cxns = vec![];

        let mut mw_from = 0;
        for marker in victim.clue.markers.clone() {
            if marker.2.cxn_type.is_some()
                && marker.2.cxn_type.unwrap() == ConnectionType::MurderWeapon
            {
                mw_from = marker.2.id;
            }
        }
        let mut mw_to = 0;
        for clue in clues {
            for marker in clue.markers.clone() {
                if marker.2.cxn_type.is_some()
                    && marker.2.cxn_type.unwrap() == ConnectionType::MurderWeapon
                {
                    mw_to = marker.2.id;

                    let note = Note::new(
                        vec![
                            ("Aha! The".to_string(), rltk::WHITE, false),
                            ("murder weapon".to_string(), rltk::RED, false),
                            ("must have been".to_string(), rltk::WHITE, false),
                            (clue.name.clone(), rltk::RED, false),
                        ],
                        None,
                    );

                    cxns.push(Connection::new(
                        mw_from,
                        mw_to,
                        ConnectionType::MurderWeapon,
                        note,
                    ));
                }
            }
        }

        cxns
    }
}

static mut NOTE_ID: u32 = 0;

#[derive(Component, Debug, PartialEq, Eq, Hash, Clone)]
pub struct Note {
    pub id: u32,
    pub note: Vec<(String, (u8, u8, u8), bool)>,
    pub cxn_type: Option<ConnectionType>,
}

impl Note {
    pub fn new(note: Vec<(String, (u8, u8, u8), bool)>, cxn_type: Option<ConnectionType>) -> Self {
        unsafe {
            let note = Note {
                id: NOTE_ID,
                note,
                cxn_type,
            };
            NOTE_ID += 1;
            note
        }
    }

    pub fn get_log_msg(&self) -> String {
        let mut log_msg = String::new();

        for pair in self.note.clone() {
            log_msg.push_str(&pair.0);
            log_msg.push_str(" ");
        }

        log_msg
    }
}

#[derive(Component, Debug, PartialEq, Clone)]
pub struct PlayerNotes {
    pub notes: HashSet<Note>,
}

impl PlayerNotes {
    pub fn new() -> Self {
        PlayerNotes {
            notes: HashSet::new(),
        }
    }

    pub fn add_note(&mut self, note: Note) {
        self.notes.insert(note);
    }
}

#[derive(Component, Clone, Debug)]
pub struct Clue {
    pub name: String,
    pub color: RGB,
    pub is_murder_weapon: bool,
    pub display: Vec<String>,
    pub markers: Vec<(i32, i32, Note, bool)>,
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

        let color = if is_murder_weapon {
            rltk::PURPLE
        } else {
            rltk::LIMEGREEN
        };

        let mut tags = vec![];

        if is_murder_weapon {
            tags.push(Note::new(
                vec![
                    ("While examining".to_string(), rltk::WHITE, false),
                    (format!("{},", name.clone()), color, false),
                    ("I found".to_string(), rltk::WHITE, false),
                    ("blood".to_string(), color, true),
                ],
                Some(ConnectionType::MurderWeapon),
            ));
            tags.push(Note::new(
                vec![
                    ("While examining".to_string(), rltk::WHITE, false),
                    (format!("{},", name.clone()), color, false),
                    ("I found".to_string(), rltk::WHITE, false),
                    (format!("{} hair", victim.hair_found), color, true),
                ],
                Some(ConnectionType::EvidenceHair),
            ));
        }

        let mut markers = vec![];

        for tag in tags.clone() {
            let mut rand = rltk::RandomNumberGenerator::new();

            let w = display[0].len() as i32;
            let h = display.len() as i32;

            loop {
                let x: i32 = EXAM_PANEL_WIDTH / 2 - w / 2 + rand.range(0, w);
                let y: i32 = EXAM_PANEL_HEIGHT / 2 - h / 2 + rand.range(0, h);

                let mut available = true;

                for (marker_x, marker_y, _, _) in markers.clone() {
                    let marker_x: i32 = marker_x;
                    let marker_y: i32 = marker_y;
                    if (x - marker_x).abs() <= 2 && (y - marker_y).abs() <= 2 {
                        available = false;
                        break;
                    }
                }

                if available {
                    markers.push((x, y, tag, false));
                    break;
                }
            }
        }

        Clue {
            name,
            color: RGB::named(color),
            is_murder_weapon,
            display,
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

    pub fn reveal_marker(&mut self, idx: usize) {
        let mut marker = self.markers.remove(idx);
        marker.3 = true;
        self.markers.insert(idx, marker);
    }
}
