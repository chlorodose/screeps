use std::ops::{Add as _, Neg as _};

use bevy::prelude::Component;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
pub struct Position {
    x: u8,
    y: u8,
    room: Room
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
#[component(immutable)]
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