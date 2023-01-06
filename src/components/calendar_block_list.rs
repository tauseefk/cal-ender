use crate::prelude::*;

#[derive(Props, PartialEq)]
pub struct CalendarBlockListProps {
    flattened_blocks: Vec<FlattenedCalendarBlock>,
}

#[allow(non_snake_case)]
pub fn CalendarBlockList(cx: Scope<CalendarBlockListProps>) -> Element {
    return cx.render(rsx!(div {
        class: "calendar flex noselect",
        cx.props.flattened_blocks.iter().map(move |flattened_block|
            rsx!(calendar_block::CalendarBlockListItem {
                calendar_block: flattened_block.block,
                stack_position: flattened_block.stack_position,
            })
        )
    }));
}
