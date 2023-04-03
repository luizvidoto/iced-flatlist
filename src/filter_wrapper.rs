use iced::widget::{column, Column};
use iced::{Element, Length};

use crate::get_start_end_pos;

pub fn _filter_wrapper<'a, Message, Renderer>(
    items: impl IntoIterator<Item = impl Into<Element<'a, Message, Renderer>>> + Clone,
    item_height: f32,
    view_height: f32,
    p_scroll_offset: f32,
    header: Option<impl Into<Element<'a, Message, Renderer>>>,
) -> Element<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + iced_native::Renderer,
{
    let items_len = items.clone().into_iter().count();
    let total_height = items_len as f32 * item_height;
    let (start_index, end_index) =
        get_start_end_pos(items_len, p_scroll_offset, item_height, view_height);
    let mut column: Column<'a, _, _> = column![].height(Length::Fixed(total_height));
    if let Some(h) = header {
        column = column.push(h.into());
    }
    items
        .into_iter()
        .enumerate()
        .filter_map(|(index, item)| {
            if index >= start_index && index <= end_index {
                Some(item.into())
            } else {
                None
            }
        })
        .fold(column, |col, item: Element<'a, Message, Renderer>| {
            col.push(item)
        })
        .into()
}
