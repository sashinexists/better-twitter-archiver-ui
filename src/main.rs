use ::reqwest;

use iced::{
    button, container, scrollable, Alignment, Background, Button, Color, Column, Container,
    Element, Row, Sandbox, Scrollable, Settings, Text,
};
use ron;
use std::fmt::Debug;

use twitter_v2::{Tweet, User};

pub fn main() -> iced::Result {
    App::run(Settings::default())
}

#[derive(Default)]
struct App {
    model: Vec<Snapshot>,
    index: usize,
    scroll: scrollable::State,
    button: button::State,
}
#[derive(Debug, Clone)]
enum Snapshot {
    Tweetview(Tweet),
    Userview(User),
    ConversationView(Tweet),
}
/*
*/

#[derive(Debug, Clone)]
enum Message {
    DisplayTweet(Tweet),
    DisplayUser(User),
    DisplayConversation(Tweet),
    Init(Snapshot),
    Back,
}

fn get_tweets_from_twitter_handle(twitter_handle: &str) -> Vec<Tweet> {
    let resp = reqwest::blocking::get(format!(
        "http://127.0.0.1:4001/user/{twitter_handle}/tweets"
    ))
    .expect("failed to get @{twitter_handle}'s tweets");
    let tweet_ron_str = resp
        .text()
        .expect("failed to parse text from @{twitter_handle}'s tweets");
    ron::from_str(&tweet_ron_str)
        .expect("Failed to parse @{twitter_handle}'s tweets data into Rusty Object Notation")
}

fn get_tweet_by_id(id: u64) -> Tweet {
    let resp = reqwest::blocking::get(format!("http://127.0.0.1:4001/tweet/{id}"))
        .expect("failed to get tweet");
    let tweet_ron_str = resp.text().expect("failed to parse text");
    ron::from_str(&tweet_ron_str).expect("Failed to parse tweet data into Rusty Object Notation")
}

fn get_user_by_id(id: u64) -> User {
    let resp = reqwest::blocking::get(format!("http://127.0.0.1:4001/userid/{id}"))
        .expect("failed to get user from id {id}");
    let user_ron_str = resp
        .text()
        .expect("failed to parse text from response to request for user of id {id}'s data");
    ron::from_str(&user_ron_str)
        .expect("Failed to parse user of id {id}'s data into Rusty Object Notation")
}

fn get_user_by_twitter_handle(twitter_handle: &str) -> User {
    let resp = reqwest::blocking::get(format!("http://127.0.0.1:4001/user/{twitter_handle}/info"))
        .expect("failed to get user from twitter handle {twitter_handle}");
    let user_ron_str = resp
        .text()
        .expect("failed to parse text from response for user @{twitter_handle}");
    ron::from_str(&user_ron_str)
        .expect("Failed to parse @{twitter_handle} user data into Rusty Object Notation")
}

fn view_user_timeline<'a>(user: &User) -> Container<'a, Message> {
    let user_timeline = Column::<'a, Message>::new()
        .spacing(20)
        .push(
            Text::new(format!("{}'s (@{}) Tweets", user.name, user.username))
                .size(30)
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .width(iced::Length::Units(700))
                .color(iced::Color::from_rgb(0.96, 0.956, 0.96)),
        )
        .push(view_tweets(&get_tweets_from_twitter_handle(&user.username)));

    Container::new(user_timeline)
}

fn view_tweets<'a>(tweets: &Vec<Tweet>) -> Container<'a, Message> {
    let tweet_view_list: Vec<Container<'a, Message>> =
        tweets.iter().map(|tweet| view_tweet(tweet)).collect();

    let tweet_column = tweet_view_list
        .into_iter()
        .fold(Column::<'a, Message>::new(), |tweets_view, tweet_view| {
            tweets_view.push(tweet_view)
        })
        .spacing(15);

    Container::new(tweet_column)
}

