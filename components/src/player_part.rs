use cgmath::{Matrix4, Vector3};
use cgmath::prelude::Transform;
use specs::{Component, VecStorage};
use std::collections::HashMap;
use utils::{Coord, Delta};

#[derive(Debug, Clone)]
pub enum PlayerPartId {
    Body,
    Arm,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum PlayerState {
    Punching,
    Ducking,
}

#[derive(Debug)]
pub struct PlayerPart {
    id: PlayerPartId,
    states: HashMap<PlayerState, Delta>,
    pos_transform: Matrix4<Coord>,
    scale_transform: Matrix4<Coord>,
    base_pos: Vector3<Coord>,
    base_scale: Vector3<Coord>,
}

impl PlayerPart {
    pub fn new(id: PlayerPartId, pos: Vector3<Coord>, scale: Vector3<Coord>) -> PlayerPart {
        PlayerPart {
            id: id,
            states: {
                let mut map = HashMap::new();
                map.insert(PlayerState::Punching, 0.0);
                map.insert(PlayerState::Ducking, 0.0);
                map
            },
            pos_transform: Matrix4::one(),
            scale_transform: Matrix4::one(),
            base_pos: pos,
            base_scale: scale,
        }
    }

    pub fn get_mut_state(&mut self, state: &PlayerState) -> Option<&mut Delta> {
        self.states.get_mut(state)
    }

    pub fn get_mut_pos_transform(&mut self) -> &mut Matrix4<Coord> {
        &mut self.pos_transform
    }

    pub fn get_mut_scale_transform(&mut self) -> &mut Matrix4<Coord> {
        &mut self.scale_transform
    }

    pub fn get_state(&self, state: &PlayerState) -> Option<&Delta> {
        self.states.get(state)
    }

    pub fn get_out_pos(&self) -> Vector3<Coord> {
        let v4 = self.pos_transform * self.base_pos.extend(1.0);
        Vector3::new(v4.x, v4.y, v4.z)
    }

    pub fn get_out_scale(&self) -> Vector3<Coord> {
        let v4 = self.scale_transform * self.base_scale.extend(1.0);
        Vector3::new(v4.x, v4.y, v4.z)
    }

    pub fn get_id(&self) -> &PlayerPartId {
        &self.id
    }
}

impl Component for PlayerPart {
    type Storage = VecStorage<PlayerPart>;
}
