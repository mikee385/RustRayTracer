use std::vec::{Vec};

pub struct Table<T> {
    pub width: uint,
    pub height: uint,
    pub data: Vec<T>
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
            //throw std::out_of_range("row");
            println!("Out of range {} >= {}", row, self.height);
        }
        if column >= self.width {
            //throw std::out_of_range("column");
            println!("Out of range {} >= {}", column, self.width);
        }

        row * self.width + column
    }
    
    pub fn set(&mut self, row: uint, column: uint, value: T) {
        let index = self.get_index(row, column);
        self.data[index] = value;
    }
    
    pub fn get<'a>(&'a self, row: uint, column: uint) -> &'a T {
        let index = self.get_index(row, column);
        &self.data[index]
    }
}