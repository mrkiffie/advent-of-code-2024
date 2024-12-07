use std::ops::{Add, Div, Mul, Rem};

use crate::Vec2;

#[derive(Debug)]
pub struct Grid<'a> {
    pub rows: usize,
    pub cols: usize,
    input: &'a str,
}

impl<'a> Grid<'a> {
    #[inline]
    pub fn new(input: &'a str) -> Self {
        let mut lines = input.lines();
        let cols = lines.next().expect("there should be lines").len();
        let rows = input.len().div(cols);

        Self { rows, cols, input }
    }
    #[inline]
    pub fn index_to_vec2(&self, index: usize) -> Vec2 {
        // + 1 offset to account for newlines
        let x = index.rem(self.cols + 1) as i32;
        let y = index.div(self.cols + 1) as i32;

        Vec2 { x, y }
    }
    #[inline]
    pub fn point_to_index(&self, point: &Vec2) -> Option<usize> {
        if (0..self.cols as i32).contains(&point.x) && (0..self.rows as i32).contains(&point.y) {
            Some((self.cols + 1).mul(point.y as usize).add(point.x as usize))
        } else {
            None
        }
    }
    #[inline]
    pub fn get(&self, point: &Vec2) -> Option<char> {
        if (0..self.cols as i32).contains(&point.x) && (0..self.rows as i32).contains(&point.y) {
            let i = self.point_to_index(point)?;
            self.input.get(i..i + 1).and_then(|s| s.chars().next())
        } else {
            None
        }
    }
}
