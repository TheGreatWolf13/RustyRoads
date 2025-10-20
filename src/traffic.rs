use rustc_hash::FxHashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use LaneType::{BikeLane, BusForward, BusReverse, DirtForward, DirtReverse, Empty, Grass, NormalForward, NormalReverse, ParkingForward, ParkingReverse, ShoulderForward, ShoulderReverse, Sidewalk};
use LaneWidth::Full;
use LaneWidth::Half;

#[derive(Copy, Clone, Eq, PartialEq, Debug, EnumIter)]
#[repr(u8)]
pub enum LaneType {
    Empty,
    Grass,
    Sidewalk,
    BikeLane,
    NormalForward,
    NormalReverse,
    DirtForward,
    DirtReverse,
    BusForward,
    BusReverse,
    ParkingForward,
    ParkingReverse,
    ShoulderForward,
    ShoulderReverse,
}

impl LaneType {
    pub fn width(self) -> LaneWidth {
        match self {
            Empty | Grass | Sidewalk | BikeLane => Half,
            NormalForward | NormalReverse | DirtForward | DirtReverse | BusForward | BusReverse | ParkingForward | ParkingReverse | ShoulderForward | ShoulderReverse => Full,
        }
    }

    fn name_internal(self) -> String {
        format!("{:?}", self)
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum LaneWidth {
    Half,
    Full,
}

pub struct LaneTypeManager {
    name_to_variant: FxHashMap<String, LaneType>,
}

impl LaneTypeManager {
    pub fn new() -> Self {
        let mut name_to_variant = FxHashMap::default();
        for lane in LaneType::iter() {
            name_to_variant.insert(lane.name_internal(), lane);
        }
        Self {
            name_to_variant,
        }
    }
}

pub struct LaneDefinition {
    lanes: LaneStorage,
}

macro_rules! lane_storage {
    ($name:ident $prefix:ident [$($N:literal)+]) => {
        paste::item!{
            enum $name {
                $([<$prefix $N>]([LaneType; $N])),+,
            }
        }

        paste::item! {
            impl LaneDefinition {
                pub fn size(&self) -> u8 {
                    match self.lanes {
                        $(LaneStorage::[<$prefix $N>](_) => $N),+,
                    }
                }
            }
        }
    }
}

lane_storage! {
    LaneStorage W [
         1  2  3  4  5  6  7  8  9 10
        11 12 13 14 15 16 17 18 19 20
        21 22 23 24 25 26 27 28 29 30
        31 32 33 34 35 36 37 38 39 40
    ]
}