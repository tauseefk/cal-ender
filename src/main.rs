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
    pub use petgraph::visit::EdgeRef;
    pub use serde::Deserialize;
    pub use thiserror::Error;
    pub use uuid::Uuid;

    pub use crate::algo::calendar_block::*;
    pub use crate::algo::calendar_tree::*;
    pub use crate::components::{calendar, calendar_block, empty_element};
    pub use crate::get_position_offsets::*;

    pub const MAX_COL_WIDTH: f64 = 500.0;
    pub const BLOCK_STACK_PADDING: f64 = 10.0;
    pub const BLOCK_TOP_OFFSET: u32 = 200;
}

use prelude::*;

cfg_block! {
    if #[cfg(feature = "console_log")] {
        fn init_log() {
            console_log::init_with_level(Level::Info).expect("error initializing log");
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
                start_minute: 530 - BLOCK_TOP_OFFSET,
                end_minute: 830 - BLOCK_TOP_OFFSET,
                block_type: CalendarBlockType::Available,
                subtree_depth: 0,
                label: String::from("Available"),
            },
            CalendarBlock {
                id: Uuid::new_v4(),
                start_minute: 550 - BLOCK_TOP_OFFSET,
                end_minute: 590 - BLOCK_TOP_OFFSET,
                block_type: CalendarBlockType::Busy,
                subtree_depth: 0,
                label: String::from("Shower"),
            },
            CalendarBlock {
                id: Uuid::new_v4(),
                start_minute: 550 - BLOCK_TOP_OFFSET,
                end_minute: 580 - BLOCK_TOP_OFFSET,
                block_type: CalendarBlockType::Busy,
                subtree_depth: 0,
                label: String::from("Shower Thoughts"),
            },
            CalendarBlock {
                id: Uuid::new_v4(),
                start_minute: 605 - BLOCK_TOP_OFFSET,
                end_minute: 665 - BLOCK_TOP_OFFSET,
                block_type: CalendarBlockType::Busy,
                subtree_depth: 0,
                label: String::from("Coffee"),
            },
            CalendarBlock {
                id: Uuid::new_v4(),
                start_minute: 605 - BLOCK_TOP_OFFSET,
                end_minute: 630 - BLOCK_TOP_OFFSET,
                block_type: CalendarBlockType::Busy,
                subtree_depth: 0,
                label: String::from("Brew"),
            },
            CalendarBlock {
                id: Uuid::new_v4(),
                start_minute: 635 - BLOCK_TOP_OFFSET,
                end_minute: 710 - BLOCK_TOP_OFFSET,
                block_type: CalendarBlockType::Busy,
                subtree_depth: 0,
                label: String::from("Contemplation"),
            },
            CalendarBlock {
                id: Uuid::new_v4(),
                start_minute: 650 - BLOCK_TOP_OFFSET,
                end_minute: 830 - BLOCK_TOP_OFFSET,
                block_type: CalendarBlockType::Busy,
                subtree_depth: 0,
                label: String::from("Code"),
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

        calendar_blocks
    });

    cx.render(rsx! {
        div {
            class: "App",
            rsx!(
                div {
                    class: "flex flex-row",
                    rsx!(calendar::Calendar {
                        calendar_blocks: calendar_blocks,
                    })
                })
        }

    })
}
