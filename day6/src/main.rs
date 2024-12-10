use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    ops::Add,
};

struct Lab {
    grid: Vec<Vec<u8>>,
    guard: Guard,
}

struct Guard {
    pos: Pos,
    dir: Dir,
}

#[derive(Default, Clone, Copy, Eq, Hash, PartialEq)]
struct Pos(i32, i32);

impl Add<Off> for Pos {
    type Output = Self;

    fn add(self, Off(dx, dy): Off) -> Self::Output {
        Pos(self.0 + dx, self.1 + dy)
    }
}

struct Off(i32, i32);

#[derive(Clone, Copy)]
enum Dir {
    Up,
    Down,
    Right,
    Left,
}

impl Dir {
    fn offset(self) -> Off {
        match self {
            Dir::Up => Off(-1, 0),
            Dir::Down => Off(1, 0),
            Dir::Right => Off(0, 1),
            Dir::Left => Off(0, -1),
        }
    }

    fn turn(self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Down => Dir::Left,
            Dir::Right => Dir::Down,
            Dir::Left => Dir::Up,
        }
    }
}

impl From<&str> for Lab {
    fn from(input: &str) -> Self {
        let grid: Vec<Vec<u8>> = input.lines().map(|line| line.bytes().collect()).collect();
        let pos = grid
            .iter()
            .enumerate()
            .find_map(|(x, row)| {
                row.iter()
                    .position(|&c| c == b'^')
                    .map(|y| Pos(x as i32, y as i32))
            })
            .unwrap_or_default();

        Self {
            grid,
            guard: Guard { pos, dir: Dir::Up },
        }
    }
}

impl Lab {
    fn get(&self, Pos(x, y): Pos) -> Option<u8> {
        self.grid.get(x as usize)?.get(y as usize).copied()
    }

    fn walk(&mut self) -> HashSet<Pos> {
        let mut visited = HashSet::new();

        loop {
            visited.insert(self.guard.pos);
            let next = self.guard.pos + self.guard.dir.offset();

            match self.get(next) {
                Some(b'#') => self.guard.dir = self.guard.dir.turn(),
                Some(_) => self.guard.pos = next,
                None => break,
            }
        }
        visited
    }
}

fn main() {
    let path = "input.txt";
    let file = File::open(path).expect("Failed to open the file");
    let reader = BufReader::new(file);
    let lines: String = reader
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>()
        .join("\n");

    let lines_as_str: &str = &lines;

    let count = Lab::from(lines_as_str).walk().len() as u32;
    println!("{count}")
}
