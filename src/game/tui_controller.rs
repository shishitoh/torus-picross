use rand::Rng;
use std::convert;

use super::problem::*;

impl Point {
    pub fn normalize(mut self, size: Size) -> Point {
        self.row %= size.row as isize;
        if self.row < 0 {
            self.row += size.row as isize
        }
        self.column %= size.column as isize;
        if self.column < 0 {
            self.column += size.column as isize
        }
        self
    }
}

pub enum Direction {
    N,
    E,
    S,
    W,
}

impl convert::From<Direction> for Point {
    fn from(dir: Direction) -> Self {
        use Direction::*;
        match dir {
            N => Point { row: -1, column: 0 },
            E => Point { row: 0, column: 1 },
            S => Point { row: 1, column: 0 },
            W => Point { row: 0, column: -1 },
        }
    }
}

pub struct ProblemController {
    problem: Problem,
    point: Point,
}

impl convert::From<Problem> for ProblemController {
    fn from(problem: Problem) -> Self {
        let size = problem.board_size();
        let mut rng = rand::thread_rng();
        let point = Point {
            row: rng.gen_range(0..size.row) as isize,
            column: rng.gen_range(0..size.column) as isize,
        };
        Self { problem, point }
    }
}

// public fn
impl ProblemController {
    pub fn key_move_up(&mut self) {
        self.key_move(Direction::N)
    }

    pub fn key_move_right(&mut self) {
        self.key_move(Direction::E)
    }

    pub fn key_move_down(&mut self) {
        self.key_move(Direction::S)
    }

    pub fn key_move_left(&mut self) {
        self.key_move(Direction::W)
    }

    pub fn key_mark_yes(&mut self) {
        self.key_mark(Mark::Yes);
    }

    pub fn key_mark_no(&mut self) {
        self.key_mark(Mark::No);
    }

    pub fn board_size(&self) -> Size {
        self.problem.board_size()
    }

    pub fn get_working_board(&self, point: Point) -> State {
        self.problem.get_working_board(point)
    }

    pub fn get_row_hints(&self) -> &Vec<Vec<usize>> {
        self.problem.get_row_hints()
    }

    pub fn get_column_hints(&self) -> &Vec<Vec<usize>> {
        self.problem.get_column_hints()
    }

    pub fn point(&self) -> Point {
        self.point.normalize(self.problem.board_size())
    }

    pub fn wrong_points(&self) -> Vec<Point> {
        self.problem.wrong_points()
    }

    pub fn is_correct_answer(&self) -> bool {
        self.problem.is_correct_answer()
    }
}

// private fn
impl ProblemController {
    fn key_move(&mut self, dir: Direction) {
        self.point += dir.into();
    }

    fn key_mark(&mut self, state: Mark) {
        self.point_normalize();
        self.problem.set_working_board(
            self.point,
            match self.problem.get_working_board(self.point) {
                State::Marked(s) if s == state => State::NotMarked,
                _ => State::Marked(state),
            },
        );
    }

    fn point_normalize(&mut self) {
        self.point = self.point.normalize(self.board_size());
    }
}
