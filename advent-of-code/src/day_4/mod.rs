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
        println!("{:#?}", split);
        println!("{}", split.next().unwrap());
        println!("{}", split.next().unwrap());
        let first: u8 = split.next().chars();
        let second: u8 = split.next().chars();
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
}
