use std::convert;
use std::ops;

impl<T> ops::Index<Point> for Vec<Vec<T>> {
    type Output = T;
    fn index(&self, point: Point) -> &Self::Output {
        &self[point.row][point.column]
    }
}

impl<T> ops::IndexMut<Point> for Vec<Vec<T>> {
    fn index_mut(&mut self, point: Point) -> &mut Self::Output {
        &mut self[point.row][point.column]
    }
}

// TODO: 後でenumの名前を変える
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum State {
    Yes,
    No,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PointState {
    NotMarked,
    Marked(State),
}

impl convert::From<PointState> for State {
    fn from(value: PointState) -> Self {
        match value {
            PointState::Marked(State::Yes) => State::Yes,
            _ => State::No,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Size {
    row: usize,
    column: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Point {
    row: usize,
    column: usize,
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
    answer_board: Vec<Vec<State>>,
    row_hints: Vec<Vec<usize>>,
    column_hints: Vec<Vec<usize>>,
    working_board: Vec<Vec<PointState>>,
    board_size: Size,
}

// publib fn
impl Problem {
    pub fn from(answer_board: Vec<Vec<State>>) -> Self {
        let row = answer_board.len();
        let column = answer_board[0].len();

        // validation check
        for vec in &answer_board[1..] {
            if column != vec.len() {
                panic!();
            }
        }

        let board_size = Size { row, column };
        let working_board = vec![vec![PointState::NotMarked; column]; row];
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

    pub fn board_size(&self) -> Size {
        self.board_size
    }

    pub fn get_answer_board(&self, point: Point) -> State {
        self.answer_board[point]
    }

    pub fn get_working_board(&self, point: Point) -> PointState {
        self.working_board[point]
    }

    pub fn set_working_board(&mut self, point: Point, p_state: PointState) {
        self.working_board[point] = p_state;
    }

    pub fn wrong_points(&self) -> Vec<Point> {
        let mut point_vec = Vec::new();
        for row in 0..self.board_size().row {
            for column in 0..self.board_size().column {
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
        self.row_hints = vec![vec![]; self.board_size.row];
        for row in 0..self.board_size.row {
            let hint = &mut self.row_hints[row];
            let mut len = 0;
            for column in 0..self.board_size.column {
                if self.answer_board[Point { row, column }] == State::Yes {
                    len += 1;
                } else {
                    if len != 0 {
                        hint.push(len);
                        len = 0;
                    }
                }
            }
            if hint.is_empty() {
                hint.push(0);
            }
            hint[0] += len;
        }

        self.column_hints = vec![vec![]; self.board_size.column];
        for column in 0..self.board_size.column {
            let hint = &mut self.column_hints[column];
            let mut len = 0;
            for row in 0..self.board_size.row {
                if self.answer_board[Point { row, column }] == State::Yes {
                    len += 1;
                } else {
                    if len != 0 {
                        hint.push(len);
                        len = 0;
                    }
                }
            }
            if hint.is_empty() {
                hint.push(0);
            }
            hint[0] += len;
        }
    }
}
