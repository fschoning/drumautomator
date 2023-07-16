use std::cmp::Ordering;
use std::fmt::Display;
use std::ops::{Add, Sub};

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum Unit {
    Whole,
    Half,
    Quarter,
    Eighth,
    Sixteenth,
    ThirtySecondth,
}

impl Default for Unit {
    fn default() -> Self {
        Self::Quarter
    }
}

impl Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Unit::Whole => "_1",
            Unit::Half => "_1_2",
            Unit::Quarter => "_1_4",
            Unit::Eighth => "_1_8",
            Unit::Sixteenth => "_1_16",
            Unit::ThirtySecondth => "_1_32",
        })
    }
}

impl Unit {
    pub fn halfed(&self) -> Self {
        match self {
            Self::Whole => Self::Half,
            Self::Half => Self::Quarter,
            Self::Quarter => Self::Eighth,
            Self::Eighth => Self::Sixteenth,
            Self::Sixteenth => Self::ThirtySecondth,
            Self::ThirtySecondth => Self::ThirtySecondth,
        }
    }
    pub fn doubled(&self) -> Self {
        match self {
            Self::Whole => Self::Whole,
            Self::Half => Self::Whole,
            Self::Quarter => Self::Half,
            Self::Eighth => Self::Quarter,
            Self::Sixteenth => Self::Eighth,
            Self::ThirtySecondth => Self::Sixteenth,
        }
    }
}

impl Unit {
    pub const _1: Self = Unit::Whole;
    pub const _1_2: Self = Unit::Half;
    pub const _1_4: Self = Unit::Quarter;
    pub const _1_8: Self = Unit::Eighth;
    pub const _1_16: Self = Unit::Sixteenth;
    pub const _1_32: Self = Unit::ThirtySecondth;
    pub fn to_ident(&self) -> String {
        format!("{}", self)
    }
    pub fn from_ident(ident: &str) -> Self {
        match ident {
            "_1" => Self::_1,
            "_1_2" => Self::_1_2,
            "_1_4" => Self::_1_4,
            "_1_8" => Self::_1_8,
            "_1_16" => Self::_1_16,
            "_1_32" => Self::_1_32,
            _ => Self::_1_4,
        }
    }
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Debug, Default)]
pub struct Units(pub f32);

impl Units {
    pub const _MIN_ACCURACY: f32 = 0.00001;
    pub const MIN_ACCURACY: Units = Units(Self::_MIN_ACCURACY);
    /*
     * This is quite messy here, the pasition passed checking needs some cleanup,
     * mainly for control the guitar view timing, and seeking next note
     */
    pub const HALF_MIN_ACCURACY: Units = Units(Self::_MIN_ACCURACY * 0.5);

    pub fn is_bigger_than(&self, other: &Units) -> bool {
        self.0 > other.0 + Self::_MIN_ACCURACY
    }
}

impl Display for Units {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Units>({})", self.0)
    }
}

impl Add for Units {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Units(self.0 + rhs.0)
    }
}

impl Sub for Units {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Units(self.0 - rhs.0)
    }
}

impl Units {
    pub fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.0 == other.0 {
            Ordering::Equal
        } else if self.0 < other.0 {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

// https://hellomusictheory.com/learn/tuplets/
#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum Duration {
    Zero,
    Simple(Unit),
    Dotted(Unit),
    Triplet(Unit),
    DottedTriplet(Unit),
}

impl Display for Duration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Duration::Zero => write!(f, "_0"),
            Duration::Simple(d) => d.fmt(f),
            Duration::Dotted(d) => {
                write!(f, "D")?;
                d.fmt(f)
            }
            Duration::Triplet(d) => {
                write!(f, "T")?;
                d.fmt(f)
            }
            Duration::DottedTriplet(d) => {
                write!(f, "DT")?;
                d.fmt(f)
            }
        }
    }
}

