#[derive(Clone, Debug)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left
}



#[derive(Clone, Debug)]
pub enum Command {
    None,

    IncreaseLength(usize),
    DecreaseLength(usize),
    SetLength(usize),

    SetDirection(Direction),

    WriteVariable(char),
    ReadVariable(char),

    Print(char),
    PrintLen,

    NumberInput
}
