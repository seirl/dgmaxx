use enum_map::Enum;

#[derive(Clone, Copy, Debug, PartialEq, Enum)]
pub enum PlayerColor {
    North,
    South,
}
