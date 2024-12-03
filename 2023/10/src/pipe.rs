#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pipe {
    Start,
    Ground,

    Pipe,
    Dash,

    NtoE,
    WtoN,
    WtoS,
    EtoS,
}

pub const PIPES_CONNECT_UP: [Pipe; 4] = [Pipe::Start, Pipe::Pipe, Pipe::NtoE, Pipe::WtoN];
pub const PIPES_CONNECT_DOWN: [Pipe; 4] = [Pipe::Start, Pipe::Pipe, Pipe::EtoS, Pipe::WtoS];
pub const PIPES_CONNECT_LEFT: [Pipe; 4] = [Pipe::Start, Pipe::Dash, Pipe::WtoN, Pipe::WtoS];
pub const PIPES_CONNECT_RIGHT: [Pipe; 4] = [Pipe::Start, Pipe::Dash, Pipe::EtoS, Pipe::NtoE];

impl Pipe {
    pub fn from_char(c: char) -> Self {
        match c {
            'S' => Self::Start,
            '.' => Self::Ground,

            '|' => Self::Pipe,
            '-' => Self::Dash,

            'L' => Self::NtoE,
            'J' => Self::WtoN,
            '7' => Self::WtoS,
            'F' => Self::EtoS,

            _ => panic!("Not matching char: {}", c)
        }
    }
}

impl Default for Pipe {
    fn default() -> Self {
        Self::Ground
    }
}
