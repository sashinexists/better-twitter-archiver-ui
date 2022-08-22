use app::data::setup;
use iced::futures::executor::block_on;
use iced::pure::widget::{Button, Column, Row, Text, TextInput};
use iced::pure::{
    button, column, container, row, scrollable, text, text_input, Application, Element, Widget,
};
use iced::{alignment, executor, Alignment, Color, Command, Length, Settings};
use sea_orm::DatabaseConnection;
use twitter_v2::oauth2::url::quirks::search;
use twitter_v2::{Tweet, User};
use utils::{SelectList, TweetData, UserData};

pub mod app;
pub mod style;
pub mod theme;
pub mod utils;
pub mod seed;
const USER_TWITTER_HANDLE: &str = "yudapearl";

pub fn main() -> iced::Result {
    App::run(Settings::default())
}
#[derive(Debug, Clone)]
struct App {
    model: SelectList<Snapshot>,
    config: Config,
    search_input: String,
    data: DatabaseConnection,
}

#[derive(Debug, Clone)]
struct Config {
    tweets_per_page: usize,
}

#[derive(Debug, Clone)]

enum Message {
    DisplayTweet(TweetData), //you might want to get rid of this, or change display conversation to this
    DisplayUsersTweets(User),
    DisplayConversation(TweetData),
    Home,
    Back,
    Forward,
    ViewMoreTweets,
    SearchInputChanged(String),
    Search(String),
    SeedConversations,
}

#[derive(Debug, Clone, PartialEq)]
enum Snapshot {
    TweetView(TweetData),
    UserView(User, Vec<TweetData>),
    ConversationView(Vec<TweetData>),
    SearchView(String, Vec<TweetData>),
}

impl Application for App {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let db = block_on(setup::set_up_db()).expect("Failed to set up database");
        (
            Self {
                model: SelectList::new(Snapshot::UserView(
                    block_on(app::load_user_from_twitter_handle(&db, USER_TWITTER_HANDLE)),
                    block_on(app::load_users_tweets_from_twitter_handle(
                        &db,
                        USER_TWITTER_HANDLE,
                    )),
                )),
                search_input: "".to_string(),
                config: Config {
                    tweets_per_page: 100,
                },
                data: db,
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
                self.model.add(Snapshot::TweetView(tweet));
                Command::none()
            }
            Message::DisplayUsersTweets(user) => {
                let users_tweets = block_on(app::load_users_tweets_from_twitter_handle(
                    &self.data,
                    &user.username,
                ));
                self.model.add(Snapshot::UserView(user, users_tweets));
                Command::none()
            }
            Message::DisplayConversation(tweet_data) => {
                let conversation = block_on(app::load_conversation_from_tweet_id(
                    &self.data,
                    tweet_data
                        .tweet
                        .conversation_id
                        .expect("bad conversation id")
                        .as_u64()
                        .try_into()
                        .expect("Failed to parse u64 into i64"),
                ));

                self.model.add(Snapshot::ConversationView(conversation));
                Command::none()
            }
            Message::Home => {
                self.model.add(self.model.previous[0].clone());
                Command::none()
            }
            Message::Back => {
                self.model.previous();
                Command::none()
            }
            Message::Forward => {
                self.model.forward();
                Command::none()
            }
            Message::ViewMoreTweets => {
                self.config.tweets_per_page += 50;
                Command::none()
            }
            Message::SearchInputChanged(input) => {
                self.search_input = input;
                Command::none()
            }
            Message::Search(search_query) => {
                let search = block_on(app::search_tweets_in_db(&self.data, &search_query));
                self.model.add(Snapshot::SearchView(search_query, search));
                Command::none()
            }
            Message::SeedConversations => {
                let users_tweets = block_on(app::load_users_tweets_from_twitter_handle(
                    &self.data,
                    "yudapearl",
                ));
                println!("seeding conversations...");
                block_on(app::seed_conversation_from_tweets(
                    &self.data,
                    &users_tweets,
                ));
                //self.model.add(Snapshot::UserView(user, users_tweets));
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let present = &self.model.selected;
        let view_content = match present {
            Snapshot::TweetView(tweet) => render_tweet_view(self, &tweet),
            Snapshot::UserView(user, tweets) => render_user_timeline_view(self, &user, &tweets),
            Snapshot::ConversationView(tweet) => render_conversation_view(self, tweet),
            Snapshot::SearchView(search_query, search_results) => {
                render_search_view(self, search_query, search_results)
            }
        };
        container(scrollable(
            column()
                .align_items(Alignment::Center)
                .width(Length::Units(700))
                .push(view_content),
        ))
        .style(style::App)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(alignment::Horizontal::Center)
        .max_width(100)
        .padding(20)
        .into()
    }
}

fn render_user_timeline_view<'a>(
    app: &App,
    user: &User,
    tweets: &Vec<TweetData>,
) -> Row<'a, Message> {
    row().push(
        column()
            .push(view_user_timeline_title(&user))
            .push(view_navigation(app))
            .push(row().push(view_user_tweets(tweets, app.config.tweets_per_page)))
            .spacing(10),
    )
}

