use rand::Rng;
use std::convert;
use std::ops;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Mark {
    Yes,
    No,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum State {
    NotMarked,
    Marked(Mark),
}

impl convert::From<Mark> for State {
    fn from(value: Mark) -> Self {
        State::Marked(value)
    }
}

impl convert::From<State> for Mark {
    fn from(value: State) -> Self {
        match value {
            State::Marked(Mark::Yes) => Mark::Yes,
            _ => Mark::No,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Size {
    pub row: usize,
    pub column: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Point {
    pub row: isize,
    pub column: isize,
}

impl<T> ops::Index<Point> for Vec<Vec<T>> {
    type Output = T;
    fn index(&self, point: Point) -> &Self::Output {
        &self[point.row as usize][point.column as usize]
    }
}

impl<T> ops::IndexMut<Point> for Vec<Vec<T>> {
    fn index_mut(&mut self, point: Point) -> &mut Self::Output {
        &mut self[point.row as usize][point.column as usize]
    }
}

impl ops::Add<Self> for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            row: self.row + rhs.row,
            column: self.column + rhs.column,
        }
    }
}

impl ops::AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        self.row += other.row;
        self.column += other.column;
    }
}

impl ops::Sub<Self> for Point {
    type Output = Point;
    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            row: self.row - rhs.row,
            column: self.column - rhs.column,
        }
    }
}

impl ops::SubAssign for Point {
    fn sub_assign(&mut self, other: Self) {
        self.row -= other.row;
        self.column -= other.column;
    }
}

pub struct Problem {
    answer_board: Vec<Vec<Mark>>,
    row_hints: Vec<Vec<usize>>,
    column_hints: Vec<Vec<usize>>,
    working_board: Vec<Vec<State>>,
    board_size: Size,
}

impl convert::From<Vec<Vec<Mark>>> for Problem {
    fn from(answer_board: Vec<Vec<Mark>>) -> Self {
        let row = answer_board.len();
        let column = answer_board[0].len();

        // validation check
        for vec in &answer_board[1..] {
            if column != vec.len() {
                panic!();
            }
        }

        let board_size = Size { row, column };
        let working_board = vec![vec![State::NotMarked; column]; row];
        let mut problem = Self {
            answer_board,
            row_hints: Vec::new(),
            column_hints: Vec::new(),
            working_board,
            board_size,
        };

        problem.set_hint();
        problem
    }
}

// publib fn
impl Problem {
    pub fn board_size(&self) -> Size {
        self.board_size
    }

    pub fn get_answer_board(&self, point: Point) -> Mark {
        self.answer_board[point]
    }

    pub fn get_working_board(&self, point: Point) -> State {
        self.working_board[point]
    }

    pub fn set_working_board(&mut self, point: Point, p_state: State) {
        self.working_board[point] = p_state;
    }

    pub fn get_row_hints(&self) -> &Vec<Vec<usize>> {
        &self.row_hints
    }

    pub fn get_column_hints(&self) -> &Vec<Vec<usize>> {
        &self.column_hints
    }

    pub fn wrong_points(&self) -> Vec<Point> {
        let mut point_vec = Vec::new();
        for row in 0..self.board_size().row as isize {
            for column in 0..self.board_size().column as isize {
                let point = Point { row, column };
                if self.answer_board[point] != self.working_board[point].into() {
                    point_vec.push(point);
                }
            }
        }

        point_vec
    }

    pub fn is_correct_answer(&self) -> bool {
        self.wrong_points().is_empty()
    }
}

// private fn
impl Problem {
    fn set_hint(&mut self) {
        let mut rng = rand::thread_rng();
        self.row_hints = vec![vec![]; self.board_size.row];

        for row in 0..self.board_size.row as isize {
            let hint = &mut self.row_hints[row as usize];
            let mut len = 0;
            let begin_yes = self.answer_board[Point { row, column: 0 }] == Mark::Yes;
            for column in 0..self.board_size.column as isize {
                if self.answer_board[Point { row, column }] == Mark::Yes {
                    len += 1;
                } else {
                    if len != 0 {
                        hint.push(len);
                        len = 0;
                    }
                }
            }
            if hint.is_empty() {
                hint.push(len);
            } else if begin_yes {
                hint[0] += len;
            } else if len != 0 {
                hint.push(len);
            }
            let rotate = rng.gen_range(0..hint.len());
            hint.rotate_left(rotate);
        }

        self.column_hints = vec![vec![]; self.board_size.column];
        for column in 0..self.board_size.column as isize {
            let hint = &mut self.column_hints[column as usize];
            let mut len = 0;
            let begin_yes = self.answer_board[Point { row: 0, column }] == Mark::Yes;
            for row in 0..self.board_size.row as isize {
                if self.answer_board[Point { row, column }] == Mark::Yes {
                    len += 1;
                } else {
                    if len != 0 {
                        hint.push(len);
                        len = 0;
                    }
                }
            }
            if hint.is_empty() {
                hint.push(len);
            } else if begin_yes {
                hint[0] += len;
            } else if len != 0 {
                hint.push(len);
            }
            let rotate = rng.gen_range(0..hint.len());
            hint.rotate_left(rotate);
        }
    }
}
