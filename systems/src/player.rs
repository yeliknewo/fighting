use cgmath::{Matrix4, Vector3};
use cgmath::prelude::One;
use components::{CompPlayer, PlayerPart, Transform};
use components::player_part::PlayerPartId;
use event_core::two_way_channel::BackChannel;
use events::control_x_player::{ControlFromPlayer, ControlToPlayer};
use specs::{Join, RunArg, System};
use std::collections::HashMap;
use utils::{Coord, Delta};

#[derive(Debug)]
struct PlayerMove {
    punching: bool,
    ducking: bool,
}

impl PlayerMove {
    fn new() -> PlayerMove {
        PlayerMove {
            punching: false,
            ducking: false,
        }
    }
}

#[allow(dead_code)]
pub struct PlayerSystem {
    control_channel: BackChannel<ControlToPlayer<f64>, ControlFromPlayer>,
    speed: Coord,
}

impl PlayerSystem {
    pub fn new(control_channel: BackChannel<ControlToPlayer<f64>, ControlFromPlayer>, speed: Coord) -> PlayerSystem {
        PlayerSystem {
            control_channel: control_channel,
            speed: speed,
        }
    }

    pub fn get_speed(&self) -> Coord {
        self.speed
    }

    pub fn get_mut_speed(&mut self) -> &mut Coord {
        &mut self.speed
    }
}

impl System<Delta> for PlayerSystem {
    fn run(&mut self, arg: RunArg, _delta_time: Delta) {
        let (players, mut player_parts, mut transforms) = arg.fetch(|w| (w.read::<CompPlayer>(), w.write::<PlayerPart>(), w.write::<Transform>()));

        let mut map = HashMap::new();

        for player in (&players).iter() {
            map.insert(player.get_player(), PlayerMove::new());
        }

        while let Some(event) = self.control_channel.try_recv() {
            match event {
                // ControlToPlayer::Right(amount, player_evt) => {
                //     if let Some(player_move) = map.get_mut(&player_evt) {
                //
                //     }
                // }
                // ControlToPlayer::Left(amount, player_evt) => {
                //     if let Some(player_move) = map.get_mut(&player_evt) {
                //
                //     }
                // }
                // ControlToPlayer::Up(amount, player_evt) => {
                //     if let Some(player_move) = map.get_mut(&player_evt) {
                //
                //     }
                // }
                // ControlToPlayer::Down(amount, player_evt) => {
                //     if let Some(player_move) = map.get_mut(&player_evt) {
                //
                //     }
                // }
                // ControlToPlayer::Joy(x, y, player_evt) => {
                //     if let Some(player_move) = map.get_mut(&player_evt) {
                //
                //     }
                // }
                ControlToPlayer::A(down, player_evt) => {
                    debug!("Got A");
                    if let Some(player_move) = map.get_mut(&player_evt) {
                        player_move.punching = down;
                    }
                }
                // ControlToPlayer::B(down, player_evt) => {
                //     if let Some(player_move) = map.get_mut(&player_evt) {
                //
                //     }
                // }
                ControlToPlayer::X(down, player_evt) => {
                    debug!("Got X");
                    if let Some(player_move) = map.get_mut(&player_evt) {
                        player_move.ducking = down;
                    }
                }
                // ControlToPlayer::Y(down, player_evt) => {
                //     if let Some(player_move) = map.get_mut(&player_evt) {
                //
                //     }
                // }
                // ControlToPlayer::L1(down, player_evt) => {
                //     if let Some(player_move) = map.get_mut(&player_evt) {
                //
                //     }
                // }
                // ControlToPlayer::L2(down, player_evt) => {
                //     if let Some(player_move) = map.get_mut(&player_evt) {
                //
                //     }
                // }
                // ControlToPlayer::R1(down, player_evt) => {
                //     if let Some(player_move) = map.get_mut(&player_evt) {
                //
                //     }
                // }
                // ControlToPlayer::R2(down, player_evt) => {
                //     if let Some(player_move) = map.get_mut(&player_evt) {
                //
                //     }
                // }
                // ControlToPlayer::Start(down, player_evt) => {
                //     if let Some(player_move) = map.get_mut(&player_evt) {
                //
                //     }
                // }
                // ControlToPlayer::Select(down, player_evt) => {
                //     if let Some(player_move) = map.get_mut(&player_evt) {
                //
                //     }
                // }
                _ => (),
            }
        }

        for (player, mut player_part, mut transform) in (&players, &mut player_parts, &mut transforms).iter() {
            if let Some(player_move) = map.get(&player.get_player()) {

                let mut pos = Matrix4::one();
                let mut scale = Matrix4::one();

                match player_part.get_id().clone() {
                    PlayerPartId::Arm => {
                        if player_move.punching {
                            scale = scale * Matrix4::from_translation(Vector3::new(1.0, 0.0, 0.0));
                            pos = pos * Matrix4::from_nonuniform_scale(0.0, 1.0, 1.0);
                        }
                        if player_move.ducking {
                            scale = scale * Matrix4::from_nonuniform_scale(1.0, 0.4, 1.0);
                        }
                    }
                    PlayerPartId::Body => {
                        if player_move.ducking {
                            scale = scale * Matrix4::from_nonuniform_scale(1.0, 0.4, 1.0);
                        }
                    }
                }

                *player_part.get_mut_pos_transform() = pos;
                *player_part.get_mut_scale_transform() = scale;
                *transform.get_mut_pos() = player_part.get_out_pos();
                *transform.get_mut_scale() = player_part.get_out_scale();
            }
        }
    }
}
