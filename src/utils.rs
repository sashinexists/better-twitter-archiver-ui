use chrono::{DateTime, FixedOffset};
use serde::Serialize;
use time::{format_description, OffsetDateTime};
use twitter_v2::{Tweet, User};

use twitter_v2::data::{ReferencedTweet, ReferencedTweetKind};
#[derive(Debug, Clone)]
pub struct SelectList<T> {
    pub previous: Vec<T>,
    pub selected: T,
    pub next: Vec<T>,
}

impl<T: Clone> SelectList<T> {
    pub fn previous(&mut self) {
        if let Some(mut new) = self.previous.pop() {
            std::mem::swap(&mut self.selected, &mut new);
            self.next.insert(0, new);
        }
    }

    pub fn forward(&mut self) {
        if !self.next.is_empty() {
            let mut new = self.next.remove(0);
            std::mem::swap(&mut self.selected, &mut new);
            self.previous.push(new);
        }
    }

    pub fn add(&mut self, item: T) {
        self.next.clear();
        self.previous.push(self.selected.clone());
        self.selected = item;
    }

    pub fn new(item: T) -> SelectList<T> {
        SelectList {
            previous: Vec::<T>::new(),
            selected: item,
            next: Vec::<T>::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TweetData {
    pub tweet: Tweet,
    pub user: UserData,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UserData {
    pub twitter_handle: String,
    pub name: String,
}

impl TweetData {
    pub fn new(user: &User, tweet: Tweet) -> Self {
        Self {
            tweet: tweet,
            user: UserData {
                twitter_handle: user.username.clone(),
                name: user.name.clone(),
            },
        }
    }

    pub fn from_vec_tweet(user: &User, tweets: Vec<Tweet>) -> Vec<Self> {
        tweets
            .into_iter()
            .map(|tweet| Self::new(user, tweet))
            .collect()
    }
}

pub fn convert_date_to_chrono(date: Option<OffsetDateTime>) -> DateTime<FixedOffset> {
    let format = format_description::parse(
        "[year]-[month]-[day]T[hour]:[minute]:[second][offset_hour \
             sign:mandatory]:[offset_minute]",
    )
    .expect("Bad formatter");

    let date_string = date
        .expect("Couldn't get the tweets date")
        .format(&format)
        .expect("Couldn't parse with thes formatter");

    chrono::DateTime::<chrono::FixedOffset>::parse_from_rfc3339(&date_string)
        .expect("failed to parse date from string")
}

pub fn to_ron<T: ?Sized + Serialize>(item: &T) -> String {
    ron::ser::to_string_pretty(item, ron::ser::PrettyConfig::new())
        .expect("Failed to parse tweet into string")
}
#[derive(Debug, Serialize)]
pub struct TweetReferenceData {
    pub reference_type: ReferencedTweetKind,
    pub source_tweet_id: i64,
    pub reference_tweet_id: i64,
}

impl TweetReferenceData {
    pub fn type_to_string(&self) -> String {
        match self.reference_type {
            ReferencedTweetKind::RepliedTo => "replied_to",
            ReferencedTweetKind::Retweeted => "retweeted",
            ReferencedTweetKind::Quoted => "quoted",
        }
        .to_string()
    }

    pub fn kind_from_string(input: &str) -> Option<ReferencedTweetKind> {
        match input {
            "replied_to" => Some(ReferencedTweetKind::RepliedTo),
            "retweeted" => Some(ReferencedTweetKind::Retweeted),
            "quoted" => Some(ReferencedTweetKind::Quoted),
            _ => None,
        }
    }

    pub fn from_referenced_tweet(id: i64, referenced_tweet: &ReferencedTweet) -> Self {
        Self {
            reference_type: referenced_tweet.kind.clone(),
            source_tweet_id: id.clone(),
            reference_tweet_id: referenced_tweet
                .id
                .as_u64()
                .try_into()
                .expect("Bad referenced tweet id"),
        }
    }

    pub fn clone(&self) -> Self {
        Self {
            reference_type: self.reference_type.clone(),
            source_tweet_id: self.source_tweet_id.clone(),
            reference_tweet_id: self.reference_tweet_id.clone(),
        }
    }
}
