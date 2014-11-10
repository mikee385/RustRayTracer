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
    pub fn iter<'a>(&'a self) -> TableItems<Items<'a, T>> {
        self.data.iter().as_table(self.width, self.height)
    }

    #[inline]
    pub fn iter_mut<'a>(&'a mut self) -> TableItems<MutItems<'a, T>> {
        self.data.iter_mut().as_table(self.width, self.height)
    }

    #[inline]
    pub fn as_slice<'a>(&'a self) -> &'a [T] {
        self.data.as_slice()
    }

    #[inline]
    pub fn as_mut_slice<'a>(&'a mut self) -> &'a mut [T] {
        self.data.as_mut_slice()
    }
}

pub struct TableItems<T> {
    iter: T,
    width: uint,
    height: uint
}

impl<A, T: Iterator<A>> Iterator<A> for TableItems<T> {
    #[inline]
    fn next(&mut self) -> Option<A> {
        self.iter.next()
    }
}

pub trait AsTable<'a, T> {
    fn as_table(self, width: uint, height: uint) -> TableItems<Self> {
        TableItems {
            iter: self,
            width: width,
            height: height
        }
    }
}

impl<'a, A, T: Iterator<A>> AsTable<'a, T> for T {}

pub struct TableEnumerate<T> {
    iter: T,
    row_count: uint,
    column_count: uint
}

impl<A, T: Iterator<A>> Iterator<((uint, uint), A)> for TableEnumerate<TableItems<T>> {
    #[inline]
    fn next(&mut self) -> Option<((uint, uint), A)> {
        match self.iter.next() {
            Some(value) => {
                let ret = Some(((self.row_count, self.column_count), value));

                self.column_count += 1;
                if self.column_count == self.iter.width {
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

impl<T> TableItems<T> {
    pub fn enumerate_2d(self) -> TableEnumerate<TableItems<T>> {
        TableEnumerate {
            iter: self,
            row_count: 0,
            column_count: 0
        }
    }

    pub fn enumerate_2d_from(self, start: (uint, uint)) -> TableEnumerate<TableItems<T>> {
        let (row_start, column_start) = start;

        TableEnumerate {
            iter: self,
            row_count: row_start,
            column_count: column_start
        }
    }

    pub fn enumerate_2d_from_index(self, index: uint) -> TableEnumerate<TableItems<T>> {
        let row_start = index / self.width;
        let column_start = index % self.width;

        TableEnumerate {
            iter: self,
            row_count: row_start,
            column_count: column_start
        }
    }
}
