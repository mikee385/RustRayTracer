#![allow(dead_code)]

use std::slice::{Items, MutItems};
use std::vec::{Vec};

pub struct Table<T> {
    dimensions: (uint, uint),
    data: Vec<T>
}

impl<T: Clone> Table<T> {
    pub fn from_elem(dimensions: (uint, uint), value: T) -> Table<T> {
        let (width, height) = dimensions;

        Table {
            dimensions: dimensions,
            data: Vec::from_elem(width * height, value)
        }
    }
}

impl<T> Table<T> {
    #[inline]
    fn get_vec_index(&self, index: (uint, uint)) -> uint {
        let (row, column) = index;
        let (width, height) = self.dimensions;

        if row >= height {
            panic!("Table::get_index: `row` overflow ({} >= {})", row, height)
        }
        if column >= width {
            panic!("Table::get_index: `column` overflow ({} >= {})", column, width)
        }

        row * width + column
    }

    pub fn get_dimensions(&self) -> (uint, uint) {
        self.dimensions
    }
    
    #[inline]
    pub fn set(&mut self, index: (uint, uint), value: T) {
        let vec_index = self.get_vec_index(index);
        self.data[vec_index] = value;
    }
    
    #[inline]
    pub fn get(&self, index: (uint, uint)) -> &T {
        let vec_index = self.get_vec_index(index);
        &self.data[vec_index]
    }

    #[inline]
    pub fn iter<'a>(&'a self) -> TableItems<Items<'a, T>> {
        self.data.iter().as_table(self.dimensions)
    }

    #[inline]
    pub fn iter_mut<'a>(&'a mut self) -> TableItems<MutItems<'a, T>> {
        self.data.iter_mut().as_table(self.dimensions)
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
    dimensions: (uint, uint)
}

impl<A, T: Iterator<A>> Iterator<A> for TableItems<T> {
    #[inline]
    fn next(&mut self) -> Option<A> {
        self.iter.next()
    }
}

pub trait AsTable<'a, T> {
    fn as_table(self, dimensions: (uint, uint)) -> TableItems<Self> {
        TableItems {
            iter: self,
            dimensions: dimensions
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
        let (width, _) = self.iter.dimensions;

        match self.iter.next() {
            Some(value) => {
                let ret = Some(((self.row_count, self.column_count), value));

                self.column_count += 1;
                if self.column_count == width {
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
        let (width, _) = self.dimensions;
        let row_start = index / width;
        let column_start = index % width;

        TableEnumerate {
            iter: self,
            row_count: row_start,
            column_count: column_start
        }
    }
}
