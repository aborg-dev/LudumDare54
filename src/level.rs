use core::fmt;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum BuildingType {
    House,
    Trash,
    Hermit,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CellType {
    Grass,
    Hole,
}

#[derive(Debug)]
pub struct Level {
    buildings: HashMap<BuildingType, usize>,
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
        write!(formatter, "{:?}", self.buildings)
    }
}

pub fn field_from_size(rows: usize, columns: usize) -> Vec<Vec<CellType>> {
    vec![vec![CellType::Grass; columns]; rows]
}

pub fn first_level() -> Level {
    Level {
        buildings: HashMap::from([(BuildingType::House, 5), (BuildingType::Trash, 1)]),
        field: field_from_size(3, 3),
    }
}

pub fn second_level() -> Level {
    Level {
        buildings: HashMap::from([(BuildingType::House, 4), (BuildingType::Hermit, 4)]),
        field: field_from_size(3, 3)
    }
}
