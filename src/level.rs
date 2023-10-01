use core::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CellType {
    Grass,
    Tree,
    Lake,
    Mountain,
}

impl CellType {
    pub fn to_char(self) -> char {
        match self {
            CellType::Grass => '.',
            CellType::Tree => 'T',
            CellType::Lake => 'L',
            CellType::Mountain => 'M',
        }
    }

    pub fn from_char(c: u8) -> CellType {
        match c {
            b'.' => CellType::Grass,
            b'T' => CellType::Tree,
            b'L' => CellType::Lake,
            b'M' => CellType::Mountain,
            _ => panic!("Unknown cell type: {}", c),
        }
    }
}

#[derive(Debug)]
pub struct Puzzle {
    pub row_count: Vec<usize>,
    pub col_count: Vec<usize>,
    pub field: Vec<Vec<CellType>>,
}

impl Puzzle {
    pub fn rows(&self) -> usize {
        self.field.len()
    }

    pub fn columns(&self) -> usize {
        self.field[0].len()
    }

    pub fn is_valid(&self, row: i32, col: i32) -> bool {
        row >= 0 && row < self.rows() as i32 && col >= 0 && col < self.columns() as i32
    }
}

impl fmt::Display for Puzzle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.rows() {
            for column in 0..self.columns() {
                write!(formatter, "{}", self.field[row][column].to_char())?
            }
            writeln!(formatter)?
        }
        writeln!(formatter, "Row count: {:?}", self.row_count)?;
        writeln!(formatter, "Col count: {:?}", self.col_count)?;
        Ok(())
    }
}

pub fn field_from_size(rows: usize, columns: usize) -> Vec<Vec<CellType>> {
    vec![vec![CellType::Grass; columns]; rows]
}

pub fn parse_field(s: Vec<&str>) -> Vec<Vec<CellType>> {
    let mut field = field_from_size(s.len(), s[0].len());
    for (row, line) in s.iter().enumerate() {
        for (column, c) in line.as_bytes().iter().enumerate() {
            // Skip cells with house objects.
            if [b'x'].contains(c) {
                continue;
            }
            field[row][column] = CellType::from_char(*c);
        }
    }
    field
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Position {
    pub row: usize,
    pub column: usize,
}

#[derive(Debug)]
pub struct Placement {
    pub position: Position,
}

#[derive(Debug, Default)]
pub struct Solution {
    pub placements: Vec<Placement>,
}

const DROW: [i32; 4] = [1, 0, -1, 0];
const DCOL: [i32; 4] = [0, 1, 0, -1];

#[derive(Debug)]
pub enum ViolationType {
    AdjacentHouse,
}

#[derive(Debug)]
pub struct PlacementViolation {
    pub house_index: usize,
    pub violation: ViolationType,
}

#[derive(Debug)]
pub enum ConstraintViolationType {
    Underflow,
    Match,
    Overflow,
}

#[derive(Debug)]
pub struct ConstraintViolation {
    pub position: Position,
    pub violation: ConstraintViolationType,
}

#[derive(Debug, Clone)]
pub enum LineStatus {
    Underflow,
    Match,
    Overflow,
}

#[derive(Debug)]
pub struct ValidationResult {
    pub row_status: Vec<LineStatus>,
    pub col_status: Vec<LineStatus>,
    pub placement_violations: Vec<PlacementViolation>,
    pub constraint_violations: Vec<ConstraintViolation>,
}

impl fmt::Display for ValidationResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(formatter, "Rows: {:?}", self.row_status)?;
        writeln!(formatter, "Columns: {:?}", self.col_status)?;
        for violation in &self.placement_violations {
            writeln!(
                formatter,
                "{}: {:?}",
                violation.house_index, violation.violation
            )?
        }
        Ok(())
    }
}

