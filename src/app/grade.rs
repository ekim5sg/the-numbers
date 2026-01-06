use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Grade {
    G1,
    G2,
    G3,
    G4,
    G5,
}

impl Grade {
    pub fn label(&self) -> &'static str {
        match self {
            Grade::G1 => "Grade 1",
            Grade::G2 => "Grade 2",
            Grade::G3 => "Grade 3",
            Grade::G4 => "Grade 4",
            Grade::G5 => "Grade 5",
        }
    }

    pub fn from_u8(v: u8) -> Grade {
        match v {
            1 => Grade::G1,
            2 => Grade::G2,
            3 => Grade::G3,
            4 => Grade::G4,
            _ => Grade::G5,
        }
    }

    pub fn as_u8(&self) -> u8 {
        match self {
            Grade::G1 => 1,
            Grade::G2 => 2,
            Grade::G3 => 3,
            Grade::G4 => 4,
            Grade::G5 => 5,
        }
    }
}