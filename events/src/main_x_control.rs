use utils::Player;

#[derive(Debug)]
pub enum MainToControl<W> {
    Up(W, Player),
    Down(W, Player),
    Left(W, Player),
    Right(W, Player),
    JoyX(W, Player),
    JoyY(W, Player),
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

#[derive(Debug)]
pub enum MainFromControl {
    Save,
}
