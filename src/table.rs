use std::vec::{Vec};

pub struct Table<T> {
    width: uint,
    height: uint,
    data: Vec<T>
}

impl<T: Clone> Table<T> {
    pub fn from_elem(width: uint, height: uint, value: T) -> Table<T> {
        Table {
            width: width,
            height: height,
            data: Vec::from_elem(width * height, value)
        }
    }
}

impl<T> Table<T> {
    fn get_index(&self, row: uint, column: uint) -> uint {
        if row >= self.height {
            fail!("Table::get_index: `row` overflow ({} >= {})", row, self.height)
        }
        if column >= self.width {
            fail!("Table::get_index: `column` overflow ({} >= {})", column, self.width)
        }

        row * self.width + column
    }

    pub fn get_width(&self) -> uint {
        self.width
    }

    pub fn get_height(&self) -> uint {
        self.height
    }
    
    pub fn set(&mut self, row: uint, column: uint, value: T) {
        let index = self.get_index(row, column);
        self.data[index] = value;
    }
    
    pub fn get(&self, row: uint, column: uint) -> &T {
        let index = self.get_index(row, column);
        &self.data[index]
    }
}