fn view_tweet<'a>(tweet: &Tweet) -> Container<'a, Message> {
    let tweet = get_tweet_by_id(tweet.id.as_u64());
    let user = get_user_by_id(tweet.author_id.expect("Failed to parse author id").as_u64());
    let twitter_name = format!("{} (@{})", user.name, user.username);
    let format =
        time::format_description::parse("Posted on [year]/[month]/[day] at [hour]:[minute]")
            .expect("failed to get format");
    let tweet_datetime = tweet
        .created_at
        .expect("failed to parse date")
        .format(&format)
        .expect("Failed to format datetime");
    let content = Column::new()
        .spacing(10)
        .width(iced::Length::Units(700))
        .push(
            Row::new()
                .align_items(iced::Alignment::End)
                .spacing(30)
                .push(
                    Text::new(twitter_name)
                        .size(15)
                        .horizontal_alignment(iced::alignment::Horizontal::Left)
                        .color(iced::Color::from_rgb(0.96, 0.956, 0.96)),
                )
                .push(
                    Text::new(format!("{}", tweet_datetime))
                        .size(15)
                        .horizontal_alignment(iced::alignment::Horizontal::Right)
                        .color(iced::Color::from_rgb(0.96, 0.956, 0.96)),
                ),
        )
        .push(
            Row::new().push(
                Text::new(tweet.text)
                    .size(20)
                    .horizontal_alignment(iced::alignment::Horizontal::Left)
                    .color(iced::Color::from_rgb(0.96, 0.956, 0.96)),
            ),
        );
    Container::new(content).style(style::Container).padding(20)
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        let mut init = Self::default();
        init.update(Message::Init(Snapshot::Userview(
            get_user_by_twitter_handle("yudapearl"),
        )));
        init
    }

    fn title(&self) -> String {
        String::from("Better Twitter Archiver")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Init(snapshot) => {
                self.model.push(snapshot);
                self.index = 0;
            }
            Message::DisplayTweet(tweet) => {
                self.model.push(Snapshot::Tweetview(tweet));
                self.index += 1;
            }
            Message::DisplayUser(user) => {
                self.model.push(Snapshot::Userview(user));
                self.index += 1;
            }
            Message::DisplayConversation(tweet) => {
                self.model.push(Snapshot::ConversationView(tweet));
                self.index += 1;
            }
            Message::Back => {
                self.index -= 1;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let present = self.model[self.index].clone();
        let view_content = match present {
            Snapshot::Userview(user) => view_user_timeline(&user),
            Snapshot::Tweetview(tweet) => Container::new(view_tweet(&tweet)),
            _ => Container::new(Text::new("Display failed")),
        };

        Scrollable::new(&mut self.scroll)
            .padding(15)
            .width(iced::Length::Fill)
            .align_items(Alignment::Center)
            .push(view_content)
            .into()
    }

    fn background_color(&self) -> iced::Color {
        theme::MAIN_BG_COLOR
    }

    fn scale_factor(&self) -> f64 {
        1.0
    }

    fn should_exit(&self) -> bool {
        false
    }

    fn run(settings: Settings<()>) -> Result<(), iced::Error>
    where
        Self: 'static + Sized,
    {
        <Self as iced::Application>::run(settings)
    }
}

mod style {
    use iced::{button, container};

    use crate::theme;

    pub struct Container;

    impl container::StyleSheet for Container {
        fn style(&self) -> container::Style {
            container::Style {
                background: theme::CONTENT_BG_COLOR.into(),
                text_color: theme::MAIN_TEXT_COLOR.into(),
                border_radius: 10.0,
                ..container::Style::default()
            }
        }
    }

    pub struct Button;

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: theme::CONTENT_BG_COLOR.into(),
                text_color: theme::MAIN_TEXT_COLOR.into(),
                border_radius: 10.0,
                ..button::Style::default()
            }
        }

        fn hovered(&self) -> button::Style {
            button::Style {
                text_color: theme::MAIN_TEXT_COLOR.into(),
                ..self.active()
            }
        }
    }
}

mod theme {
    use iced::Color;
    pub const MAIN_TEXT_COLOR: Color = colors::CULTURED_WHITE;
    pub const CONTENT_BG_COLOR: Color = colors::RAISIN_BLACK;
    pub const MAIN_BG_COLOR: Color = colors::RICH_BLACK;
    mod colors {
        use iced::Color;
        pub const RAISIN_BLACK: Color = Color::from_rgb(0.12, 0.11, 0.12);
        pub const RICH_BLACK: Color = Color::from_rgb(0.047, 0.035, 0.0431);
        pub const CULTURED_WHITE: Color = Color::from_rgb(0.96, 0.956, 0.96);
    }
}
