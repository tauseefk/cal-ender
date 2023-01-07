use crate::prelude::*;

#[derive(Props)]
pub struct CalendarBlockListItemProps<'block> {
    top: f64,
    left: f64,
    width: f64,
    height: f64,
    opacity: u8,
    label: &'block str,
    block_type: CalendarBlockType,
    stack_position: usize,
    onmousedown: EventHandler<'block, MouseEvent>,
    onmouseup: EventHandler<'block, MouseEvent>,
}

#[allow(non_snake_case)]
pub fn CalendarBlockListItem<'block>(
    cx: Scope<'block, CalendarBlockListItemProps<'block>>,
) -> Element {
    let stack_position = cx.props.stack_position;

    let block_type_class = match cx.props.block_type {
        CalendarBlockType::Wrapper => "wrapper",
        CalendarBlockType::Busy => "busy",
        CalendarBlockType::Available => "available",
    };

    return cx.render(rsx!(div {
        class: "absolute calendar-block {block_type_class}",
        top: "{cx.props.top}px",
        left: "{cx.props.left}px",
        height: "{cx.props.height}px",
        width: "{cx.props.width}px",
        opacity: "{cx.props.opacity}%",
        onmousedown: move |evt| cx.props.onmousedown.call(evt),
        onmouseup: move |evt| cx.props.onmouseup.call(evt),
        "{cx.props.label}"
    }));
}
