use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct Elves<'a> {
    elves: HashMap<i32, &'a Elf>,
    last_index: i32,
}

impl Elves {
    pub fn new() -> Self {
        Self {
            elves: HashMap::new(),
            last_index: -1,
        }
    }

    pub fn load_elves(filepath: String) -> Self {
        let mut instance = Self::new();
        let file = File::open(filepath).expect("Unable to load file!");
        let reader = BufReader::new(file);
        let mut elf = Elf::new();

        for line in reader.lines() {
            if let Ok(ip) = line {
                if ip == String::from("") {
                    instance.add_elf(&elf);
                    let mut elf = Elf::new();
                } else {
                    elf.add_food(Food::new(ip.parse::<i32>().unwrap()));
                }
            }
        }
        return instance;
    }

    pub fn add_elf(&mut self, new_elf: &Elf) {
        self.elves.insert(self.get_last_index() + 1, new_elf);
    }

    pub fn get_last_index(&self) -> i32 {
        return self.last_index;
    }
}

#[derive(Debug)]
pub struct Elf {
    foods: Vec<Food>,
}

impl Elf {
    pub fn new() -> Self {
        Self { foods: Vec::new() }
    }

    pub fn add_food(&mut self, new_food: Food) {
        self.foods.push(new_food)
    }
}

#[derive(Debug)]
pub struct Food {
    calories: i32,
}

impl Food {
    pub fn new(calories: i32) -> Self {
        Self { calories: calories }
    }

    pub fn get_calories(&self) -> i32 {
        return self.calories;
    }
}