impl Duration {
    pub const _0: Self = Duration::Zero;
    pub const _1: Self = Duration::Simple(Unit::Whole);
    pub const _1_2: Self = Duration::Simple(Unit::Half);
    pub const _1_4: Self = Duration::Simple(Unit::Quarter);
    pub const _1_8: Self = Duration::Simple(Unit::Eighth);
    pub const _1_16: Self = Duration::Simple(Unit::Sixteenth);
    pub const _1_32: Self = Duration::Simple(Unit::ThirtySecondth);
    pub const D_1: Self = Duration::Dotted(Unit::Whole);
    pub const D_1_2: Self = Duration::Dotted(Unit::Half);
    pub const D_1_4: Self = Duration::Dotted(Unit::Quarter);
    pub const D_1_8: Self = Duration::Dotted(Unit::Eighth);
    pub const D_1_16: Self = Duration::Dotted(Unit::Sixteenth);
    pub const D_1_32: Self = Duration::Dotted(Unit::ThirtySecondth);
    pub const T_1: Self = Duration::Triplet(Unit::Whole);
    pub const T_1_2: Self = Duration::Triplet(Unit::Half);
    pub const T_1_4: Self = Duration::Triplet(Unit::Quarter);
    pub const T_1_8: Self = Duration::Triplet(Unit::Eighth);
    pub const T_1_16: Self = Duration::Triplet(Unit::Sixteenth);
    pub const T_1_32: Self = Duration::Triplet(Unit::ThirtySecondth);
    pub const DT_1: Self = Duration::DottedTriplet(Unit::Whole);
    pub const DT_1_2: Self = Duration::DottedTriplet(Unit::Half);
    pub const DT_1_4: Self = Duration::DottedTriplet(Unit::Quarter);
    pub const DT_1_8: Self = Duration::DottedTriplet(Unit::Eighth);
    pub const DT_1_16: Self = Duration::DottedTriplet(Unit::Sixteenth);
    pub const DT_1_32: Self = Duration::DottedTriplet(Unit::ThirtySecondth);

    /// Returns `true` if the duration is [`Simple`].
    pub fn is_simple(&self) -> bool {
        matches!(self, Self::Simple(..))
    }

    pub fn as_simple(&self) -> Option<&Unit> {
        if let Self::Simple(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

impl Duration {
    pub fn to_ident(&self) -> String {
        format!("{}", self)
    }
    pub fn from_ident(ident: &str) -> Self {
        match ident {
            "_0" => Self::_0,
            "_1" => Self::_1,
            "_1_2" => Self::_1_2,
            "_1_4" => Self::_1_4,
            "_1_8" => Self::_1_8,
            "_1_16" => Self::_1_16,
            "_1_32" => Self::_1_32,
            "D_1" => Self::D_1,
            "D_1_2" => Self::D_1_2,
            "D_1_4" => Self::D_1_4,
            "D_1_8" => Self::D_1_8,
            "D_1_16" => Self::D_1_16,
            "D_1_32" => Self::D_1_32,
            "T_1" => Self::T_1,
            "T_1_2" => Self::T_1_2,
            "T_1_4" => Self::T_1_4,
            "T_1_8" => Self::T_1_8,
            "T_1_16" => Self::T_1_16,
            "T_1_32" => Self::T_1_32,
            "DT_1" => Self::DT_1,
            "DT_1_2" => Self::DT_1_2,
            "DT_1_4" => Self::DT_1_4,
            "DT_1_8" => Self::DT_1_8,
            "DT_1_16" => Self::DT_1_16,
            "DT_1_32" => Self::DT_1_32,
            _ => Self::_0,
        }
    }
}

impl Default for Duration {
    fn default() -> Self {
        Self::Simple(Unit::default())
    }
}

impl From<Unit> for Duration {
    fn from(v: Unit) -> Self {
        Self::Simple(v)
    }
}

impl From<f32> for Units {
    fn from(v: f32) -> Self {
        Self(v)
    }
}

impl From<Unit> for Units {
    fn from(v: Unit) -> Self {
        match v {
            Unit::Whole => 1.0,
            Unit::Half => 1.0 / 2.0,
            Unit::Quarter => 1.0 / 4.0,
            Unit::Eighth => 1.0 / 8.0,
            Unit::Sixteenth => 1.0 / 16.0,
            Unit::ThirtySecondth => 1.0 / 32.0,
        }
        .into()
    }
}

impl From<Duration> for Units {
    fn from(v: Duration) -> Self {
        match v {
            Duration::Zero => 0.0,
            Duration::Simple(v) => Units::from(v).0,
            Duration::Dotted(v) => Units::from(v).0 * 1.5,
            Duration::Triplet(v) => Units::from(v).0 * 2.0 / 3.0,
            Duration::DottedTriplet(v) => Units::from(v).0 * 4.0 / 3.0,
        }
        .into()
    }
}
