use crate::prelude::*;

#[derive(Props, PartialEq)]
pub struct CalendarBlockListItemProps {
    calendar_block: CalendarBlock,
    stack_position: usize,
}

#[allow(non_snake_case)]
pub fn CalendarBlockListItem(cx: Scope<CalendarBlockListItemProps>) -> Element {
    let block = cx.props.calendar_block;
    let stack_position = cx.props.stack_position;

    let block_type_class = match block.block_type {
        CalendarBlockType::Wrapper => "wrapper",
        CalendarBlockType::Busy => "busy",
        CalendarBlockType::Available => "available",
    };

    let height = block.end_minute - block.start_minute;
    let (left_offset, width) = get_position_offsets(stack_position);

    return cx.render(rsx!(div {
        class: "absolute calendar-block {block_type_class}",
        key: "{block.start_minute}",
        top: "{block.start_minute}px",
        left: "{left_offset}px",
        height: "{height}px",
        opacity: "1",
        width: "{width}px",
        "{block.start_minute}"
    }));
}
