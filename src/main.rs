use iced::pure::widget::{Button, Column, Row, Text};
use iced::pure::{button, column, container, row, scrollable, text, Application, Element};
use iced::{alignment, executor, Alignment, Color, Command, Length, Settings};
use twitter_v2::{Tweet, User};
pub mod api;
pub mod style;
pub mod theme;
pub fn main() -> iced::Result {
    App::run(Settings::default())
}
#[derive(Debug, Clone)]
struct App {
    model: Vec<Snapshot>,
    index: usize,
}

#[derive(Debug, Clone)]

enum Message {
    DisplayTweet(Tweet), //you might want to get rid of this, or change display conversation to this
    DisplayUser(User),
    DisplayConversation(Tweet),
    Back,
    Forward,
}

#[derive(Debug, Clone)]
enum Snapshot {
    TweetView(Tweet),
    UserView(User),
    ConversationView(Tweet),
}

impl Application for App {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                model: vec![Snapshot::UserView(api::get_user_by_twitter_handle(
                    "yudapearl",
                ))],
                index: 0,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Better Twitter Archiver")
    }

    fn background_color(&self) -> Color {
        theme::MAIN_BG_COLOR
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::DisplayTweet(tweet) => {
                self.model.push(Snapshot::TweetView(tweet));
                self.index += 1;
                Command::none()
            }
            Message::DisplayUser(user) => {
                self.model.push(Snapshot::UserView(user));
                self.index += 1;
                Command::none()
            }
            Message::DisplayConversation(tweet) => {
                self.model.push(Snapshot::ConversationView(tweet));
                self.index += 1;
                Command::none()
            }
            Message::Back => {
                self.index -= 1;
                Command::none()
            }
            Message::Forward => {
                self.index += 1;
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let present = self.model[self.index].clone();
        let view_content = match present {
            Snapshot::TweetView(tweet) => row().push(view_tweet(&tweet)),
            Snapshot::UserView(user) => row().push(view_user_tweets(&user)),
            Snapshot::ConversationView(tweet) => row().push(view_conversation(&tweet)),
        };
        container(scrollable(
            column()
                .align_items(Alignment::Center)
                .width(Length::Units(700))
                .push(view_navigation())
                .push(view_content),
        ))
        .style(style::App)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(alignment::Horizontal::Center)
        .align_y(alignment::Vertical::Center)
        .max_width(100)
        .padding(20)
        .into()
    }
}

fn view_tweet<'a>(tweet: &Tweet) -> Button<'a, Message> {
    let user = api::get_user_by_id(tweet.author_id.expect("Failed to get tweet id").as_u64());
    button(
        column()
            .push(
                row()
                    .push(view_tweet_author_name(&user))
                    .push(view_tweet_datetime(&tweet))
                    .spacing(30),
            )
            .push(row().push(text(&tweet.text)))
            .spacing(10),
    )
    .style(style::Tweet)
    .width(Length::Fill)
    .padding(20)
    .on_press(Message::DisplayConversation(tweet.clone()))
}

fn view_tweet_author_name<'a>(user: &User) -> Text {
    text(format!("{} (@{})", user.name, user.username)).size(15)
}

fn view_tweet_datetime(tweet: &Tweet) -> Text {
    let format =
        time::format_description::parse("Posted on [year]/[month]/[day] at [hour]:[minute]")
            .expect("failed to get format");
    let tweet_datetime_string = tweet
        .created_at
        .expect("failed to parse date")
        .format(&format)
        .expect("Failed to format datetime");
    text(tweet_datetime_string).size(15)
}

fn view_tweets<'a>(tweets: &Vec<Tweet>) -> Column<'a, Message> {
    let tweet_view_list: Vec<Button<'a, Message>> =
        tweets.iter().map(|tweet| view_tweet(tweet)).collect();
    tweet_view_list
        .into_iter()
        .fold(column(), |tweets_view, tweet_view| {
            tweets_view.push(tweet_view)
        })
        .spacing(15)
}

fn view_user_tweets<'a>(user: &User) -> Column<'a, Message> {
    column()
        .push(row().push(view_user_timeline_title(user)))
        .push(row().push(view_tweets(&api::get_tweets_from_twitter_handle(
            &user.username,
        ))))
        .spacing(25)
}

fn view_user_timeline_title(user: &User) -> Text {
    text(format!("{}'s (@{}) Tweets", user.name, user.username))
        .size(30)
        .width(Length::Fill)
        .horizontal_alignment(iced::alignment::Horizontal::Center)
}

fn view_conversation<'a>(tweet: &Tweet) -> Column<'a, Message> {
    let mut conversation = api::get_conversation_by_tweet_id(tweet.id.as_u64());
    if conversation.len() > 1 {
        conversation.reverse();
        column()
            .push(view_conversation_title(tweet))
            .push(view_tweets(&conversation))
            .spacing(25)
    } else {
        column()
            .push(view_tweet_title(&tweet))
            .push(view_tweet(tweet))
    }
}

fn view_conversation_title(tweet: &Tweet) -> Text {
    let user = api::get_user_by_id(
        tweet
            .author_id
            .expect("Failed to get tweet's author id")
            .as_u64(),
    );
    text(format!(
        "Conversation containing @{}'s tweet posted on {}",
        user.username,
        get_tweet_created_datetime_string(&tweet)
    ))
    .size(30)
    .width(Length::Fill)
    .horizontal_alignment(iced::alignment::Horizontal::Center)
}

fn view_tweet_title(tweet: &Tweet) -> Text {
    let user = api::get_user_by_id(tweet.author_id.expect("Failed to parse tweet id").as_u64());
    text(format!(
        "Tweet by @{} posted at {}",
        user.username,
        get_tweet_created_datetime_string(tweet)
    ))
    .size(30)
    .width(Length::Fill)
    .horizontal_alignment(iced::alignment::Horizontal::Center)
}

fn view_navigation<'a>() -> Row<'a, Message> {
    row()
        .push(button("⬅").on_press(Message::Back))
        .push(button("⮕").on_press(Message::Forward))
}

fn get_tweet_created_datetime_string(tweet: &Tweet) -> String {
    let format = time::format_description::parse("[year]/[month]/[day] at [hour]:[minute]")
        .expect("failed to get format");
    tweet
        .created_at
        .expect("failed to parse date")
        .format(&format)
        .expect("Failed to format datetime")
}
