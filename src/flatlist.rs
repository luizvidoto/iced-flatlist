use iced::{Element, Size};

use crate::Scroller;

pub fn flatlist<'a, Message, Renderer, T: Sized, I>(
    size: Size,
    row_h: f32,
    items: I,
    f: impl Fn(Vec<T>) -> Element<'a, Message, Renderer> + 'a,
) -> Scroller<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer,
    I: IntoIterator<Item = T> + Copy + 'a,
{
    let rows_fit = (size.height / row_h).floor() / 3.0;
    let item_count = items.into_iter().count();
    let scroll_by = rows_fit / item_count as f32 * size.height;

    Scroller::new(size, scroll_by, move |slider_pos| {
        let slider_pos_pct = slider_pos / size.height;
        let filtered_i = get_subset_by_slider_position(items, slider_pos_pct, row_h, size.height);
        f(filtered_i)
    })
}

pub fn get_subset_by_slider_position<'a, T, I>(
    items: I,
    slider_pos_pct: f32,
    row_h: f32,
    view_h: f32,
) -> Vec<T>
where
    I: IntoIterator<Item = T> + Copy + 'a,
{
    let mut result = Vec::new();
    let num_items = items.into_iter().count();
    let rows_fit = (view_h / row_h).floor() as usize;
    let start_min = (num_items as f32 * slider_pos_pct).floor() as usize;
    let start_max = num_items.checked_sub(rows_fit).unwrap_or(0);
    let start = start_min.max(0).min(start_max);
    // let end = (start + rows_fit).min(num_items);

    for item in items.into_iter().skip(start).take(rows_fit) {
        result.push(item);
    }

    result
}

pub fn get_start_end_pos(
    item_count: usize,
    scroll_pos_pct: f32,
    row_h: f32,
    view_h: f32,
) -> (usize, usize) {
    let rows_fit = (view_h / row_h).floor() as usize;
    let start_min = (item_count as f32 * scroll_pos_pct).floor() as usize;
    let start_max = item_count.checked_sub(rows_fit).unwrap_or(0);
    let start = start_min.max(0).min(start_max);
    let end = (start + rows_fit).min(item_count);
    (start, end)
}
