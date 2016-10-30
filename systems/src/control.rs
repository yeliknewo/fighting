use event_core::two_way_channel::{BackChannel, FrontChannel};
use events::control_x_player::{ControlFromPlayer, ControlToPlayer};
use events::main_x_control::{MainFromControl, MainToControl};
use specs::{RunArg, System};
use std::collections::HashMap;
use utils::{Delta, Player};

#[derive(Debug)]
pub struct ControlSystem {
    main_channel: BackChannel<MainToControl<f64>, MainFromControl>,
    player_channel: FrontChannel<ControlToPlayer<f64>, ControlFromPlayer>,
    repeat_map: HashMap<RepeatEvent, ControlToPlayer<f64>>,
    time: f64,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
enum RepeatEvent {
    Left(Player),
    Right(Player),
    Up(Player),
    Down(Player),
    Joy(Player),
    A(Player),
    B(Player),
    X(Player),
    Y(Player),
    L1(Player),
    L2(Player),
    R1(Player),
    R2(Player),
    Start(Player),
    Select(Player),
}

impl ControlSystem {
    pub fn new(main_channel: BackChannel<MainToControl<f64>, MainFromControl>, player_channel: FrontChannel<ControlToPlayer<f64>, ControlFromPlayer>) -> ControlSystem {
        ControlSystem {
            main_channel: main_channel,
            player_channel: player_channel,
            repeat_map: HashMap::new(),
            time: 0.0,
        }
    }

    fn process_main_event(&mut self, event: &MainToControl<f64>) {
        debug!("Got event from main");
        match event {
            &MainToControl::JoyX(x, player) => self.handle_joy(Some(x), None, player),
            &MainToControl::JoyY(y, player) => self.handle_joy(None, Some(y), player),
            event => self.send_repeat(ControlToPlayer::from(event)),
        }
    }

    fn handle_joy(&mut self, x_opt: Option<f64>, y_opt: Option<f64>, player: Player) {
        // TODO Implement This Function
        let event = if x_opt.is_none() || y_opt.is_none() {
            if let Some(event) = self.repeat_map.get(&RepeatEvent::Joy(player)) {
                match event.clone() {
                    ControlToPlayer::Joy(x, y, player) => ControlToPlayer::Joy(x_opt.unwrap_or(x), y_opt.unwrap_or(y), player),
                    _ => ControlToPlayer::Joy(x_opt.unwrap_or_default(), y_opt.unwrap_or_default(), player),
                }
            } else {
                ControlToPlayer::Joy(x_opt.unwrap_or_default(), y_opt.unwrap_or_default(), player)
            }
        } else {
            ControlToPlayer::Joy(x_opt.unwrap_or_default(), y_opt.unwrap_or_default(), player)
        };
        self.send_repeat(event);
    }

    fn send_repeat(&mut self, event: ControlToPlayer<f64>) {
        match &event {
            &ControlToPlayer::Up(_, player) => self.repeat_map.insert(RepeatEvent::Up(player), event),
            &ControlToPlayer::Down(_, player) => self.repeat_map.insert(RepeatEvent::Down(player), event),
            &ControlToPlayer::Right(_, player) => self.repeat_map.insert(RepeatEvent::Right(player), event),
            &ControlToPlayer::Left(_, player) => self.repeat_map.insert(RepeatEvent::Left(player), event),
            &ControlToPlayer::Joy(_, _, player) => self.repeat_map.insert(RepeatEvent::Joy(player), event),
            &ControlToPlayer::A(_, player) => self.repeat_map.insert(RepeatEvent::A(player), event),
            &ControlToPlayer::B(_, player) => self.repeat_map.insert(RepeatEvent::B(player), event),
            &ControlToPlayer::X(_, player) => self.repeat_map.insert(RepeatEvent::X(player), event),
            &ControlToPlayer::Y(_, player) => self.repeat_map.insert(RepeatEvent::Y(player), event),
            &ControlToPlayer::L1(_, player) => self.repeat_map.insert(RepeatEvent::L1(player), event),
            &ControlToPlayer::L2(_, player) => self.repeat_map.insert(RepeatEvent::L2(player), event),
            &ControlToPlayer::R1(_, player) => self.repeat_map.insert(RepeatEvent::R1(player), event),
            &ControlToPlayer::R2(_, player) => self.repeat_map.insert(RepeatEvent::R2(player), event),
            &ControlToPlayer::Start(_, player) => self.repeat_map.insert(RepeatEvent::Start(player), event),
            &ControlToPlayer::Select(_, player) => self.repeat_map.insert(RepeatEvent::Select(player), event),
        };
    }

    // fn send_once(&mut self, event: ControlToPlayer<f64>) {
    //     self.player_channel.send(event);
    // }

    fn trigger_repeats(&mut self) {
        for value in self.repeat_map.clone().values() {
            debug!("Sending event to player");
            self.player_channel.send(value.clone())
        }
    }
}

impl System<Delta> for ControlSystem {
    fn run(&mut self, arg: RunArg, delta_time: Delta) {
        trace!("Started Control Run");

        self.time += delta_time;

        if self.time >= 300.0 {
            self.time = 0.0;
            self.main_channel.send(MainFromControl::Save);
        }

        let mut needs_fetch = vec![true];

        while needs_fetch.iter().any(|item| *item) {
            if let Some(event) = self.main_channel.try_recv() {
                self.process_main_event(&event);
            } else {
                needs_fetch[0] = false;
            }
        }

        trace!("Control Got Events");

        self.trigger_repeats();

        trace!("Control Handled Repeats");

        arg.fetch(|_| ());

        trace!("Control Exiting");
    }
}
