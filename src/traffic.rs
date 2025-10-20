use rustc_hash::FxHashMap;
use strum_macros::EnumIter;
use LaneType::{BikeLane, BusForward, BusReverse, DirtForward, DirtReverse, Empty, Grass, NormalForward, NormalReverse, ParkingForward, ParkingReverse, ShoulderForward, ShoulderReverse, Sidewalk};
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
    lanes: LaneStorage
}

enum LaneStorage {
    _1([LaneType; 1]),
    _2([LaneType; 2]),
    _3([LaneType; 3]),
    _4([LaneType; 4]),
    _5([LaneType; 5]),
    _6([LaneType; 6]),
    _7([LaneType; 7]),
    _8([LaneType; 8]),
    _9([LaneType; 9]),
    _10([LaneType; 10]),
    _11([LaneType; 11]),
    _12([LaneType; 12]),
    _13([LaneType; 13]),
    _14([LaneType; 14]),
    _15([LaneType; 15]),
    _16([LaneType; 16]),
    _17([LaneType; 17]),
    _18([LaneType; 18]),
    _19([LaneType; 19]),
    _20([LaneType; 20]),
    _21([LaneType; 21]),
    _22([LaneType; 22]),
    _23([LaneType; 23]),
    _24([LaneType; 24]),
    _25([LaneType; 25]),
    _26([LaneType; 26]),
    _27([LaneType; 27]),
    _28([LaneType; 28]),
    _29([LaneType; 29]),
    _30([LaneType; 30]),
    _31([LaneType; 31]),
    _32([LaneType; 32]),
    _33([LaneType; 33]),
    _34([LaneType; 34]),
    _35([LaneType; 35]),
    _36([LaneType; 36]),
    _37([LaneType; 37]),
    _38([LaneType; 38]),
    _39([LaneType; 39]),
    _40([LaneType; 40]),
}

impl LaneDefinition {
    pub fn size(&self) -> u8 {
        match self.lanes {
            LaneStorage::_1(_) => 1,
            LaneStorage::_2(_) => 2,
            LaneStorage::_3(_) => 3,
            LaneStorage::_4(_) => 4,
            LaneStorage::_5(_) => 5,
            LaneStorage::_6(_) => 6,
            LaneStorage::_7(_) => 7,
            LaneStorage::_8(_) => 8,
            LaneStorage::_9(_) => 9,
            LaneStorage::_10(_) => 10,
            LaneStorage::_11(_) => 11,
            LaneStorage::_12(_) => 12,
            LaneStorage::_13(_) => 13,
            LaneStorage::_14(_) => 14,
            LaneStorage::_15(_) => 15,
            LaneStorage::_16(_) => 16,
            LaneStorage::_17(_) => 17,
            LaneStorage::_18(_) => 18,
            LaneStorage::_19(_) => 19,
            LaneStorage::_20(_) => 20,
            LaneStorage::_21(_) => 21,
            LaneStorage::_22(_) => 22,
            LaneStorage::_23(_) => 23,
            LaneStorage::_24(_) => 24,
            LaneStorage::_25(_) => 25,
            LaneStorage::_26(_) => 26,
            LaneStorage::_27(_) => 27,
            LaneStorage::_28(_) => 28,
            LaneStorage::_29(_) => 29,
            LaneStorage::_30(_) => 30,
            LaneStorage::_31(_) => 31,
            LaneStorage::_32(_) => 32,
            LaneStorage::_33(_) => 33,
            LaneStorage::_34(_) => 34,
            LaneStorage::_35(_) => 35,
            LaneStorage::_36(_) => 36,
            LaneStorage::_37(_) => 37,
            LaneStorage::_38(_) => 38,
            LaneStorage::_39(_) => 39,
            LaneStorage::_40(_) => 40,
        }
    }
}