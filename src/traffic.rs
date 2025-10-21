use rustc_hash::FxHashMap;
use seq_macro::seq;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use LaneType::{BusForward, BusReverse, DirtForward, DirtReverse, Grass, NormalForward, NormalReverse, ParkingForward, ParkingReverse, ShoulderForward, ShoulderReverse, Sidewalk};
use LaneWidth::Full;
use LaneWidth::Half;
use LaneDirection::{Forward, Reverse};

#[derive(Copy, Clone, Eq, PartialEq, Debug, EnumIter)]
#[repr(u8)]
pub enum LaneType {
    Grass,
    Sidewalk,
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
            Grass | Sidewalk => Half,
            NormalForward | NormalReverse | DirtForward | DirtReverse | BusForward | BusReverse | ParkingForward | ParkingReverse | ShoulderForward | ShoulderReverse => Full,
        }
    }

    fn name_internal(self) -> String {
        format!("{:?}", self)
    }

    pub fn direction(self) -> Option<LaneDirection> {
        match self {
            Grass | Sidewalk => None,
            NormalForward | DirtForward | BusForward | ParkingForward | ShoulderForward => Some(Forward),
            NormalReverse | DirtReverse | BusReverse | ParkingReverse | ShoulderReverse => Some(Reverse),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum LaneWidth {
    Half,
    Full,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum LaneDirection {
    Forward,
    Reverse,
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
                  N => LaneStorage::W~N([Grass; N]),
                )*
                _ => panic!("Exceeded max size!"),
            };
            Self {
                lanes
            }
        }

        pub fn get_size(&self) -> u8 {
            match self.lanes {
                #(
                  LaneStorage::W~N(_) => N,
                )*
            }
        }
    }
});