fn render_tweet_view<'a>(app: &App, tweet_data: &TweetData) -> Row<'a, Message> {
    row().push(
        column()
            .push(view_tweet_title(&tweet_data))
            .push(view_navigation(app))
            .push(row().push(view_tweet(&tweet_data)))
            .spacing(10),
    )
}

fn render_conversation_view<'a>(app: &App, conversation: &Vec<TweetData>) -> Row<'a, Message> {
    if &conversation.len() > &1 {
        row().push(
            column()
                .push(view_conversation_title(&conversation[0]))
                .push(view_navigation(app))
                .push(view_conversation(conversation))
                .spacing(10),
        )
    } else {
        row().push(
            column()
                .push(column().push(view_tweet_title(&conversation[0])))
                .push(view_navigation(app))
                .push(view_tweet(&conversation[0]))
                .spacing(10),
        )
    }
}

fn render_search_view<'a>(
    app: &App,
    search_query: &str,
    search_results: &Vec<TweetData>,
) -> Row<'a, Message> {
    let number_of_results = &search_results.len();
    if &search_results.len() > &0 {
        row().push(
            column()
                .push(view_search_title(number_of_results, &search_query))
                .push(view_navigation(app))
                .push(view_tweets(&search_results))
                .spacing(10),
        )
    } else {
        row().push(
            column()
                .push(view_search_title(number_of_results, &search_query))
                .push(view_navigation(app))
                .push(
                    text("Sorry, no results found")
                        .horizontal_alignment(iced::alignment::Horizontal::Center)
                        .width(Length::Fill)
                        .size(20),
                )
                .spacing(10),
        )
    }
}

fn view_tweet<'a>(tweet_data: &TweetData) -> Button<'a, Message> {
    button(
        column()
            .push(
                row()
                    .push(view_tweet_author_name(&tweet_data.user))
                    .push(view_tweet_datetime(&tweet_data.tweet))
                    .spacing(30),
            )
            .push(row().push(text(&tweet_data.tweet.text)))
            .spacing(10),
    )
    .style(style::Tweet)
    .width(Length::Fill)
    .padding(20)
    .on_press(Message::DisplayConversation(tweet_data.clone()))
}

fn view_tweet_author_name<'a>(user: &UserData) -> Text {
    text(format!("{} (@{})", user.name, user.twitter_handle)).size(15)
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

fn view_tweets<'a>(tweets: &Vec<TweetData>) -> Column<'a, Message> {
    let tweet_view_list: Vec<Button<'a, Message>> =
        tweets.iter().map(|tweet| view_tweet(tweet)).collect();
    tweet_view_list
        .into_iter()
        .fold(column(), |tweets_view, tweet_view| {
            tweets_view.push(tweet_view)
        })
        .spacing(15)
}

