use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CalendarBlockType {
    Busy,
    Available,
    Wrapper,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CalendarBlock {
    pub id: Uuid,
    pub start_minute: u32,
    pub end_minute: u32, // exclusive
    pub block_type: CalendarBlockType,
}

#[derive(Debug)]
pub enum CalendarBlockOverlap {
    Swallows,
    GetsSwallowed,
}

impl CalendarBlock {
    pub fn does_overlap(&self, block: CalendarBlock) -> Option<CalendarBlockOverlap> {
        if self.start_minute >= block.end_minute || self.end_minute < block.start_minute {
            return None;
        }

        if self.start_minute > block.start_minute
            || (self.start_minute == block.start_minute && self.end_minute <= block.end_minute)
        {
            return Some(CalendarBlockOverlap::GetsSwallowed);
        }

        return Some(CalendarBlockOverlap::Swallows);
    }
}
