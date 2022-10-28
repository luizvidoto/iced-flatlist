use iced::{Element, Size};

use crate::Offsetter;

pub fn flatlist<'a, Message, Renderer, T: Sized>(
    size: Size,
    row_height: f32,
    velocity: f32,
    items: &'a [T],
    f: impl Fn(Vec<&'a T>) -> Element<'a, Message, Renderer> + 'a,
) -> Offsetter<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: iced_native::Renderer + 'a,
{
    let rows = (size.height / row_height) as f32;
    let items_len = items.len() as f32;
    Offsetter::new(size, velocity, move |offset| {
        let offset_pct = offset / size.height;
        let min = (items_len * offset_pct).max(0.0).min(items_len - rows);
        let max = min + rows;
        let filtered_items: Vec<&T> = items
            .iter()
            .enumerate()
            .filter_map(|(idx, item)| {
                let idx_f = idx as f32;
                if idx_f >= min && idx_f <= max {
                    Some(item)
                } else {
                    None
                }
            })
            .collect();
        f(filtered_items)
    })
}
