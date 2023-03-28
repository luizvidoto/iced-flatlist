use std::fmt::Display;

use fake::faker::name::en::Name;
use fake::{Dummy, Fake, Faker};
use iced::widget::{button, container, row, text};
use iced::{Element, Length, Sandbox, Settings};
use iced_flatlist::virtual_scroller::{self, VirtualScroller};

pub fn main() {
    Example::run(Settings::default()).expect("Must not fail")
}

struct Example {
    users: Vec<User>,
    scroller: VirtualScroller<User>,
}

#[derive(Debug, Clone)]
pub enum Message {
    DetailsPress(User),
    VirtualScrollerMsg(virtual_scroller::Message),
}

#[derive(Debug, Dummy, Clone)]
pub struct User {
    n: usize,
    #[dummy(faker = "1000..2000")]
    order_id: usize,
    customer: String,
    paid: bool,
}
impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.customer)
    }
}
impl User {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            order_id: Faker::fake(&Faker),
            customer: Name().fake(),
            paid: Faker::fake(&Faker),
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
    pub fn view(&self, row_h: u16) -> Element<Message> {
        row![
            text(self.n).width(Length::Fill),
            text(self.order_id).width(Length::Fill),
            text(&self.customer).width(Length::Fill),
            text(self.paid).width(Length::Fill),
            button(text("Details")).on_press(Message::DetailsPress(self.to_owned()))
        ]
        .width(Length::Fill)
        .height(row_h)
        .into()
    }
}

impl Sandbox for Example {
    type Message = Message;

    fn new() -> Self {
        let item_height = 40.0;
        let mut users: Vec<User> = vec![];
        for n in 0..100_000 {
            users.push(User::new(n));
        }
        let scroller = VirtualScroller::new(users.clone(), item_height, 400.0);
        Example { users, scroller }
    }

    fn title(&self) -> String {
        String::from("Custom widget - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::DetailsPress(user) => {
                println!("{:?}", user);
            }
            Message::VirtualScrollerMsg(msg) => self.scroller.update(msg),
        }
    }

    fn view(&self) -> Element<Message> {
        container(self.scroller.view().map(Message::VirtualScrollerMsg))
            .width(Length::Fill)
            .height(Length::Fixed(400.0))
            .center_x()
            .center_y()
            .into()
    }
}
