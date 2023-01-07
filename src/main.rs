mod algo;
mod components;
mod get_position_offsets;

mod prelude {
    pub use std::cmp::Ordering;
    pub use std::collections::HashMap;
    pub use std::collections::VecDeque;
    pub use std::fmt::Display;

    pub use cfg_block::cfg_block;
    pub use core::fmt;
    pub use dioxus::events::MouseEvent;
    pub use dioxus::prelude::*;

    pub use log::{info, Level};
    pub use petgraph::dot::Dot;
    pub use petgraph::graph::{Graph, NodeIndex};
    pub use serde::Deserialize;
    pub use thiserror::Error;
    pub use uuid::Uuid;

    pub use crate::algo::calendar_block::*;
    pub use crate::algo::calendar_trie::*;
    pub use crate::components::{calendar, calendar_block, empty_element, root, url_input};
    pub use crate::get_position_offsets::*;

    pub const MAX_COL_WIDTH: f64 = 500.0;
    pub const BLOCK_STACK_PADDING: f64 = 10.0;
}

use prelude::*;

cfg_block! {
    if #[cfg(feature = "console_log")] {
        fn init_log() {
            console_log::init_with_level(Level::Trace).expect("error initializing log");
        }
    } else {
        fn init_log() {}
    }
}

fn main() {
    init_log();
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    let calendar_blocks = use_state(&cx, || {
        let mut calendar_blocks = vec![
            CalendarBlock {
                id: Uuid::new_v4(),
                start_minute: 750,
                end_minute: 1050,
                block_type: CalendarBlockType::Available,
            },
            CalendarBlock {
                id: Uuid::new_v4(),
                start_minute: 780,
                end_minute: 915,
                block_type: CalendarBlockType::Busy,
            },
            CalendarBlock {
                id: Uuid::new_v4(),
                start_minute: 780,
                end_minute: 900,
                block_type: CalendarBlockType::Busy,
            },
            CalendarBlock {
                id: Uuid::new_v4(),
                start_minute: 780,
                end_minute: 825,
                block_type: CalendarBlockType::Busy,
            },
        ];

        calendar_blocks.sort_by(|a, b| {
            if a.start_minute < b.start_minute
                || a.start_minute == b.start_minute && a.end_minute >= b.end_minute
            {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        return calendar_blocks;
    });

    // TODO:
    // pass the &UseState<Vec<CalendarBlock>> to the calendar,
    // populate the trie and Vec<FlattenedCalendarBlock> from the &UseState<Vec<CalendarBlock>>,
    // handle_update_calendar_blocks should update the Vec<CalendarBlock> itself

    cx.render(rsx! {
        calendar::Calendar {
            calendar_blocks: calendar_blocks,
        }
    })
}
