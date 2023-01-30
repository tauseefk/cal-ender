use crate::prelude::*;

#[derive(Props)]
pub struct CalendarProps<'cal> {
    calendar_blocks: &'cal UseState<Vec<CalendarBlock>>,
}

fn get_time_from_minutes(minutes: u32) -> String {
    let start_time = (minutes as f64) / 60.;
    let start_hour = start_time.floor();
    let start_time_remainder = (start_time - start_hour) * 60.;

    format!("{:02}{:02}", start_hour as u32, start_time_remainder as u32)
}

#[allow(non_snake_case)]
pub fn Calendar<'cal>(cx: Scope<'cal, CalendarProps<'cal>>) -> Element {
    let ghost_block_top = use_state(&cx, || 0_f64);
    let click_offset = use_state(&cx, || 0_f64);
    let dragged_block = use_state(&cx, || None::<FlattenedCalendarBlock>);
    let use_g_stacking_algorithm = use_state(&cx, || true);

    let mut calendar_trie = CalendarTrie::new();
    cx.props.calendar_blocks.get().iter().for_each(|block| {
        let block = (*block).clone();
        let _ = calendar_trie.add(block, None);
    });

    calendar_trie.display();
    let flattened_blocks = calendar_trie.traverse();
    let handle_ghost_block_drag = move |evt: MouseEvent| {
        if dragged_block.get().is_some() {
            let position_y = evt.client_y as f64;
            let destination_pos = ((position_y - click_offset.get()) / 15.).floor() * 15.;
            ghost_block_top.set(destination_pos);
        }
    };

    let handle_move_calendar_block = move |_| {
        if let Some(dragged_block_value) = dragged_block.get() {
            let mut updated_blocks: Vec<CalendarBlock> = cx
                .props
                .calendar_blocks
                .get()
                .iter()
                .map(|block| {
                    let offset = *ghost_block_top.get() as u32;
                    let (start_minute, end_minute) = match block.id == dragged_block_value.block.id
                    {
                        true => (
                            offset,
                            dragged_block_value.block.end_minute
                                - dragged_block_value.block.start_minute
                                + offset,
                        ),
                        false => (block.start_minute, block.end_minute),
                    };
                    CalendarBlock {
                        id: block.id,
                        block_type: block.block_type,
                        subtree_depth: block.subtree_depth,
                        end_minute,
                        start_minute,
                        label: block.label.clone(),
                    }
                })
                .collect();
            updated_blocks.sort_by(|a, b| {
                if a.start_minute < b.start_minute
                    || a.start_minute == b.start_minute && a.end_minute >= b.end_minute
                {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
            cx.props.calendar_blocks.set(updated_blocks);
            dragged_block.set(None);
        };
    };

    let ghost_block = match dragged_block.get() {
        Some(d_block) => {
            let height = (d_block.block.end_minute - d_block.block.start_minute) as f64;
            let label = format!(
                "{}, {}",
                d_block.block.label,
                get_time_from_minutes(d_block.block.start_minute)
            );

            rsx!(calendar_block::CalendarBlockListItem {
                class: "ghost",
                top: format!("{}px", *ghost_block_top.get()),
                left: format!("{}px", 0),
                height: format!("{}px", height),
                width: format!("{}px", MAX_COL_WIDTH),
                opacity: 100,
                label: "{label}",
                block_type: d_block.block.block_type,
                onmouseup: handle_move_calendar_block,
                onmousemove: handle_ghost_block_drag,
            })
        }
        None => rsx!(empty_element::EmptyElement {}),
    };
    return cx.render(rsx! {
        button {
            class: "btn",
            onclick: move |_| {
                use_g_stacking_algorithm.set(!use_g_stacking_algorithm.get());
            },
            "Switch Stacking Algorithm"
        }
        div {
            class: "calendar-container",
            div {
                class: "calendar flex noselect",
                onmousemove: handle_ghost_block_drag,
                flattened_blocks.iter().map(move |flattened_block|
                    {
                        let flattened_block = (*flattened_block).clone();
                        let flattened_block_2 = flattened_block.clone();
                        let dragged_block_option = dragged_block.get();

                        let opacity = match dragged_block_option.is_some()
                            && (flattened_block.block.id.to_string() == dragged_block_option.as_ref().unwrap().block.id.to_string()) {
                            true => 50,
                            false => 100,
                        };

                        let (left, width) = match use_g_stacking_algorithm.get() {
                            true => get_g_transforms(flattened_block.stack_position, flattened_block.block.subtree_depth),
                            false => get_position_offsets(flattened_block.stack_position)
                        };
                        let top = format!("{}px", flattened_block.block.start_minute);
                        let height = format!("{}px", flattened_block.block.end_minute - flattened_block.block.start_minute);

                        let label = format!("{}, {}",
                            flattened_block.block.label,
                            get_time_from_minutes(flattened_block.block.start_minute));


                        return rsx!(calendar_block::CalendarBlockListItem {
                            key: "{flattened_block.block.id}",
                            left: left,
                            top: top,
                            width: width,
                            height: height,
                            label: "{label}",
                            block_type: flattened_block.block.block_type,
                            opacity: opacity,
                            onmousedown: move |evt: MouseEvent| {
                                let flattened_block = flattened_block_2.clone();
                                dragged_block.set(Some(flattened_block.clone()));
                                ghost_block_top.set(flattened_block.block.start_minute as f64);
                                click_offset.set(evt.client_y  as f64 - flattened_block.block.start_minute as f64);
                            },
                            onmouseup: handle_move_calendar_block,
                        });
                    }
                )
                rsx!(ghost_block)
            }
        }
    });
}
