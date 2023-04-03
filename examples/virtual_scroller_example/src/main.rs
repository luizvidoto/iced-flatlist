use std::fmt::Display;

use fake::faker::name::en::Name;
use fake::{Dummy, Fake, Faker};
use iced::widget::{button, container, row, text, Column};
use iced::{Element, Length, Sandbox, Settings};
use iced_flatlist::{get_start_end_pos, NewScrollable};

pub fn main() {
    Example::run(Settings::default()).unwrap_or_else(|err| {
        eprintln!("An error occurred: {}", err);
    })
}

struct Example {
    users: Vec<User>,
    p_scroll_offset: f32,
    item_height: f32,
}

#[derive(Debug, Clone)]
pub enum Message {
    DetailsPress(User),
    SetScrollOffset(f32),
}

#[derive(Debug, Dummy, Clone)]
pub struct User {
    n: usize,
    #[dummy(faker = "1000..2000")]
    order_id: usize,
    customer: String,
    paid: bool,
    item_height: f32,
}
impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.customer)
    }
}
impl User {
    pub fn new(n: usize, item_height: f32) -> Self {
        Self {
            n,
            order_id: Faker::fake(&Faker),
            customer: Name().fake(),
            paid: Faker::fake(&Faker),
            item_height,
        }
    }
    pub fn header() -> Element<'static, Message> {
        row![
            text("#").width(Length::Fill),
            text("Order ID").width(Length::Fill),
            text("Customer").width(Length::Fill),
            text("Paid").width(Length::Fill),
            text("Action"),
        ]
        .width(Length::Fill)
        .into()
    }
    pub fn view(&self) -> Element<Message> {
        row![
            text(self.n).width(Length::Fill),
            text(self.order_id).width(Length::Fill),
            text(&self.customer).width(Length::Fill),
            text(self.paid).width(Length::Fill),
            button(text("Details")).on_press(Message::DetailsPress(self.to_owned()))
        ]
        .width(Length::Fill)
        .height(self.item_height)
        .into()
    }
}

impl Sandbox for Example {
    type Message = Message;

    fn new() -> Self {
        let item_height = 40.0;
        let mut users: Vec<User> = vec![];
        for n in 0..100_000 {
            users.push(User::new(n, item_height));
        }
        Example {
            users,
            item_height,
            p_scroll_offset: 0.0,
        }
    }

    fn title(&self) -> String {
        String::from("Custom widget - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::DetailsPress(user) => {
                println!("{:?}", user);
            }
            Message::SetScrollOffset(offset) => self.p_scroll_offset = offset,
        }
    }

    fn view(&self) -> Element<Message> {
        let view_height = 400.0;
        let items_len = self.users.len();
        let total_height = items_len as f32 * self.item_height;
        let (start_index, end_index) = get_start_end_pos(
            items_len,
            self.p_scroll_offset,
            self.item_height,
            view_height,
        );
        let visible_items: Element<_> = self
            .users
            .iter()
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
                |col, user| col.push(user.view()),
            )
            .into();

        let scrollable =
            NewScrollable::new(visible_items).on_scroll(|r_off| Message::SetScrollOffset(r_off.y));

        container(scrollable)
            .width(Length::Fill)
            .height(Length::Fixed(view_height))
            .center_x()
            .center_y()
            .into()
    }
}
