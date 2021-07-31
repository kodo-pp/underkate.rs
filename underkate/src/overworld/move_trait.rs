use crate::geometry::OverworldPoint;

pub type Position = OverworldPoint<f32>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Forward,
    Backward,
}

pub trait Move {
    fn position(&self) -> Position;
    fn set_position(&mut self, position: Position);

    fn direction(&self) -> Direction;
    fn set_direction(&mut self, direction: Direction);
}

pub struct MoveContext {
    pub position: Position,
    pub direction: Direction,
}

pub trait HasMoveContext: AsRef<MoveContext> + AsMut<MoveContext> {}

impl<T: HasMoveContext> Move for T {
    fn position(&self) -> Position {
        self.as_ref().position
    }

    fn set_position(&mut self, position: Position) {
        self.as_mut().position = position;
    }

    fn direction(&self) -> Direction {
        self.as_ref().direction
    }

    fn set_direction(&mut self, direction: Direction) {
        self.as_mut().direction = direction;
    }
}
