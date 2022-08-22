use chrono::DateTime;
use reqwest;
use ron;
use std::process::Command;
use time::OffsetDateTime;
use twitter_v2::{Tweet, User};

use crate::utils;
const API_PROD: &str = "https://judea-pearl-tweets-archive.onrender.com/";
const API: &str = API_PROD;

//const API_PROD: &str = "https://better-twitter-archiver.onrender.com/";
//const API_DEV: &str = "http://127.0.0.1:8000/";
pub fn get_tweets_from_twitter_handle(twitter_handle: &str) -> Vec<Tweet> {
    let resp = reqwest::blocking::get(format!("{API}user/{twitter_handle}/tweets"))
        .expect(&format!("failed to get @{twitter_handle}'s tweets"));
    let tweet_ron_str = resp.text().expect(&format!(
        "failed to parse text from @{twitter_handle}'s tweets"
    ));
    let tweets: Vec<Tweet> = ron::from_str(&tweet_ron_str).expect(&format!(
        "Failed to parse @{twitter_handle}'s tweets data into Rusty Object Notation"
    ));
    tweets
}

pub fn get_conversation_by_tweet_id(id: i64) -> Vec<Tweet> {
    let resp = reqwest::blocking::get(format!("{API}conversation/{id}")).expect(&format!(
        "failed to get conversation ending in tweet of id {id}"
    ));
    let conversation_ron_str = resp.text().expect(&format!(
        "failed to parse text for conversation ending in tweet of id @{id}'s tweets"
    ));

    ron::from_str(&conversation_ron_str)
                .expect(&format!("Failed to parse conversation ending in tweet of id {id}'s tweets data into Rusty Object Notation"))
}

pub fn get_tweet_by_id(id: i64) -> Option<Tweet> {
    let request = reqwest::blocking::get(format!("{API}tweet/{id}"))
        .unwrap_or_else(|error| panic!("Failed to get request from server. Error {:?}", error));
    let response = request
        .text()
        .unwrap_or_else(|error| panic!("Couldn't get server text. Error: {:?}", error));
    ron::from_str(&response).unwrap_or_else(|error| {
        println!("Failed to get tweet from server, waiting 15 minutes before trying again. Error: {:?}", error);
        Command::new("sleep")
            .arg("910")
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
        let request = reqwest::blocking::get(format!("{API}tweet/{id}"))
            .unwrap_or_else(|error| panic!("(Second attempt) Failed to get request from server. Error {:?}", error));
        let response = request
            .text()
            .unwrap_or_else(|error| panic!("(Second attempt) Couldn't get server text. Error: {:?}", error));
        ron::from_str(&response).unwrap_or_else(|error| {
            panic!("Second Attempt: Failed to get tweet from server even after waiting fifteen minutes. Error: {:?}", error);
        })
    })
    // match reqwest::blocking::get(format!("{API}tweet/{id}")) {
    //     Ok(server_response) => match server_response.text() {
    //         Ok(text_from_server) => match ron::from_str(&text_from_server) {
    //             Ok(option_tweet) => option_tweet,
    //             Err(e) => {
    //                 println!(
    //                     "Failed to get tweet from server, waiting 15 minutes before trying again"
    //                 );
    //                 Command::new("sleep")
    //                     .arg("910")
    //                     .spawn()
    //                     .unwrap()
    //                     .wait()
    //                     .unwrap();
    //                 match reqwest::blocking::get(format!("{API}tweet/{id}")) {
    //                     Ok(server_response) => match server_response.text() {
    //                         Ok(text_from_server) => match ron::from_str(&text_from_server) {
    //                             Ok(option_tweet) => option_tweet,
    //                             Err(e) => {
    //                                 panic!("Failed on second attempt to parse server response text into tweet")
    //                             }
    //                         },
    //                         Err(e) => {
    //                             panic!("Failed to get text from server on second try. Error {e}")
    //                         }
    //                     },
    //                     Err(e) => {
    //                         panic!("Server response failed on second try. Error {e}")
    //                     }
    //                 }
    //             }
    //         },
    //         Err(e) => {
    //             panic!("Couldn't get the server text. Error: {e}")
    //         }
    //     },
    //     Err(e) => {
    //         panic!("Server response failed. Error: {e}")
    //     }
    // }
}

pub fn get_user_by_id(id: u64) -> User {
    let resp = reqwest::blocking::get(format!("{API}userbyid/{id}"))
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

pub fn find_out_if_user_has_tweeted_since(twitter_handle: &str, since: &OffsetDateTime) -> bool {
    let since = utils::convert_date_to_chrono(Some(since.clone()));
    let rfc3339_datestring = since.to_rfc3339();
    let resp = reqwest::blocking::get(format!(
        "{API}user/{twitter_handle}/has_tweeted_since/{rfc3339_datestring}"
    ))
    .expect(&format!(
        "failed to get user from twitter handle @{twitter_handle}"
    ));
    let user_ron_str = resp
        .text()
        .expect("failed to parse text from response for user @{twitter_handle}");
    ron::from_str(&user_ron_str).expect(&format!(
        "Failed to parse @{twitter_handle} user data into Rusty Object Notation"
    ))
}

pub fn get_users_tweets_since(twitter_handle: &str, since: &OffsetDateTime) -> Vec<Tweet> {
    let since = utils::convert_date_to_chrono(Some(since.clone()));
    let rfc3339_datestring = since.to_rfc3339();

    let resp = reqwest::blocking::get(format!(
        "{API}user/{twitter_handle}/tweets-since/{rfc3339_datestring}"
    ))
    .expect(&format!(
        "failed to get user from twitter handle @{twitter_handle}"
    ));
    let user_ron_str = resp
        .text()
        .expect("failed to parse text from response for user @{twitter_handle}");
    ron::from_str(&user_ron_str).expect(&format!(
        "Failed to parse @{twitter_handle} user data into Rusty Object Notation"
    ))
}
