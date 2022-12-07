use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct SectionAssignment {
    start_section_id: u8,
    end_section_id: u8,
}

impl SectionAssignment {
    pub fn new(start_section_id: u8, end_section_id: u8) -> Self {
        Self {
            start_section_id: start_section_id,
            end_section_id: end_section_id,
        }
    }

    pub fn from_line(line: &str) -> Self {
        let mut split = line.split("-");
        let first: u8 = split.next().unwrap().parse().unwrap();
        let second: u8 = split.next().unwrap().parse().unwrap();
        return Self::new(first, second);
    }

    pub fn get_start_id(&self) -> u8 {
        return self.start_section_id;
    }

    pub fn get_end_id(&self) -> u8 {
        return self.end_section_id;
    }
}

pub struct Section {
    first_elf: SectionAssignment,
    second_elf: SectionAssignment,
}

impl Section {
    pub fn from_line(text_line: &str) -> Self {
        let mut split = text_line.split(",");
        return Self {
            first_elf: SectionAssignment::from_line(split.next().unwrap()),
            second_elf: SectionAssignment::from_line(split.next().unwrap())
        }
    }

    pub fn get_first_elf(&self) -> &SectionAssignment {
        return &self.first_elf;
    }

    pub fn get_second_elf(&self) -> &SectionAssignment {
        return &self.second_elf;
    }

    pub fn get_first_start_id(&self) -> u8 {
        return self.first_elf.get_start_id();
    }

    pub fn get_first_end_id(&self) -> u8 {
        return self.first_elf.get_end_id();
    }

    pub fn get_second_start_id(&self) -> u8 {
        return self.second_elf.get_start_id();
    }

    pub fn get_second_end_id(&self) -> u8 {
        return self.second_elf.get_end_id();
    }

    pub fn fully_contains(&self) -> bool {
        if self.get_first_start_id() < self.get_second_start_id() {
            return self.get_first_end_id() >= self.get_second_end_id();
        } else if self.get_first_start_id() == self.get_second_start_id() {
            if self.get_first_end_id() >= self.get_first_start_id() && self.get_second_end_id() >= self.get_second_start_id() {
                return true;
            } else if self.get_first_end_id() <= self.get_first_start_id() && self.get_second_end_id() <= self.get_second_start_id() {
                return true;
            } else {
                return false;
            }
        } else {
            return self.get_second_end_id() >= self.get_first_end_id();
        }
    }

    pub fn partial_contains(&self) -> bool {
        if self.get_first_start_id() < self.get_second_start_id() {
            return self.get_first_end_id() >= self.get_second_start_id();
        } else if self.get_first_start_id() == self.get_second_start_id() {
            return (self.get_first_end_id() >= self.get_first_end_id() && self.get_second_end_id() >= self.get_first_end_id()) || (self.get_first_end_id() <= self.get_first_end_id() && self.get_second_end_id() <= self.get_first_end_id())
        } else {
            return self.get_second_end_id() >= self.get_first_start_id();
        }
    }
}

#[derive(Debug)]
pub struct Sections {
    fully_contains_count: u32,
    partially_contains_count: u32,
}

impl Sections {
    pub fn new() -> Self {
        Self { fully_contains_count: 0, partially_contains_count: 0 }
    }

