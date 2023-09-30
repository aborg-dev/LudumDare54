use core::fmt;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum BuildingType {
    House,
    Trash,
    Hermit,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CellType {
    Grass,
    Hole,
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
                let c = match self.field[row][column] {
                    CellType::Grass => "g",
                    CellType::Hole => ".",
                };
                write!(formatter, "{}", c)?
            }
            write!(formatter, "\n")?
        }
        write!(formatter, "{:?}", self.building_count)
    }
}

pub fn field_from_size(rows: usize, columns: usize) -> Vec<Vec<CellType>> {
    vec![vec![CellType::Grass; columns]; rows]
}

pub struct Solution {
    building_location: Vec<(BuildingType, i32, i32)>,
}

pub fn validate_solution(solution: &Solution, level: &Level) -> bool {
    // Check that we have the right count of each building.
    let mut building_count = HashMap::new();
    for (building, _, _) in &solution.building_location {
        *building_count.entry(*building).or_insert(0) += 1;
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
            building_location: vec![
                (BuildingType::House, 0, 0),
            ],
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
            building_location: vec![
                (BuildingType::House, 0, 0),
            ],
        },
    )
}
