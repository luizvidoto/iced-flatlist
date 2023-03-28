use iced::{
    widget::{column, text},
    Element, Length,
};
use iced_native::widget::NewScrollable;

use crate::get_start_end_pos;

#[derive(Debug, Clone)]
pub enum Message {
    SetScrollOffset(f32),
    // SomeMsg(Message),
}

pub struct VirtualScroller<T> {
    items: Vec<T>,
    item_height: f32,
    view_height: f32,
    p_scroll_offset: f32,
    // renderer: Box<dyn Fn(&T) -> Element<'a, Message, Renderer>>,
}

impl<T> VirtualScroller<T> {
    pub fn new(
        items: Vec<T>,
        item_height: f32,
        view_height: f32,
        // renderer: impl Fn(&T) -> Element<'a, Message, Renderer>,
    ) -> Self {
        Self {
            items,
            item_height,
            view_height,
            p_scroll_offset: 0.0,
            // renderer: Box::new(renderer),
        }
    }
    pub fn update(&mut self, message: Message) {
        match message {
            Message::SetScrollOffset(offset) => {
                self.p_scroll_offset = offset;
            } // Message::SomeMsg(msg) => {
              //     todo!()
              // }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let total_height = self.items.len() as f32 * self.item_height;
        let (start_index, end_index) = get_start_end_pos(
            self.items.len(),
            self.p_scroll_offset,
            self.item_height,
            self.view_height,
        );

        let content: Element<_> = self.items[start_index..end_index]
            .iter()
            .enumerate()
            .fold(
                column![].height(Length::Fixed(total_height)),
                |col, (i, item)| {
                    // let el = (self.renderer)(item);
                    let el: Element<_> = text(format!("id: {}", start_index + i))
                        .height(Length::Fixed(self.item_height))
                        .width(Length::Fill)
                        .into();
                    col.push(el)
                },
            )
            .into();

        let scroller =
            NewScrollable::new(content).on_scroll(|r_off| Message::SetScrollOffset(r_off.y));

        scroller.into()
    }
}
