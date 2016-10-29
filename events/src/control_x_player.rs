// use ::ai_x_control::AiToControl;

use ::main_x_control::MainToControl;
use utils::Player;

#[derive(Debug, Clone)]
pub enum ControlToPlayer<W: Clone + Default> {
    Up(W, Player),
    Down(W, Player),
    Left(W, Player),
    Right(W, Player),
    Joy(W, W, Player),
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

impl<'a, W> From<&'a MainToControl<W>> for ControlToPlayer<W>
    where W: Clone + Default
{
    fn from(other: &MainToControl<W>) -> ControlToPlayer<W> {
        match other {
            &MainToControl::Up(ref amount, ref player) => ControlToPlayer::Up(amount.clone(), player.clone()),
            &MainToControl::Down(ref amount, ref player) => ControlToPlayer::Down(amount.clone(), player.clone()),
            &MainToControl::Left(ref amount, ref player) => ControlToPlayer::Left(amount.clone(), player.clone()),
            &MainToControl::Right(ref amount, ref player) => ControlToPlayer::Right(amount.clone(), player.clone()),
            &MainToControl::JoyX(ref x, ref player) => ControlToPlayer::Joy(x.clone(), W::default(), player.clone()),
            &MainToControl::JoyY(ref y, ref player) => ControlToPlayer::Joy(W::default(), y.clone(), player.clone()),
            &MainToControl::A(ref player) => ControlToPlayer::A(player.clone()),
            &MainToControl::B(ref player) => ControlToPlayer::B(player.clone()),
            &MainToControl::X(ref player) => ControlToPlayer::X(player.clone()),
            &MainToControl::Y(ref player) => ControlToPlayer::Y(player.clone()),
            &MainToControl::L1(ref player) => ControlToPlayer::L1(player.clone()),
            &MainToControl::L2(ref player) => ControlToPlayer::L2(player.clone()),
            &MainToControl::R1(ref player) => ControlToPlayer::R1(player.clone()),
            &MainToControl::R2(ref player) => ControlToPlayer::R2(player.clone()),
            &MainToControl::Start(ref player) => ControlToPlayer::Start(player.clone()),
            &MainToControl::Select(ref player) => ControlToPlayer::Select(player.clone()),
        }
    }
}

#[derive(Debug)]
pub enum ControlFromPlayer {

}
