use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct CrateStack {
    crates: Vec<char>
}

impl CrateStack {
    pub fn new() -> Self {
        Self { crates: Vec::new() }
    }

    pub fn add_crate(&mut self, value: char) {
        self.crates.push(value);
    }

    pub fn get_crates(&self) -> &Vec<char> {
        return &self.crates;
    }

    pub fn pop(&mut self) -> char {
        return self.crates.pop().expect("This stack no longer has any crates!!");
    }

    pub fn push(&mut self, new_crate: char) {
        self.crates.push(new_crate);
    }

    pub fn get_top_crate(&self) -> char {
        return self.crates[self.crates.len() - 1];
    }
}

#[derive(Debug)]
pub struct Supplies {
    stacks: HashMap<u32, CrateStack>,
    last_index: u32,
}

impl Supplies {
    pub fn new() -> Self {
        Self { stacks: HashMap::new(), last_index: 0 }
    }

    pub fn add_stack(&mut self, crate_stack: CrateStack) {
        self.last_index += 1;
        self.stacks.insert(self.last_index, crate_stack);
    }

    pub fn get_crate_stack(&self, index: u32) -> &CrateStack {
        return self.stacks.get(&index).expect("");
    }

    pub fn from_array(data: Vec<Vec<char>>) -> Self {
        let mut instance = Self::new();
        for stack in data.iter() {
            let mut crate_stack: CrateStack = CrateStack::new();
            for crate_value in stack.iter() {
                crate_stack.add_crate(*crate_value);
            }
            instance.add_stack(crate_stack);
        }
        return instance;
    }

    pub fn move_crate(&mut self, first_stack: u32, second_stack: u32) {
        let first = self.stacks.get_mut(&first_stack).unwrap();
        let move_crate: char = first.pop();

        let second = self.stacks.get_mut(&second_stack).unwrap();
        second.push(move_crate);
    }

    pub fn get_top_crates(&self) -> String {
        let mut crates: String = String::new();
        for index in 1..=self.last_index {
            let stack = self.get_crate_stack(index);
            crates.push_str(&String::from(stack.get_top_crate()));

        }
        return crates;
    }
}

#[derive(Debug)]
pub struct Crane {
    supplies: Supplies,
}

impl Crane {
    pub fn new() -> Self {
        Self { supplies: Supplies::new() }
    }

    pub fn get_supplies(&self) -> &Supplies {
        return &self.supplies;
    }

    pub fn get_crates(&self, index: u32) -> &Vec<char> {
        return &self.supplies.get_crate_stack(index).get_crates();
    }

    pub fn from_array(data: Vec<Vec<char>>) -> Self {
        Self { supplies: Supplies::from_array(data) }
    }

    pub fn parse_instructions_file(&mut self, filepath: &str) {
        let path = String::from(filepath);
        let file = File::open(path).expect("Unable to load file!");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(ip) = line {
                self.move_crate(&ip);
            }
        }
    }

    pub fn move_crate(&mut self, move_text: &str) {
        let mut instructions = move_text.split_ascii_whitespace();
        instructions.next();
        let amount: u32 = instructions.next().unwrap().parse().unwrap();
        instructions.next();
        let from: u32 = instructions.next().unwrap().parse().unwrap();
        instructions.next();
        let to: u32 = instructions.next().unwrap().parse().unwrap();
        for _ in 0..amount {
            self.supplies.move_crate(from, to);
        }
    }

    pub fn get_top_crates(&self) -> String {
        return self.supplies.get_top_crates();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let data: Vec<Vec<char>> = vec![
        vec!['Z', 'N'],
        vec!['M', 'C', 'D'],
        vec!['P'],
        ];
        let mut crane: Crane = Crane::from_array(data);
        assert_eq!(&vec!['Z', 'N'], crane.get_crates(1));
        assert_eq!(&vec!['M', 'C', 'D'], crane.get_crates(2));
        assert_eq!(&vec!['P'], crane.get_crates(3));

        crane.move_crate("move 1 from 2 to 1");
        assert_eq!(&vec!['Z', 'N', 'D'], crane.get_crates(1));
        assert_eq!(&vec!['M', 'C'], crane.get_crates(2));
        assert_eq!(&vec!['P'], crane.get_crates(3));

        crane.move_crate("move 3 from 1 to 3");
        assert_eq!(&Vec::<char>::new(), crane.get_crates(1));
        assert_eq!(&vec!['M', 'C'], crane.get_crates(2));
        assert_eq!(&vec!['P', 'D', 'N', 'Z'], crane.get_crates(3));

        crane.move_crate("move 2 from 2 to 1");
        assert_eq!(&vec!['C', 'M'], crane.get_crates(1));
        assert_eq!(&Vec::<char>::new(), crane.get_crates(2));
        assert_eq!(&vec!['P', 'D', 'N', 'Z'], crane.get_crates(3));

        crane.move_crate("move 1 from 1 to 2");
        assert_eq!(&vec!['C'], crane.get_crates(1));
        assert_eq!(&vec!['M'], crane.get_crates(2));
        assert_eq!(&vec!['P', 'D', 'N', 'Z'], crane.get_crates(3));

        assert_eq!("CMZ", crane.get_top_crates());
    }
}