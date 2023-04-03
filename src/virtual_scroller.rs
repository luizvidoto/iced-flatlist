use iced::widget::{container, text, Column};
use iced::{Element, Length};

use crate::get_start_end_pos;
use crate::new_scrollable::NewScrollable;

pub trait WithView {
    type Message: std::fmt::Debug + Send;
    fn view(&self) -> Element<'_, Self::Message>;
}

#[derive(Debug, Clone)]
pub enum Message {
    SetScrollOffset(f32),
}

pub struct VirtualScroller<T>
where
    T: WithView,
{
    items: Vec<T>,
    p_scroll_offset: f32,
    item_height: f32,
}

impl<T> VirtualScroller<T>
where
    T: WithView,
{
    pub fn new(items: impl IntoIterator<Item = T>, item_height: f32) -> Self {
        Self {
            items: items.into_iter().collect(),
            item_height,
            p_scroll_offset: 0.0,
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::SetScrollOffset(offset) => self.p_scroll_offset = offset,
        }
    }

    pub fn view(&self, view_height: f32) -> Element<'_, Message> {
        let items_len = self.items.len();
        let total_height = items_len as f32 * self.item_height;
        let (start_index, end_index) = get_start_end_pos(
            items_len,
            self.p_scroll_offset,
            self.item_height,
            view_height,
        );

        // cant make this work :(
        let visible_items =
            create_visible_items(self.items.iter(), start_index, end_index, total_height);

        let scrollable =
            NewScrollable::new(visible_items).on_scroll(|r_off| Message::SetScrollOffset(r_off.y));

        container(text("test"))
            .width(Length::Fill)
            .height(Length::Fixed(view_height))
            .center_x()
            .center_y()
            .into()
    }
}

fn create_visible_items<'a, T, I, Message>(
    items: I,
    start_index: usize,
    end_index: usize,
    total_height: f32,
) -> Element<'a, Message>
where
    T: 'a + Into<Element<'a, Message>>,
    I: 'a + IntoIterator<Item = &'a T>,
    Message: 'a,
{
    items
        .into_iter()
        .enumerate()
        .filter_map(|(idx, item)| {
            if idx >= start_index && idx < end_index {
                Some(item)
            } else {
                None
            }
        })
        .fold(
            Column::new().height(Length::Fixed(total_height)),
            |col, item| col.push(<T as Into<Element<'a, Message>>>::into(*item)),
        )
        .into()
}