    pub fn from_file(filepath: &str) -> Self {
        let path = String::from(filepath);
        let mut instance = Self::new();
        let file = File::open(path).expect("Unable to load file!");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(ip) = line {
                let section = Section::from_line(&ip);
                if section.fully_contains() {
                    instance.fully_contains_count += 1;
                }
                if section.partial_contains() {
                    instance.partially_contains_count += 1;
                }
            }
        }
        return instance;
    }

    pub fn get_fully_count(&self) -> u32 {
        return self.fully_contains_count;
    }

    pub fn get_partially_count(&self) -> u32 {
        return self.partially_contains_count;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_section_assignment_from_line() {
        let line = "2-8";
        let sa = SectionAssignment::from_line(&line);
        assert_eq!(2, sa.get_start_id());
        assert_eq!(8, sa.get_end_id());
    }

    #[test]
    fn test_create_section_from_line() {
        let line = "60-60,45-60";
        let section = Section::from_line(line);
        assert_eq!(60, section.get_first_elf().get_start_id());
        assert_eq!(60, section.get_first_elf().get_end_id());
        assert_eq!(45, section.get_second_elf().get_start_id());
        assert_eq!(60, section.get_second_elf().get_end_id());
        assert_eq!(true, section.fully_contains());
    }

    #[test]
    fn test_fully_contains_1() {
        let line = "2-4,6-8";
        let section = Section::from_line(line);
        assert_eq!(false, section.fully_contains());
    }

    #[test]
    fn test_fully_contains_2() {
        let line = "2-3,4-5";
        let section = Section::from_line(line);
        assert_eq!(false, section.fully_contains());
    }

    #[test]
    fn test_fully_contains_3() {
        let line = "5-7,7-9";
        let section = Section::from_line(line);
        assert_eq!(false, section.fully_contains());
    }

    #[test]
    fn test_fully_contains_4() {
        let line = "2-8,3-7";
        let section = Section::from_line(line);
        assert_eq!(true, section.fully_contains());
    }

    #[test]
    fn test_fully_contains_5() {
        let line = "6-6,4-6";
        let section = Section::from_line(line);
        assert_eq!(true, section.fully_contains());
    }

    #[test]
    fn test_fully_contains_6() {
        let line = "2-6,4-8";
        let section = Section::from_line(line);
        assert_eq!(false, section.fully_contains());
    }

    #[test]
    fn test_fully_contains_7() {
        let line = "5-7,2-8";
        let section = Section::from_line(line);
        assert_eq!(true, section.fully_contains());
    }

    #[test]
    fn test_fully_contains_8() {
        let line = "4-90,4-4";
        let section = Section::from_line(line);
        assert_eq!(true, section.fully_contains());
    }

    #[test]
    fn test_fully_contains_9() {
        let line = "4-4,4-90";
        let section = Section::from_line(line);
        assert_eq!(true, section.fully_contains());
    }

    #[test]
    fn test_fully_contains_10() {
        let line = "4-90,6-90";
        let section = Section::from_line(line);
        assert_eq!(true, section.fully_contains());
    }

    #[test]
    fn test_partial_contains_1() {
        let line = "2-4,6-8";
        let section = Section::from_line(line);
        assert_eq!(false, section.partial_contains());
    }

    #[test]
    fn test_partial_contains_2() {
        let line = "2-3,4-5";
        let section = Section::from_line(line);
        assert_eq!(false, section.partial_contains());
    }

    #[test]
    fn test_partial_contains_3() {
        let line = "5-7,7-9";
        let section = Section::from_line(line);
        assert_eq!(true, section.partial_contains());
    }

    #[test]
    fn test_partial_contains_4() {
        let line = "2-8,3-7";
        let section = Section::from_line(line);
        assert_eq!(true, section.partial_contains());
    }

    #[test]
    fn test_partial_contains_5() {
        let line = "6-6,4-6";
        let section = Section::from_line(line);
        assert_eq!(true, section.partial_contains());
    }

    #[test]
    fn test_partial_contains_6() {
        let line = "2-6,4-8";
        let section = Section::from_line(line);
        assert_eq!(true, section.partial_contains());
    }

    #[test]
    fn test_partial_contains_7() {
        let line = "5-7,2-8";
        let section = Section::from_line(line);
        assert_eq!(true, section.partial_contains());
    }

    #[test]
    fn test_partial_contains_8() {
        let line = "4-90,4-4";
        let section = Section::from_line(line);
        assert_eq!(true, section.partial_contains());
    }

    #[test]
    fn test_partial_contains_9() {
        let line = "4-4,4-90";
        let section = Section::from_line(line);
        assert_eq!(true, section.partial_contains());
    }

    #[test]
    fn test_partial_contains_10() {
        let line = "4-90,6-90";
        let section = Section::from_line(line);
        assert_eq!(true, section.partial_contains());
    }


    #[test]
    fn test_sections() {
        let sections = Sections::from_file("src/day_4/test.txt");
        assert_eq!(2, sections.get_fully_count());
        assert_eq!(4, sections.get_partially_count());
    }
}
