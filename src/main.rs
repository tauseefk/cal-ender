mod algo;
mod components;
mod get_position_offsets;

mod prelude {
    pub use std::cmp::Ordering;
    pub use std::collections::HashMap;
    pub use std::collections::VecDeque;

    pub use core::fmt;
    pub use dioxus::prelude::*;
    pub use petgraph::dot::Dot;
    pub use petgraph::graph::{Graph, NodeIndex};
    pub use serde::Deserialize;
    pub use thiserror::Error;
    pub use uuid::Uuid;

    pub use crate::algo::calendar_block::*;
    pub use crate::algo::calendar_trie::*;
    pub use crate::components::{calendar_block, root, url_input};
    pub use crate::get_position_offsets::*;

    pub const MAX_COL_WIDTH: f64 = 500.0;
    pub const BLOCK_STACK_PADDING: f64 = 10.0;
}

use prelude::*;

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    let mut calendar_trie = CalendarTrie::new();

    let mut calendar_blocks = vec![
        CalendarBlock {
            id: Uuid::new_v4(),
            start_minute: 10,
            end_minute: 180,
            block_type: CalendarBlockType::Available,
        },
        CalendarBlock {
            id: Uuid::new_v4(),
            start_minute: 20,
            end_minute: 90,
            block_type: CalendarBlockType::Busy,
        },
        CalendarBlock {
            id: Uuid::new_v4(),
            start_minute: 20,
            end_minute: 90,
            block_type: CalendarBlockType::Busy,
        },
        CalendarBlock {
            id: Uuid::new_v4(),
            start_minute: 20,
            end_minute: 110,
            block_type: CalendarBlockType::Busy,
        },
        CalendarBlock {
            id: Uuid::new_v4(),
            start_minute: 140,
            end_minute: 155,
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

    calendar_blocks.iter().for_each(|block| {
        let _ = calendar_trie.add(*block, None);
    });

    calendar_trie.display();
    let flattened_blocks = calendar_trie.traverse();

    cx.render(rsx! {
        div {
            class: "calendar-container",
            div {
                class: "calendar flex noselect",
                flattened_blocks.iter().map(move |flattened_block|
                    rsx!(calendar_block::CalendarBlockListItem {
                        calendar_block: flattened_block.block,
                        stack_position: flattened_block.stack_position,
                    })
                )
            }
        }
    })
}
