mod game;
use game::*;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use std::{
    env,
    error::Error,
    fs::File,
    io::{self, prelude::*, BufReader},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::Paragraph,
    Frame, Terminal,
};

struct App {
    controller: ProblemController,
}

fn main() -> Result<(), Box<dyn Error>> {
    let argv: Vec<String> = env::args().collect();
    let path = match argv.get(1) {
        Some(p) => p,
        None => return Err("set path to the problem file in commandline option.".into()),
    };
    let file = File::open(path)?;
    let file = BufReader::new(file);

    let format_err: Result<(), Box<dyn Error>> = Err("invalid format.".into());

    let mut lines = file.lines().map(|l| l.unwrap());
    let line = match lines.next() {
        Some(l) => l,
        None => return format_err,
    };
    let v: Vec<&str> = line.split_whitespace().collect();
    if v.len() != 2 {
        return format_err;
    }
    let row_size: usize = v[0].parse()?;
    let column_size: usize = v[1].parse()?;
    let mut problem: Vec<Vec<Mark>> = vec![vec![Mark::No; column_size]; row_size];
    for row in &mut problem {
        if let Some(line) = lines.next() {
            let v: Vec<&str> = line.split_whitespace().collect();
            if v.len() != column_size {
                return format_err;
            }
            for (c, str) in &mut row.iter_mut().zip(v) {
                *c = match str {
                    "0" => Mark::No,
                    _ => Mark::Yes,
                };
            }
        } else {
            return format_err;
        }
    }
    if let Some(_) = lines.next() {
        return format_err;
    }

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    execute!(stdout, Clear(ClearType::All)).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App {
        controller: ProblemController::from(Problem::from(problem)),
    };
    let res = run_app(&mut terminal, app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Char('h') => app.controller.key_move_left(),
                KeyCode::Char('j') => app.controller.key_move_down(),
                KeyCode::Char('k') => app.controller.key_move_up(),
                KeyCode::Char('l') => app.controller.key_move_right(),
                KeyCode::Char('d') => app.controller.key_mark_yes(),
                KeyCode::Char('f') => app.controller.key_mark_no(),
                _ => (),
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let row_hints_width = row_hints_width(app);
    let column_hints_height = column_hints_height(app);
    let size = app.controller.board_size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Min(0),
            ]
            .as_ref(),
        )
        .split(f.size());
    if app.controller.is_correct_answer() {
        let clear_text = Span::styled("congratulations!", Style::default().add_modifier(Modifier::RAPID_BLINK));
        let clear_paragraph = Paragraph::new(clear_text);
        f.render_widget(clear_paragraph, chunks[0]);
    }
    let game_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Length(row_hints_width),
                Constraint::Length(2 * size.column as u16),
                Constraint::Min(0),
            ]
            .as_ref(),
        )
        .split(chunks[1]);
    let tmp_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(column_hints_height),
                Constraint::Length(size.row as u16),
            ]
            .as_ref(),
        )
        .split(game_chunks[0]);
    let row_hint_chunks = tmp_chunks[1];

    let row_hints_span: Vec<Spans> = (0..size.row)
        .cycle()
        .skip(app.controller.point().row as usize + size.row - size.row / 2)
        .take(size.row)
        .map(|i| {
            Spans::from(Span::raw(
                app.controller.get_row_hints()[i]
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(" "),
            ))
        })
        .collect();
    let row_hints_paragraph = Paragraph::new(row_hints_span).alignment(Alignment::Right);
    f.render_widget(row_hints_paragraph, row_hint_chunks);
    let tmp_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(column_hints_height),
                Constraint::Length(size.row as u16),
            ]
            .as_ref(),
        )
        .split(game_chunks[1]);
    let column_hint_chunk = tmp_chunks[0];
    let board_chunk = tmp_chunks[1];
    let column_hints_span: Vec<Spans> = (0..column_hints_height)
        .map(|i| i as usize)
        .map(|i| {
            Spans::from(Span::raw(
                (0..size.column)
                    .cycle()
                    .skip(app.controller.point().column as usize + size.column - size.column / 2)
                    .take(size.column)
                    .map(|j| {
                        let v = &app.controller.get_column_hints()[j];
                        if i + 1 <= v.len() {
                            format!("{:>2}", v[v.len() - i - 1])
                        } else {
                            "  ".to_string()
                        }
                    })
                    .collect::<Vec<String>>()
                    .join(""),
            ))
        })
        .rev()
        .collect();
    let column_hints_paragraph = Paragraph::new(column_hints_span);
    f.render_widget(column_hints_paragraph, column_hint_chunk);
    let board_span: Vec<Spans> = (0..size.row)
        .cycle()
        .skip(app.controller.point().row as usize + size.row - size.row / 2)
        .take(size.row)
        .map(|i| {
            Spans::from(
                (0..size.column)
                    .cycle()
                    .skip(app.controller.point().column as usize + size.column - size.column / 2)
                    .take(size.column)
                    .map(|j| {
                        let str = match app.controller.get_working_board(Point {
                            row: i as isize,
                            column: j as isize,
                        }) {
                            State::NotMarked => " .",
                            State::Marked(Mark::Yes) => "[]",
                            State::Marked(Mark::No) => " !",
                        };
                        let style: Style = if app
                            .controller
                            .point()
                            .normalize(app.controller.board_size())
                            == (Point {
                                row: i as isize,
                                column: j as isize,
                            }) {
                            Style::default().add_modifier(Modifier::REVERSED)
                        } else {
                            Style::default()
                        };
                        Span::styled(str, style)
                    })
                    .collect::<Vec<Span>>(),
            )
        })
        .collect();
    let board_paragraph = Paragraph::new(board_span);
    f.render_widget(board_paragraph, board_chunk);
}

fn row_hints_width(app: &App) -> u16 {
    app.controller
        .get_row_hints()
        .iter()
        .map(|v: &Vec<usize>| -> usize {
            v.len() - 1
                + v.iter()
                    .map(|u| if *u == 0 { 1 } else { u.ilog10() as usize + 1 })
                    .sum::<usize>()
        })
        .max()
        .unwrap() as u16
}

fn column_hints_height(app: &App) -> u16 {
    app.controller
        .get_column_hints()
        .iter()
        .map(|v| v.len() as u16)
        .max()
        .unwrap()
}
