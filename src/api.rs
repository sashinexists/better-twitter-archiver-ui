use reqwest;

use ron;
pub mod io;
use twitter_v2::{Tweet, User};
const API: &str = API_DEV;
//const API_PROD: &str = "https://better-twitter-archiver.onrender.com/";
const API_DEV: &str = "http://127.0.0.1:8000/";
pub fn get_tweets_from_twitter_handle(twitter_handle: &str) -> Vec<Tweet> {
    match io::read::user_tweets_string_from_ron(twitter_handle) {
        Ok(tweets) => {
            println!(
                "Loading @{twitter_handle}'s tweets from \"data/user-tweets_{twitter_handle}.ron\""
            );
            ron::from_str(&tweets)
                .expect("Failed to load tweets from \"data/user-tweets_{twitter_handle}\"")
        }
        Err(_error) => {
            let resp = reqwest::blocking::get(format!("{API}user/{twitter_handle}/tweets"))
                .expect(&format!("failed to get @{twitter_handle}'s tweets"));
            let tweet_ron_str = resp.text().expect(&format!(
                "failed to parse text from @{twitter_handle}'s tweets"
            ));
            let mut tweets: Vec<Tweet> = ron::from_str(&tweet_ron_str).expect(&format!(
                "Failed to parse @{twitter_handle}'s tweets data into Rusty Object Notation"
            ));
            io::write::user_tweets_to_ron(&tweets, twitter_handle);
            io::write::tweets_to_ron(&mut tweets);
            tweets
        }
    }
}

pub fn get_conversation_by_tweet_id(id: u64) -> Vec<Tweet> {
    match io::read::conversations_string_from_ron() {
        Ok(conversations_ron) => {
            let conversations: Vec<Vec<Tweet>> = ron::from_str(&conversations_ron)
                .expect("Failed to parse conversations from \"data/conversations.ron\"");
            if conversations
                .iter()
                .any(|conversation| conversation[conversation.len() - 1].id.as_u64() == id)
            {
                println!("Loading conversation {id} from \"data/conversations.ron\"");
                conversations
                    .into_iter()
                    .filter(|conversation| conversation[conversation.len() - 1].id.as_u64() == id)
                    .next()
                    .expect("Failed to get conversation")
            } else {
                let resp = reqwest::blocking::get(format!("{API}conversation/{id}")).expect(
                    &format!("failed to get conversation ending in tweet of id {id}"),
                );
                let conversation_ron_str = resp.text().expect(&format!(
                    "failed to parse text for conversation ending in tweet of id @{id}'s tweets"
                ));
                ron::from_str(&conversation_ron_str)
                    .expect(&format!("Failed to parse conversation ending in tweet of id {id}'s tweets data into Rusty Object Notation"))
            }
        }
        Err(_error) => {
            let resp = reqwest::blocking::get(format!("{API}conversation/{id}")).expect(&format!(
                "failed to get conversation ending in tweet of id {id}"
            ));
            let conversation_ron_str = resp.text().expect(&format!(
                "failed to parse text for conversation ending in tweet of id @{id}'s tweets"
            ));

            ron::from_str(&conversation_ron_str)
                .expect(&format!("Failed to parse conversation ending in tweet of id {id}'s tweets data into Rusty Object Notation"))
        }
    }
}

pub fn get_tweet_by_id(id: u64) -> Tweet {
    let resp = reqwest::blocking::get(format!("{API}tweet/{id}"))
        .expect(&format!("Failed to fetch tweet of id {id}"));
    let tweet_ron_str = resp
        .text()
        .expect(&format!("Failed to parse tweet of id {id} into text"));
    ron::from_str(&tweet_ron_str).expect(&format!(
        "Failed to parse tweet data for tweet of id {id} into Rusty Object Notation"
    ))
}

pub fn get_user_by_id(id: u64) -> User {
    let resp = reqwest::blocking::get(format!("{API}userid/{id}"))
        .expect(&format!("failed to get user from id {id}"));
    let user_ron_str = resp.text().expect(&format!(
        "failed to parse text from response to request for user of id {id}'s data"
    ));
    ron::from_str(&user_ron_str).expect(&format!(
        "Failed to parse user of id {id}'s data into Rusty Object Notation",
    ))
}

pub fn get_user_by_twitter_handle(twitter_handle: &str) -> User {
    let resp = reqwest::blocking::get(format!("{API}user/{twitter_handle}/info")).expect(&format!(
        "failed to get user from twitter handle @{twitter_handle}"
    ));
    let user_ron_str = resp
        .text()
        .expect("failed to parse text from response for user @{twitter_handle}");
    ron::from_str(&user_ron_str).expect(&format!(
        "Failed to parse @{twitter_handle} user data into Rusty Object Notation"
    ))
}
