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
    onmousedown: Option<EventHandler<'block, MouseEvent>>,
    onmouseup: EventHandler<'block, MouseEvent>,
    onmousemove: Option<EventHandler<'block, MouseEvent>>,
}

#[allow(non_snake_case)]
pub fn CalendarBlockListItem<'block>(
    cx: Scope<'block, CalendarBlockListItemProps<'block>>,
) -> Element {
    let block_type_class = match cx.props.block_type {
        CalendarBlockType::Wrapper => "wrapper",
        CalendarBlockType::Busy => "busy",
        CalendarBlockType::Available => "available",
    };

    return cx.render(rsx!(div {
        class: "absolute calendar-block {block_type_class}",
        top: "{cx.props.top}px",
        left: "calc(100% * {cx.props.left})",
        height: "{cx.props.height}px",
        width: "calc(100% * {cx.props.width})",
        opacity: "{cx.props.opacity}%",
        onmousedown: move |evt| {
            if let Some(handle_mouse_down) = &cx.props.onmousedown {
                handle_mouse_down.call(evt);
            }
        },
        onmouseup: move |evt| cx.props.onmouseup.call(evt),
        onmousemove: move |evt| {
            if let Some(handle_mouse_move) = &cx.props.onmousemove {
                handle_mouse_move.call(evt);
            }
        },
        "{cx.props.label}"
    }));
}
