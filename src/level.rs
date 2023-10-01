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

impl Solution {
    pub fn parse(s: Vec<&str>) -> Solution {
        let mut solution = Solution {
            placements: Vec::new(),
        };
        for (row, line) in s.iter().enumerate() {
            for (column, c) in line.as_bytes().iter().enumerate() {
                // Skip cells with background objects.
                if [b'.', b'g', b'x'].contains(c) {
                    continue;
                }
                solution.placements.push(Placement {
                    position: Position { row, column },
                })
            }
        }
        solution
    }
}

const DROW: [i32; 4] = [1, 0, -1, 0];
const DCOL: [i32; 4] = [0, 1, 0, -1];

#[derive(Debug)]
enum ViolationType {
    AdjacentHouse,
}

#[derive(Debug)]
pub struct PlacementViolation {
    pub house_index: usize,
    pub violation: ViolationType,
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
    placement_violations: Vec<PlacementViolation>,
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

        let mut found_house = false;
        for d in 0..4 {
            let nrow = position.row as i32 + DROW[d];
            let ncol = position.column as i32 + DCOL[d];
            if !puzzle.is_valid(nrow, ncol) {
                continue;
            }

            let nrow = nrow as usize;
            let ncol = ncol as usize;
            if has_house[nrow][ncol] {
                found_house = true;
                break;
            }
        }

        if found_house {
            placement_violations.push(PlacementViolation {
                house_index: index,
                violation: ViolationType::AdjacentHouse,
            })
        }
    }

    ValidationResult {
        row_status,
        col_status,
        placement_violations,
    }
}

pub struct GameLevel {
    pub name: String,
    pub puzzle: Puzzle,
    pub solution: Solution,
}

#[rustfmt::skip]
pub fn first_level() -> GameLevel {
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
        solution: Solution::parse(vec![
           "1gT", 
           "11g",
           "g11",
        ]),
    }
}

pub fn all_levels() -> Vec<GameLevel> {
    vec![first_level()]
}