pub fn validate_solution(solution: &Solution, puzzle: &Puzzle) -> ValidationResult {
    let mut placement_violations = Vec::new();

    let mut has_house = vec![vec![false; puzzle.columns()]; puzzle.rows()];
    for placement in &solution.placements {
        has_house[placement.position.row][placement.position.column] = true;
    }

    // Check that each row and column is satisfied.
    let mut row_status = vec![LineStatus::Underflow; puzzle.rows()];
    for row in 0..puzzle.rows() {
        let house_count = has_house[row].iter().filter(|&b| *b).count();
        row_status[row] = match house_count.cmp(&puzzle.row_count[row]) {
            std::cmp::Ordering::Less => LineStatus::Underflow,
            std::cmp::Ordering::Equal => LineStatus::Match,
            std::cmp::Ordering::Greater => LineStatus::Overflow,
        };
    }
    let mut col_status = vec![LineStatus::Underflow; puzzle.columns()];
    for col in 0..puzzle.columns() {
        let mut house_count = 0;
        for row in 0..puzzle.rows() {
            house_count += has_house[row][col] as usize;
        }
        col_status[col] = match house_count.cmp(&puzzle.col_count[col]) {
            std::cmp::Ordering::Less => LineStatus::Underflow,
            std::cmp::Ordering::Equal => LineStatus::Match,
            std::cmp::Ordering::Greater => LineStatus::Overflow,
        };
    }

    // Check that houses don't have other houses nearby.
    for (index, placement) in solution.placements.iter().enumerate() {
        let position = placement.position;
        if count_adjacent_houses(position.row, position.column, &has_house, puzzle) > 0 {
            placement_violations.push(PlacementViolation {
                house_index: index,
                violation: ViolationType::AdjacentHouse,
            })
        }
    }

    let mut constraint_violations = Vec::new();
    for row in 0..puzzle.rows() {
        for col in 0..puzzle.columns() {
            match puzzle.field[row][col] {
                CellType::Grass => {}
                CellType::Tree => {}
                CellType::Lake => {
                    let count = count_houses_in_3x3(row, col, &has_house, puzzle);
                    let t = match count.cmp(&3) {
                        std::cmp::Ordering::Less => ConstraintViolationType::Underflow,
                        std::cmp::Ordering::Equal => ConstraintViolationType::Match,
                        std::cmp::Ordering::Greater => ConstraintViolationType::Overflow,
                    };
                    constraint_violations.push(ConstraintViolation {
                        position: Position { row, column: col },
                        violation: t,
                    });
                }
                CellType::Mountain => {
                    let count = count_diagnoal_houses(row, col, &has_house, puzzle);
                    let t = match count.cmp(&2) {
                        std::cmp::Ordering::Less => ConstraintViolationType::Underflow,
                        std::cmp::Ordering::Equal => ConstraintViolationType::Match,
                        std::cmp::Ordering::Greater => ConstraintViolationType::Overflow,
                    };
                    constraint_violations.push(ConstraintViolation {
                        position: Position { row, column: col },
                        violation: t,
                    });
                }
            };
        }
    }

    ValidationResult {
        row_status,
        col_status,
        placement_violations,
        constraint_violations,
    }
}

pub fn count_diagnoal_houses(
    row: usize,
    col: usize,
    has_house: &Vec<Vec<bool>>,
    puzzle: &Puzzle,
) -> usize {
    let mut count = 0;
    for drow in [-1, 1] {
        for dcol in [-1, 1] {
            for d in 1..puzzle.rows().max(puzzle.columns()) {
                let nrow = row as i32 + drow * d as i32;
                let ncol = col as i32 + dcol * d as i32;
                if !puzzle.is_valid(nrow, ncol) {
                    break;
                }
                let nrow = nrow as usize;
                let ncol = ncol as usize;
                if has_house[nrow][ncol] {
                    count += 1;
                    break;
                }
            }
        }
    }
    count
}

