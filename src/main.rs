mod algo;
mod components;
mod get_position_offsets;

mod prelude {
    pub use std::cmp::Ordering;
    pub use std::collections::HashMap;
    pub use std::collections::VecDeque;

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
    pub use crate::components::{calendar_block, empty_element, root, url_input};
    pub use crate::get_position_offsets::*;

    pub const MAX_COL_WIDTH: f64 = 500.0;
    pub const BLOCK_STACK_PADDING: f64 = 10.0;
}

use prelude::{empty_element::EmptyElement, *};

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
    let flattened_blocks = use_state(&cx, || calendar_trie.traverse());

    cx.render(rsx! {
        Calendar {
            calendar_blocks: flattened_blocks,
        }
    })
}

#[derive(Props)]
pub struct CalendarProps<'cal> {
    calendar_blocks: &'cal Vec<FlattenedCalendarBlock>,
}

#[allow(non_snake_case)]
pub fn Calendar<'cal>(cx: Scope<'cal, CalendarProps<'cal>>) -> Element {
    let flattened_blocks = &cx.props.calendar_blocks;
    let ghost_block_top = use_state(&cx, || 0_f64);
    let click_offset = use_state(&cx, || 0_f64);

    let dragged_block = use_state(&cx, || None::<FlattenedCalendarBlock>);

    let handle_ghost_block_drag = move |evt: MouseEvent| {
        if let Some(_) = dragged_block.get() {
            let position_y = evt.client_y as f64;
            let destination_pos = ((position_y - click_offset.get()) / 15.).floor() * 15.;
            ghost_block_top.set(destination_pos);
        }
    };

    let ghost_block = match dragged_block.get() {
        Some(d_block) => {
            let height = (d_block.block.end_minute - d_block.block.start_minute) as f64;
            rsx!(calendar_block::CalendarBlockListItem {
                top: *ghost_block_top.get(),
                left: 0.,
                height: height,
                width: 500.,
                label: "{d_block.block.start_minute} - {d_block.block.end_minute}",
                block_type: d_block.block.block_type,
                stack_position: 4,
                onmousedown: move |_| {},
                opacity: 100,
                onmouseup: move |_| {
                    dragged_block.set(None);
                }
            })
        }
        None => rsx!(empty_element::EmptyElement {}),
    };
    return cx.render(rsx! {
        div {
            class: "calendar-container",
            div {
                class: "calendar flex noselect",
                onmousemove: handle_ghost_block_drag,
                flattened_blocks.iter().map(move |flattened_block|
                    {
                        let flattened_block = flattened_block.clone();
                        let dragged_block_option = dragged_block.get();

                        let opacity = match dragged_block_option.is_some() && (flattened_block.block.id.to_string() == dragged_block_option.unwrap().block.id.to_string()) {
                            true => 50,
                            false => 100,
                        };

                        let (left, width) = get_position_offsets(flattened_block.stack_position);
                        let top = flattened_block.block.start_minute as f64;
                        let height = (flattened_block.block.end_minute - flattened_block.block.start_minute) as f64;

                        return rsx!(calendar_block::CalendarBlockListItem {
                            key: "{flattened_block.block.id}",
                            left: left,
                            top: top,
                            width: width,
                            height: height,
                            label: "{flattened_block.block.start_minute} - {flattened_block.block.end_minute}",
                            block_type: flattened_block.block.block_type,
                            stack_position: flattened_block.stack_position,
                            opacity: opacity,
                            onmousedown: move |evt: MouseEvent| {
                                dragged_block.set(Some(flattened_block));
                                ghost_block_top.set(flattened_block.block.start_minute as f64);
                                click_offset.set(evt.client_y  as f64 - flattened_block.block.start_minute as f64);
                            },
                            onmouseup: move |_| { dragged_block.set(None); }
                        });
                    }
                )
                rsx!(ghost_block)
            }
        }
    });
}
