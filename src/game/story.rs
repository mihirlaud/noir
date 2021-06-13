use js_sys::Math::floor;
use js_sys::Math::random;

use petgraph::prelude::*;

#[derive(Debug)]
pub struct Story {
    setting: Setting,
    people: UnGraph<Person, f64>,
}

impl Story {
    pub fn rand_gen() -> Self {
        Self {
            setting: Setting::rand_gen(),
            people: Person::create_characters(),
        }
    }
}

#[derive(Debug)]
pub struct Setting {
    location: String,
}

impl Setting {
    pub fn rand_gen() -> Self {
        unsafe {
            let location = floor(random() * 3.0) as i32;
            let location = match location {
                0 => "Suburbs".to_string(),
                1 => "City".to_string(),
                _ => "Mansion".to_string(),
            };
            Self { location }
        }
    }
}

#[derive(Debug)]
pub struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    pub fn create_characters() -> UnGraph<Person, f64> {
        let mut graph: UnGraph<Person, f64> = Graph::new_undirected();

        let victim = Person::rand_gen();

        let victim_id = graph.add_node(victim);

        for i in 0..4 {
            let relative_id = graph.add_node(Person::rand_gen());
            unsafe {
                graph.add_edge(victim_id, relative_id, random());
            }
        }

        graph
    }

    pub fn rand_gen() -> Self {
        let first_names = vec!["John", "Jack", "Bill", "Bob"];
        let last_names = vec!["Smith", "Johnson", "Phillips", "Williams"];

        unsafe {
            let first_name = first_names
                .get(floor(random() * first_names.len() as f64) as usize)
                .unwrap()
                .to_string();

            let last_name = last_names
                .get(floor(random() * last_names.len() as f64) as usize)
                .unwrap()
                .to_string();

            Self {
                first_name,
                last_name,
            }
        }
    }
}
