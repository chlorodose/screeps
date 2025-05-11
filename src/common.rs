use std::ops::{Add as _, Neg as _};

use bevy::prelude::Component;

use arrayvec::ArrayVec;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// The room's coordinates in the game world
pub struct Room {
    x: i16,
    y: i16
}
impl ToString for Room {
    fn to_string(&self) -> String {
        let (x_sign, x) = if self.x.is_negative() {
            ('W', self.x.add(1).neg())
        } else {
            ('E', self.x)
        };
        let (y_sign, y) = if self.y.is_negative() {
            ('S',  self.y.add(1).neg())
        } else {
            ('N', self.y)
        };
        format!("{x_sign}{x}{y_sign}{y}")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Component)]
/// The object's location in the game world
pub struct Position {
    x: u8,
    y: u8,
    room: Room
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
#[component(immutable)]
/// The unique id of the game object
pub struct Id([u8; 12]);
impl Default for Id {
    fn default() -> Self {
        Self(rand::random())
    }
}
impl ToString for Id {
    fn to_string(&self) -> String {
        self.0.iter().flat_map(|x| [x / 16, x % 16]).map(|x| if x < 10 {
            x + b'0'
        } else {
            x - 10 + b'a'
        }).map(|x| x as char).collect()
    }
}

#[derive(Debug, Clone, Copy, Hash, Component)]
/// The game object will decay once the gien game tick reached
pub struct DecayTimer(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
/// The owner's user id for the game object
pub struct OwnerId(u32);

/// An obejct stores all resources in the game object
pub trait Store {
    /// Get total capacity of the total store or given resource type
    fn get_capacity(&self, resource: Option<ResourceType>) -> u32;
    /// Get free capacity of the total store or given resource type
    fn get_free_capacity(&self, resource: Option<ResourceType>) -> u32;
    /// Get used capacity of the total store or given resource type
    fn get_used_capacity(&self, resource: Option<ResourceType>) -> u32;
    /// Test is given resource type is vaild for given store
    fn is_vaild(&self, resource: ResourceType) -> bool;
    /// Try to store resource into the store, returns if the action successed
    fn try_store(&mut self, resource: ResourceType, amount: u32) -> bool;
    /// Try to withdraw resource out of the store, returns if the action successed
    fn try_withdraw(&mut self, resource: ResourceType, amount: u32) -> bool;
}

#[derive(Debug, Clone, Copy, Default, Component)]
/// The HP to the game object
pub struct Hits {
    max_hits: u32,
    hits: u32
}

#[derive(Debug, Clone, Component)]
/// The creep's body parts vector
pub struct BodyParts(ArrayVec<(BodyPartType, ResourceType), 50>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
#[non_exhaustive]
/// Creep body part types
pub enum BodyPartType {
    Move,
    Work,
    Carry,
    Attack,
    RangedAttack,
    Heal,
    Claim,
    Tough
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u16)]
#[non_exhaustive]
/// Resource types
pub enum ResourceType {
    #[default]
    Empty = 0,
    Energy,
    Power,
    Hydrogen,
    Oxygen,
    Utrium,
    Lemergium,
    Keanium,
    Zynthium,
    Catalyst,
    Ghodium,
    Silicon,
    Metal,
    Biomass,
    Mist,
    Hydroxide,
    ZynthiumKeanite,
    UtriumLemergite,
    UtriumHydride,
    UtriumOxide,
    KeaniumHydride,
    KeaniumOxide,
    LemergiumHydride,
    LemergiumOxide,
    ZynthiumHydride,
    ZynthiumOxide,
    GhodiumHydride,
    GhodiumOxide,
    UtriumAcid,
    UtriumAlkalide,
    KeaniumAcid,
    KeaniumAlkalide,
    LemergiumAcid,
    LemergiumAlkalide,
    ZynthiumAcid,
    ZynthiumAlkalide,
    GhodiumAcid,
    GhodiumAlkalide,
    CatalyzedUtriumAcid,
    CatalyzedUtriumAlkalide,
    CatalyzedKeaniumAcid,
    CatalyzedKeaniumAlkalide,
    CatalyzedLemergiumAcid,
    CatalyzedLemergiumAlkalide,
    CatalyzedZynthiumAcid,
    CatalyzedZynthiumAlkalide,
    CatalyzedGhodiumAcid,
    CatalyzedGhodiumAlkalide,
    Ops,
    UtriumBar,
    LemergiumBar,
    ZynthiumBar,
    KeaniumBar,
    GhodiumMelt,
    Oxidant,
    Reductant,
    Purifier,
    Battery,
    Composite,
    Crystal,
    Liquid,
    Wire,
    Switch,
    Transistor,
    Microchip,
    Circuit,
    Device,
    Cell,
    Phlegm,
    Tissue,
    Muscle,
    Organoid,
    Organism,
    Alloy,
    Tube,
    Fixtures,
    Frame,
    Hydraulics,
    Machine,
    Condensate,
    Concentrate,
    Extract,
    Spirit,
    Emanation,
    Essence,
    Score,
    SymbolAleph,
    SymbolBeth,
    SymbolGimmel,
    SymbolDaleth,
    SymbolHe,
    SymbolWaw,
    SymbolZayin,
    SymbolHeth,
    SymbolTeth,
    SymbolYodh,
    SymbolKaph,
    SymbolLamedh,
    SymbolMem,
    SymbolNun,
    SymbolSamekh,
    SymbolAyin,
    SymbolPe,
    SymbolTsade,
    SymbolQoph,
    SymbolRes,
    SymbolSin,
    SymbolTaw,
    Thorium,
}