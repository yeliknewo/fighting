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
    A(bool, Player),
    B(bool, Player),
    X(bool, Player),
    Y(bool, Player),
    L1(bool, Player),
    L2(bool, Player),
    R1(bool, Player),
    R2(bool, Player),
    Start(bool, Player),
    Select(bool, Player),
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
            &MainToControl::A(down, ref player) => ControlToPlayer::A(down, player.clone()),
            &MainToControl::B(down, ref player) => ControlToPlayer::B(down, player.clone()),
            &MainToControl::X(down, ref player) => ControlToPlayer::X(down, player.clone()),
            &MainToControl::Y(down, ref player) => ControlToPlayer::Y(down, player.clone()),
            &MainToControl::L1(down, ref player) => ControlToPlayer::L1(down, player.clone()),
            &MainToControl::L2(down, ref player) => ControlToPlayer::L2(down, player.clone()),
            &MainToControl::R1(down, ref player) => ControlToPlayer::R1(down, player.clone()),
            &MainToControl::R2(down, ref player) => ControlToPlayer::R2(down, player.clone()),
            &MainToControl::Start(down, ref player) => ControlToPlayer::Start(down, player.clone()),
            &MainToControl::Select(down, ref player) => ControlToPlayer::Select(down, player.clone()),
        }
    }
}

#[derive(Debug)]
pub enum ControlFromPlayer {

}
