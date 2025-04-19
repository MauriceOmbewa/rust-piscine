use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Antigen {
    A,
    B,
    AB,
    O,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

#[derive(Debug)]
pub struct ParseBloodTypeError;

impl fmt::Display for ParseBloodTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid blood type format")
    }
}

impl std::error::Error for ParseBloodTypeError {}

impl FromStr for BloodType {
    type Err = ParseBloodTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {
            return Err(ParseBloodTypeError);
        }

        let antigen = match &s[..s.len() - 1] {
            "A" => Antigen::A,
            "B" => Antigen::B,
            "AB" => Antigen::AB,
            "O" => Antigen::O,
            _ => return Err(ParseBloodTypeError),
        };

        let rh_factor = match &s[s.len() - 1..] {
            "+" => RhFactor::Positive,
            "-" => RhFactor::Negative,
            _ => return Err(ParseBloodTypeError),
        };

        Ok(BloodType { antigen, rh_factor })
    }
}

impl PartialOrd for BloodType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        let antigen_order = match (&self.antigen, &other.antigen) {
            (Antigen::O, Antigen::O) => Ordering::Equal,
            (Antigen::O, _) => Ordering::Less,
            (_, Antigen::O) => Ordering::Greater,
            (Antigen::A, Antigen::A) => Ordering::Equal,
            (Antigen::A, _) => Ordering::Less,
            (_, Antigen::A) => Ordering::Greater,
            (Antigen::B, Antigen::B) => Ordering::Equal,
            (Antigen::B, _) => Ordering::Less,
            (_, Antigen::B) => Ordering::Greater,
            (Antigen::AB, Antigen::AB) => Ordering::Equal,
        };

        if antigen_order == Ordering::Equal {
            match (&self.rh_factor, &other.rh_factor) {
                (RhFactor::Negative, RhFactor::Negative) => Ordering::Equal,
                (RhFactor::Negative, RhFactor::Positive) => Ordering::Less,
                (RhFactor::Positive, RhFactor::Negative) => Ordering::Greater,
                (RhFactor::Positive, RhFactor::Positive) => Ordering::Equal,
            }
        } else {
            antigen_order
        }
    }
}

impl fmt::Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let antigen_str = match self.antigen {
            Antigen::A => "A",
            Antigen::B => "B",
            Antigen::AB => "AB",
            Antigen::O => "O",
        };

        let rh_str = match self.rh_factor {
            RhFactor::Positive => "+",
            RhFactor::Negative => "-",
        };

        write!(f, "{}{}", antigen_str, rh_str)
    }
}

impl BloodType {
    pub fn can_receive_from(&self, other: &BloodType) -> bool {
        if self.rh_factor == RhFactor::Negative && other.rh_factor == RhFactor::Positive {
            return false;
        }

        match self.antigen {
            Antigen::O => other.antigen == Antigen::O,
            Antigen::A => other.antigen == Antigen::A || other.antigen == Antigen::O,
            Antigen::B => other.antigen == Antigen::B || other.antigen == Antigen::O,
            Antigen::AB => true, // AB can receive from any antigen type
        }
    }

    pub fn donors(&self) -> Vec<BloodType> {
        let all_blood_types = Self::all_blood_types();
        all_blood_types
            .into_iter()
            .filter(|bt| self.can_receive_from(bt))
            .collect()
    }

    pub fn recipients(&self) -> Vec<BloodType> {
        let all_blood_types = Self::all_blood_types();
        all_blood_types
            .into_iter()
            .filter(|bt| bt.can_receive_from(self))
            .collect()
    }

    fn all_blood_types() -> Vec<BloodType> {
        let antigens = [Antigen::A, Antigen::B, Antigen::AB, Antigen::O];
        let rh_factors = [RhFactor::Positive, RhFactor::Negative];

        let mut blood_types = Vec::with_capacity(8); // 4 antigens * 2 rh factors = 8 types
        for &antigen in &antigens {
            for &rh_factor in &rh_factors {
                blood_types.push(BloodType { antigen, rh_factor });
            }
        }
        blood_types
    }
}

impl fmt::Display for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}