use crate::prelude::*;

#[derive(Props)]
pub struct CalendarBlockListItemProps<'block> {
    class: Option<&'block str>,
    top: String,
    left: String,
    width: String,
    height: String,
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
    let classes = match cx.props.class {
        Some(c) => c.to_string(),
        None => "".to_string(),
    };

    return cx.render(rsx!(div {
        class: "absolute calendar-block {block_type_class} {classes}",
        title: "{cx.props.label}",
        top: "{cx.props.top}",
        left: "{cx.props.left}",
        height: "{cx.props.height}",
        width: "{cx.props.width}",
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
