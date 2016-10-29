use event_core::two_way_channel::BackChannel;
use events::control_x_player::{ControlFromPlayer, ControlToPlayer};
use specs::{RunArg, System};
use utils::{Coord, Delta};

#[allow(dead_code)]
pub struct PlayerSystem<F1>
    where F1: Send + Fn(&mut PlayerSystem<F1>, RunArg)
{
    control_channel: BackChannel<ControlToPlayer<f64>, ControlFromPlayer>,
    event_handler: Option<F1>,
    speed: Coord,
}

impl<F1> PlayerSystem<F1>
    where F1: Send + Fn(&mut PlayerSystem<F1>, RunArg)
{
    pub fn new(control_channel: BackChannel<ControlToPlayer<f64>, ControlFromPlayer>, speed: Coord, event_handler: F1) -> PlayerSystem<F1> {
        PlayerSystem {
            control_channel: control_channel,
            speed: speed,
            event_handler: Some(event_handler),
        }
    }

    pub fn get_speed(&self) -> Coord {
        self.speed
    }

    pub fn get_mut_speed(&mut self) -> &mut Coord {
        &mut self.speed
    }

    pub fn get_mut_control_channel(&mut self) -> &mut BackChannel<ControlToPlayer<f64>, ControlFromPlayer> {
        &mut self.control_channel
    }
}

impl<F1> System<Delta> for PlayerSystem<F1>
    where F1: Send + Fn(&mut PlayerSystem<F1>, RunArg)
{
    fn run(&mut self, arg: RunArg, _: Delta) {

        let handler = self.event_handler.take().unwrap_or_else(|| panic!("Event Handler was none"));
        handler(self, arg);
        self.event_handler = Some(handler);
    }
}
