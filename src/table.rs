#![allow(dead_code)]
#![allow(unused_variables)]

use std::slice::{Items, MutItems};
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
    #[inline]
    fn get_index(&self, row: uint, column: uint) -> uint {
        if row >= self.height {
            panic!("Table::get_index: `row` overflow ({} >= {})", row, self.height)
        }
        if column >= self.width {
            panic!("Table::get_index: `column` overflow ({} >= {})", column, self.width)
        }

        row * self.width + column
    }

    #[inline]
    pub fn get_width(&self) -> uint {
        self.width
    }

    #[inline]
    pub fn get_height(&self) -> uint {
        self.height
    }
    
    #[inline]
    pub fn set(&mut self, row: uint, column: uint, value: T) {
        let index = self.get_index(row, column);
        self.data[index] = value;
    }
    
    #[inline]
    pub fn get(&self, row: uint, column: uint) -> &T {
        let index = self.get_index(row, column);
        &self.data[index]
    }

    #[inline]
    pub fn iter<'a>(&'a self) -> TableItems<'a, T> {
        TableItems {
            iter: self.data.iter(),
            width: self.width,
            height: self.height
        }
    }

    #[inline]
    pub fn iter_mut<'a>(&'a mut self) -> MutTableItems<'a, T> {
        MutTableItems {
            iter: self.data.iter_mut(),
            width: self.width,
            height: self.height
        }
    }
}

pub struct TableItems<'a, T: 'a> {
    iter: Items<'a, T>,
    width: uint,
    height: uint
}

impl<'a, T> Iterator<&'a T> for TableItems<'a, T> {
    #[inline]
    fn next(&mut self) -> Option<&'a T> {
        self.iter.next()
    }
}

pub struct MutTableItems<'a, T: 'a> {
    iter: MutItems<'a, T>,
    width: uint,
    height: uint
}

impl<'a, T> Iterator<&'a mut T> for MutTableItems<'a, T> {
    #[inline]
    fn next(&mut self) -> Option<&'a mut T> {
        self.iter.next()
    }
}

pub struct TableEnumerate<T> {
    iter: T,
    height: uint,
    width: uint,
    row_count: uint,
    column_count: uint
}

impl<'a, A, T: Iterator<A>> Iterator<((uint, uint), A)> for TableEnumerate<T> {
    #[inline]
    fn next(&mut self) -> Option<((uint, uint), A)> {
        match self.iter.next() {
            Some(value) => {
                let ret = Some(((self.row_count, self.column_count), value));

                self.column_count += 1;
                if self.column_count == self.width {
                    self.row_count += 1;
                    self.column_count = 0;
                }
                ret
            },
            None => None
        }
    }

    #[inline]
    fn size_hint(&self) -> (uint, Option<uint>) {
        self.iter.size_hint()
    }
}

impl<'a, T> TableItems<'a, T> {
    pub fn enumerate_2d(self) -> TableEnumerate<TableItems<'a, T>> {
        let width = self.width;
        let height = self.height;

        TableEnumerate {
            iter: self,
            width: width,
            height: height,
            row_count: 0,
            column_count: 0
        }
    }
}

impl<'a, T> MutTableItems<'a, T> {
    pub fn enumerate_2d(self) -> TableEnumerate<MutTableItems<'a, T>> {
        let width = self.width;
        let height = self.height;

        TableEnumerate {
            iter: self,
            width: width,
            height: height,
            row_count: 0,
            column_count: 0
        }
    }
}