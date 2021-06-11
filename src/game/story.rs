use macroquad::rand::*;
use macroquad::time::get_time;
use petgraph::{graph::*, Undirected};

#[derive(Clone, Debug)]
pub struct Story {
    setting: Setting,
    people: Person,
}

impl Story {
    pub fn rand_gen() -> Self {
        Self {
            setting: Setting::new(),
            people: Person::new(),
        }
    }
}

#[derive(Clone, Debug)]
struct Setting {
    location: String,
}

impl Setting {
    pub fn new() -> Self {
        let location: u32 = get_rand(0, 3);
        let location = match location {
            1 => String::from("Suburbs"),
            2 => String::from("City"),
            _ => String::from("Mansion"),
        };
        Self { location }
    }
}

#[derive(Clone, Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u32,
}

impl Person {
    pub fn new() -> Self {
        let first_names = vec!["Jack", "John", "Bill", "Bob"];
        let last_names = vec!["Smith", "Johnson", "Williams", "Phillips"];

        let first_name = String::from(
            first_names
                .get(get_rand(0, first_names.len()))
                .unwrap()
                .clone(),
        );

        let last_name = String::from(
            last_names
                .get(get_rand(0, last_names.len()))
                .unwrap()
                .clone(),
        );

        let age = get_rand(20, 80);

        Self {
            first_name,
            last_name,
            age,
        }
    }

    pub fn create_graph() -> UnGraph<Person, f32> {
        Graph::<Person, f32, Undirected>::new_undirected()
    }
}

#[derive(Debug)]
struct Crime {}

fn get_rand<T>(low: T, high: T) -> T
where
    T: RandomRange,
{
    srand((get_time() * 10e15) as u64 % 1000 * (get_time() * 10e15) as u64 % 1000);
    gen_range(low, high)
}
