use rustc_hash::FxHashMap;
use strum_macros::EnumIter;
use LaneType::{BikeLane, BusForward, BusReverse, Empty, Grass, NormalForward, NormalReverse, ParkingForward, ParkingReverse, ShoulderForward, ShoulderReverse, Sidewalk};
use LaneWidth::Half;
use LaneWidth::Full;

#[derive(Copy, Clone, Eq, PartialEq, Debug, EnumIter)]
#[repr(u8)]
pub enum LaneType {
    Empty,
    Grass,
    Sidewalk,
    BikeLane,
    NormalForward,
    NormalReverse,
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
            NormalForward | NormalReverse | BusForward | BusReverse | ParkingForward | ParkingReverse | ShoulderForward | ShoulderReverse => Full,
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