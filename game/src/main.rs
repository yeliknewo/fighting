#[macro_use]
extern crate log;
extern crate core;
extern crate utils;
extern crate components;
extern crate cgmath;
extern crate find_folder;
extern crate art;
extern crate graphics;
extern crate event_core;
extern crate systems;
extern crate specs;
extern crate events;
extern crate env_logger;

use art::{layers, main, make_square_render};
use cgmath::{Euler, Point3, Rad, Vector3};
use components::{Camera, CompMoving, CompPlayer, RenderData, Transform};
use event_core::two_way_channel::two_way_channel;
use find_folder::Search;
use graphics::load_texture;
use systems::{ControlSystem, MovingSystem, PlayerSystem};
use utils::Player;
use utils::ortho_helper::OrthographicHelper;

pub mod player_system;

fn main() {
    env_logger::init().unwrap_or_else(|err| panic!("Unable to Initate Env Logger: {}", err));

    warn!("Starting Game");

    core::start(None,
                Some(&"glutin sdl2".to_string()),
                (640, 640),
                "PuzzleCity",
                OrthographicHelper::new(1.0, -10.0, 10.0, 0.0, 10.0),
                Box::new(|planner, events, renderer, factory, ortho| {
        planner.mut_world()
            .create_now()
            .with(Camera::new(Point3::new(0.0, 0.0, 2.0), Point3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0), ortho, true))
            .build();

        let packet = make_square_render();

        let assets = Search::ParentsThenKids(3, 3).for_folder("assets").unwrap_or_else(|err| panic!("Did you forget to make an assets folder? Err: {:?}", err));

        let main_render = {
            let texture = load_texture(factory, assets.join(main::NAME));
            renderer.add_render(factory, &packet, texture)
        };

        planner.mut_world()
            .create_now()
            .with(CompMoving::new(Vector3::new(0.0, 0.0, 0.0)))
            .with(CompPlayer::new(Player::One))
            .with(Transform::new(Vector3::new(0.0, 0.0, 0.0), Euler::new(Rad(0.0), Rad(0.0), Rad(0.0)), Vector3::new(1.0, 1.0, 1.0)))
            .with(main_render.clone())
            .with(RenderData::new(layers::PLAYER_2, *main::PLAYER_1_TINT, main::PLAYER_STAND, main::SIZE))
            .build();

        planner.mut_world()
            .create_now()
            .with(CompMoving::new(Vector3::new(0.0, 0.0, 0.0)))
            .with(CompPlayer::new(Player::Two))
            .with(Transform::new(Vector3::new(0.0, 0.0, 0.0), Euler::new(Rad(0.0), Rad(0.0), Rad(0.0)), Vector3::new(1.0, 1.0, 1.0)))
            .with(main_render.clone())
            .with(RenderData::new(layers::PLAYER_2, *main::PLAYER_2_TINT, main::PLAYER_STAND, main::SIZE))
            .build();

        let (control_to_player_front_channel, control_to_player_back_channel) = two_way_channel();

        planner.add_system(ControlSystem::new(events.take_control().unwrap_or_else(|| panic!("Control was none")), control_to_player_front_channel), "control", 30);
        planner.add_system(PlayerSystem::new(control_to_player_back_channel, 5.0, (|me, run_arg| player_system::basic_all_dir(me, run_arg))), "player", 20);
        planner.add_system(MovingSystem::new(), "moving", 15);

        warn!("Finished Setup");
    }),
                Box::new(|_planner, _events| {
                    panic!("Don't run without graphics");
                }));
    warn!("Game Exited");
}
