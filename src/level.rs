use core::fmt;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum BuildingType {
    House,  // 1
    Trash,  // T
    Hermit, // H
}

impl BuildingType {
    pub fn to_char(&self) -> char {
        match self {
            BuildingType::House => '1',
            BuildingType::Trash => 'T',
            BuildingType::Hermit => 'H',
        }
    }

    pub fn from_char(c: char) -> BuildingType {
        match c {
            '1' => BuildingType::House,
            'T' => BuildingType::Trash,
            'H' => BuildingType::Hermit,
            _ => panic!("Unknown building type: {}", c),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CellType {
    Grass,
    Hole,
}

impl CellType {
    pub fn to_char(&self) -> char {
        match self {
            CellType::Grass => 'g',
            CellType::Hole => 'x',
        }
    }
}

#[derive(Debug)]
pub struct Level {
    pub building_count: HashMap<BuildingType, usize>,
    field: Vec<Vec<CellType>>,
}

impl Level {
    pub fn rows(&self) -> usize {
        self.field.len()
    }

    pub fn columns(&self) -> usize {
        self.field[0].len()
    }
}

impl fmt::Display for Level {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.rows() {
            for column in 0..self.columns() {
                write!(formatter, "{}", self.field[row][column].to_char())?
            }
            write!(formatter, "\n")?
        }
        write!(formatter, "{:?}", self.building_count)
    }
}

pub fn field_from_size(rows: usize, columns: usize) -> Vec<Vec<CellType>> {
    vec![vec![CellType::Grass; columns]; rows]
}

pub struct Placement {
    building: BuildingType,
    row: usize,
    column: usize,
}

pub struct Solution {
    placements: Vec<Placement>,
}

pub fn validate_solution(solution: &Solution, level: &Level) -> bool {
    // Check that we have the right count of each building.
    let mut building_count = HashMap::new();
    for placement in &solution.placements {
        *building_count.entry(placement.building).or_insert(0) += 1;
    }
    if level.building_count != building_count {
        return false;
    }
    true
}

pub fn first_level() -> (Level, Solution) {
    (
        Level {
            building_count: HashMap::from([(BuildingType::House, 5), (BuildingType::Trash, 1)]),
            field: field_from_size(3, 3),
        },
        Solution {
            placements: vec![Placement {
                building: BuildingType::House,
                row: 0,
                column: 0,
            }],
        },
    )
}

pub fn second_level() -> (Level, Solution) {
    (
        Level {
            building_count: HashMap::from([(BuildingType::House, 4), (BuildingType::Hermit, 4)]),
            field: field_from_size(3, 3),
        },
        Solution {
            placements: vec![Placement {
                building: BuildingType::House,
                row: 0,
                column: 0,
            }],
        },
    )
}
