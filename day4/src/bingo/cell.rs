#[derive(PartialEq, Debug)]
pub struct Cell {
    value: u32,
    is_marked: bool,
}

impl Clone for Cell {
    fn clone(&self) -> Self {
        Cell { value: self.value, is_marked: self.is_marked }
    }
}

impl Cell {
    pub fn new(value: u32) -> Cell {
        Cell {
            value,
            is_marked: false,
        }
    }

    pub fn value(&self) -> u32 {
        self.value
    }

    pub fn is_marked(&self) -> bool {
        self.is_marked
    }

    pub fn mark(&mut self, called_number: u32) {
        if called_number == self.value {
            self.is_marked = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bingo::*;
    use crate::bingo::cell::Cell;

    #[test]
    fn marks_self_when_value_is_called() {
        let mut cell = Cell::new(5);
        cell.mark(5);
        assert_eq!(true, cell.is_marked());
    }

    #[test]
    fn doesnt_mark_self_when_other_number_is_called() {
        let mut cell = Cell::new(5);
        cell.mark(10);
        assert_eq!(false, cell.is_marked());
    }
}
