use nom_locate::LocatedSpan;
use serde::Serialize;
#[derive(Debug, Clone, Serialize)]
pub struct Position {
    pub start_line: u32,
    pub start_column: usize,
    pub end_line: u32,
    pub end_column: usize,
    pub start_offset: usize,
    pub end_offset: usize,
}

pub fn make_position(start: LocatedSpan<&str>, end: LocatedSpan<&str>) -> Position {
    Position {
        start_line: start.location_line(),
        start_column: start.get_column(),
        start_offset: start.location_offset(),
        end_line: end.location_line(),
        end_column: end.get_column(),
        end_offset: end.location_offset(),
    }
}