pub fn count_houses_in_3x3(
    row: usize,
    col: usize,
    has_house: &Vec<Vec<bool>>,
    puzzle: &Puzzle,
) -> usize {
    let mut count = 0;
    for drow in -1..=1 {
        for dcol in -1..=1 {
            let nrow = row as i32 + drow;
            let ncol = col as i32 + dcol;
            if !puzzle.is_valid(nrow, ncol) {
                continue;
            }
            let nrow = nrow as usize;
            let ncol = ncol as usize;
            if has_house[nrow][ncol] {
                count += 1;
            }
        }
    }
    count
}

pub fn count_adjacent_houses(
    row: usize,
    col: usize,
    has_house: &Vec<Vec<bool>>,
    puzzle: &Puzzle,
) -> usize {
    let mut count = 0;
    for d in 0..4 {
        let nrow = row as i32 + DROW[d];
        let ncol = col as i32 + DCOL[d];
        if !puzzle.is_valid(nrow, ncol) {
            continue;
        }

        let nrow = nrow as usize;
        let ncol = ncol as usize;
        if has_house[nrow][ncol] {
            count += 1;
            break;
        }
    }
    count
}

pub struct GameLevel {
    pub name: String,
    pub puzzle: Puzzle,
}

#[rustfmt::skip]
pub fn two_takes() -> GameLevel {
    GameLevel {
        name: "two_lakes".into(),
        puzzle: Puzzle {
            field: parse_field(vec![
               "....",
               "L.L.",
               "....",
               "..T.",
            ]),
            row_count: vec![2, 1, 2, 1],
            col_count: vec![2, 1, 1, 2],
        },
    }
}

#[rustfmt::skip]
pub fn trees_4x4() -> GameLevel {
    GameLevel {
        name: "trees_4x4".into(),
        puzzle: Puzzle {
            field: parse_field(vec![
               "..T.",
               ".T..",
               "..T.",
               ".T..",
            ]),
            row_count: vec![2, 1, 2, 1],
            col_count: vec![2, 1, 1, 2],
        },
    }
}

#[rustfmt::skip]
pub fn mountain_4x4() -> GameLevel {
    GameLevel {
        name: "mountain_4x4".into(),
        puzzle: Puzzle {
            field: parse_field(vec![
               "..T.",
               ".M..",
               ".TT.",
               "T...",
            ]),
            row_count: vec![1, 1, 1, 2],
            col_count: vec![1, 2, 1, 1],
        },
    }
}

#[rustfmt::skip]
pub fn lake_and_trees_4x4() -> GameLevel {
    GameLevel {
        name: "lake_and_trees_4x4".into(),
        puzzle: Puzzle {
            field: parse_field(vec![
               "....",
               ".TL.",
               "..T.",
               "....",
            ]),
            row_count: vec![1, 1, 1, 2],
            col_count: vec![1, 2, 1, 1],
        },
    }
}

#[rustfmt::skip]
pub fn mountain_lakes_5x5() -> GameLevel {
    GameLevel {
        name: "mountain_lakes_5x5".into(),
        puzzle: Puzzle {
            field: parse_field(vec![
               ".....",
               ".T.LT",
               "..M..",
               "..TL.",
               ".....",
            ]),
            row_count: vec![2, 1, 1, 2, 1],
            col_count: vec![1, 1, 2, 1, 2],
        },
    }
}

#[rustfmt::skip]
pub fn twin_mountains_5x5() -> GameLevel {
    GameLevel {
        name: "twin_mountains_5x5".into(),
        puzzle: Puzzle {
            field: parse_field(vec![
               ".....",
               ".....",
               ".M.M.",
               ".....",
               ".....",
            ]),
            row_count: vec![2, 1, 0, 1, 2],
            col_count: vec![2, 0, 2, 0, 2],
        },
    }
}

pub fn all_levels() -> Vec<GameLevel> {
    vec![
        two_takes(),
        trees_4x4(),
        mountain_4x4(),
        lake_and_trees_4x4(),
        twin_mountains_5x5(),
        mountain_lakes_5x5(),
    ]
}