fn view_tweets_paginated<'a>(
    tweets: &Vec<TweetData>,
    tweets_per_page: usize,
) -> Column<'a, Message> {
    let mut tweet_view_list: Vec<Button<'a, Message>> = tweets
        .iter()
        .map(|tweet_data| view_tweet(tweet_data))
        .collect();
    tweet_view_list.truncate(tweets_per_page);
    tweet_view_list
        .into_iter()
        .fold(column(), |tweets_view, tweet_view| {
            tweets_view.push(tweet_view)
        })
        .push(
            row()
                .push(
                    column()
                        .push(
                            button(
                                text("+")
                                    .size(25)
                                    .horizontal_alignment(iced::alignment::Horizontal::Center)
                                    .vertical_alignment(iced::alignment::Vertical::Center),
                            )
                            .style(style::MoreTweetsButton)
                            .width(Length::Units(200))
                            .height(Length::Units(50))
                            .padding(10)
                            .on_press(Message::ViewMoreTweets),
                        )
                        .width(Length::FillPortion(100))
                        .align_items(iced::Alignment::Center),
                )
                .align_items(iced::Alignment::Center),
        )
        .spacing(15)
}

fn view_user_tweets<'a>(tweets: &Vec<TweetData>, tweets_per_page: usize) -> Column<'a, Message> {
    column()
        .push(row().push(view_tweets_paginated(tweets, tweets_per_page)))
        .spacing(25)
}

fn view_user_timeline_title(user: &User) -> Text {
    text(format!("{}'s (@{}) Tweets", user.name, user.username))
        .size(30)
        .width(Length::Fill)
        .horizontal_alignment(iced::alignment::Horizontal::Center)
}

fn view_conversation<'a>(conversation: &Vec<TweetData>) -> Column<'a, Message> {
    let mut display_conversation = conversation.clone();
    display_conversation.reverse();
    column()
        .push(view_tweets(&display_conversation))
        .spacing(25)
}

fn view_conversation_title(tweet_data: &TweetData) -> Text {
    text(format!(
        "Conversation containing @{}'s tweet posted on {}",
        tweet_data.user.twitter_handle,
        get_tweet_created_datetime_string(&tweet_data.tweet)
    ))
    .size(30)
    .width(Length::Fill)
    .horizontal_alignment(iced::alignment::Horizontal::Center)
}

fn view_tweet_title(tweet_data: &TweetData) -> Text {
    text(format!(
        "Tweet by @{} posted at {}",
        tweet_data.user.twitter_handle,
        get_tweet_created_datetime_string(&tweet_data.tweet)
    ))
    .size(30)
    .width(Length::Fill)
    .horizontal_alignment(iced::alignment::Horizontal::Center)
}

fn view_navigation<'a>(app: &App) -> Row<'a, Message> {
    let is_back_button_active: bool = app.model.previous.len() > 0;
    let is_forward_button_active: bool = app.model.next.len() > 0;
    let is_home_button_active: bool =
        app.model.previous.len() > 0 && app.model.previous[0] != app.model.selected;
    row()
        .push(
            column().push(
                row()
                    .push(view_navigation_button(
                        "Back",
                        Message::Back,
                        is_back_button_active,
                    ))
                    .push(view_navigation_button(
                        "Forward",
                        Message::Forward,
                        is_forward_button_active,
                    ))
                    .push(view_navigation_button(
                        "Home",
                        Message::Home,
                        is_home_button_active,
                    ))
                    .push(view_navigation_button(
                        "Seed",
                        Message::SeedConversations,
                        true,
                    ))
                    .spacing(20),
            ),
        )
        .push(column().width(Length::Fill))
        .push(
            column()
                .push(row().push(view_search(&app.search_input)))
                .align_items(iced::Alignment::End),
        )
        .spacing(100)
}

fn view_search<'a>(search_input: &String) -> TextInput<'a, Message> {
    text_input(
        "type to search then press enter",
        search_input,
        Message::SearchInputChanged,
    )
    .size(15)
    .width(Length::Units(200))
    .padding(8)
    .style(style::SearchBar)
    .on_submit(Message::Search(search_input.to_string()))
}

fn view_search_title(number_of_results: &usize, search_query: &str) -> Text {
    text(format!(
        "{} results for search: \"{}\"",
        number_of_results, search_query
    ))
    .size(30)
    .width(Length::Fill)
    .horizontal_alignment(iced::alignment::Horizontal::Center)
}

fn view_navigation_button<'a>(
    label: &str,
    action: Message,
    is_active: bool,
) -> Button<'a, Message> {
    if is_active {
        button(text(label).size(15))
            .on_press(action)
            .padding(8)
            .style(style::NavButton)
    } else {
        button(text(label).size(15))
            .padding(8)
            .style(style::NavButton)
    }
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
