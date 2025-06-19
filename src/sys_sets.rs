// system sets are temporaly implemented for education.
//

use bevy::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PlayerSet {
    Movement,
    Confinement,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct EnemySet;