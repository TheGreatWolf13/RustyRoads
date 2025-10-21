use rustc_hash::FxHashMap;
use seq_macro::seq;
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

seq!(N in 1..=40 {
    enum LaneStorage {
        #(
          W~N([LaneType; N]),
        )*
    }

    impl LaneDefinition {
        pub fn new(size: u8) -> Self {
            let lanes = match size {
                0 => panic!("Size cannot be zero!"),
                #(
                  N => LaneStorage::W~N([Empty; N]),
                )*
                _ => panic!("Exceeded max size!"),
            };
            Self {
                lanes
            }
        }

        pub fn size(&self) -> u8 {
            match self.lanes {
                #(
                  LaneStorage::W~N(_) => N,
                )*
            }
        }
    }
});