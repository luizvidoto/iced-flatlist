use fake::faker::name::en::Name;
use fake::{Dummy, Fake, Faker};
use iced::widget::{button, container, text};
use iced::{Element, Length, Sandbox, Settings};
use iced_flatlist::flatlist;
use iced_native::{column, row};

pub fn main() {
    Example::run(Settings::default()).expect("Must not fail")
}

#[derive(Debug)]
struct Example {
    users: Vec<User>,
}

#[derive(Debug, Clone)]
pub enum Message {
    DetailsPress(User),
}

#[derive(Debug, Dummy, Clone)]
pub struct User {
    n: usize,
    #[dummy(faker = "1000..2000")]
    order_id: usize,
    customer: String,
    paid: bool,
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
    pub fn view(&self) -> Element<Message> {
        row![
            text(self.n).width(Length::Fill),
            text(self.order_id).width(Length::Fill),
            text(&self.customer).width(Length::Fill),
            text(self.paid).width(Length::Fill),
            button(text("Details")).on_press(Message::DetailsPress(self.to_owned()))
        ]
        .width(Length::Fill)
        .into()
    }
}

impl Sandbox for Example {
    type Message = Message;

    fn new() -> Self {
        let mut users: Vec<User> = vec![];
        for n in 0..100_000_0 {
            users.push(User::new(n));
        }
        Example { users }
    }

    fn title(&self) -> String {
        String::from("Custom widget - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::DetailsPress(user) => {
                println!("{:?}", user);
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let row_height = 50.0;
        let velocity = 0.01;
        let lazy_content = iced_lazy::responsive(move |size| {
            flatlist(size, row_height, velocity, &self.users, move |users| {
                users
                    .iter()
                    .fold(column![User::header()], |column, user| {
                        column.push(user.view())
                    })
                    .into()
            })
            .into()
        });

        container(lazy_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
