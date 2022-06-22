use ron::ser::PrettyConfig;
use std::fs::{self};
use twitter_v2::{Tweet, User};

pub fn conversation_to_ron(conversation: &mut Vec<Tweet>) {
    match fs::read_to_string("data/conversations.ron") {
        Ok(conversations_ron) => {
            let mut conversations: Vec<Vec<Tweet>> = ron::from_str(&conversations_ron)
                .expect("Failed to parse conversations from \"conversations.ron\"");
            if !conversations.iter().any(|conversation_from_ron| {
                conversation_from_ron[conversation_from_ron.len() - 1].id
                    == conversation[conversation.len() - 1].id
            }) {
                println!("Writing conversation to \"data/conversations.ron\"");
                conversations.push(conversation.clone());
                fs::write(
                    "data/conversations.ron",
                    ron::ser::to_string_pretty(&conversations, ron::ser::PrettyConfig::new())
                        .expect("Failed to parse conversations into ron string"),
                )
                .expect("Failed to write to \"data/conversations.ron\"");
            }
        }
        Err(_error) => {
            println!("Creating file \"data/conversations.ron\"");
            let mut conversations = Vec::<Vec<Tweet>>::new();
            conversations.push(conversation.clone());
            fs::write(
                "data/conversations.ron",
                ron::ser::to_string_pretty(&conversations, ron::ser::PrettyConfig::new())
                    .expect("Failed to parse conversations into ron string"),
            )
            .expect("Failed to write to \"data/conversations.ron\"");
        }
    }
}

pub fn tweets_to_ron(tweets: &mut Vec<Tweet>) {
    match fs::read_to_string("data/tweets.ron") {
        Ok(tweets_from_ron_string) => {
            let mut tweets_from_ron: Vec<Tweet> = ron::from_str(&tweets_from_ron_string)
                .expect("Failed to read tweets from \"data/tweets.ron\"");
            tweets.into_iter().for_each(|tweet| {
                if !tweets_from_ron
                    .iter()
                    .any(|tweet_from_ron| tweet_from_ron.id == tweet.id)
                {
                    tweets_from_ron.push(tweet.clone())
                }
            });
            println!("Writing tweets to \"data/tweets.ron\"");
            fs::write(
                "data/tweets.ron",
                ron::ser::to_string_pretty(&tweets_from_ron, ron::ser::PrettyConfig::new())
                    .expect("Failed to parse tweets_from_ron back into a string"),
            )
            .expect("Failed to write to \"data/tweets.ron\"");
        }
        Err(_error) => {
            println!("Creating new file \"data/tweets.ron\"");
            fs::write(
                "data/tweets.ron",
                ron::ser::to_string_pretty(&tweets, ron::ser::PrettyConfig::new())
                    .expect("Failed to parse tweets into a ron string"),
            )
            .expect("Failed to create a new \"data/tweets.ron\"")
        }
    }
}

pub fn user_info_to_ron(user: &User, twitter_handle: &str) {
    user_to_users_ron(user);
    println!("Creating new file \"data/user-info_{twitter_handle}.ron\"");
    fs::write(
        &format!("data/user-info_{twitter_handle}.ron"),
        ron::ser::to_string_pretty(user, PrettyConfig::new()).expect(&format!(
            "Failed to parse user @{twitter_handle} into a ron pretty string"
        )),
    )
    .expect(&format!(
        "Failed to write info for @{twitter_handle} to \"data/user-info_{twitter_handle}.ron"
    ));
}

pub fn user_tweets_to_ron(tweets: &Vec<Tweet>, twitter_handle: &str) {
    println!("Creating new file \"data/user-tweets_{twitter_handle}.ron\"");
    fs::write(
        &format!("data/user-tweets_{twitter_handle}.ron"),
        ron::ser::to_string_pretty(tweets, PrettyConfig::new()).expect(&format!(
            "Failed to parse user @{twitter_handle}'s tweets into a ron pretty string"
        )),
    )
    .expect(&format!(
        "Failed to write tweets for @{twitter_handle} to \"data/user-tweets_{twitter_handle}.ron"
    ));
}

pub fn user_conversations_to_ron(conversations: &Vec<Vec<Tweet>>, twitter_handle: &str) {
    println!("Creating new file \"data/user-conversations_{twitter_handle}.ron\"");
    fs::write(
        &format!("data/user-conversations_{twitter_handle}.ron"),
        ron::ser::to_string_pretty(conversations, PrettyConfig::new()).expect(&format!(
            "Failed to parse user @{twitter_handle}'s conversations into a ron pretty string"
        )),
    )
    .expect(&format!(
        "Failed to write conversations for @{twitter_handle} to \"data/user-conversations_{twitter_handle}.ron"
    ));
}

pub fn user_to_users_ron(user: &User) {
    match fs::read_to_string("data/users.ron") {
        Ok(users_from_ron_string) => {
            let mut users_from_ron: Vec<User> = ron::from_str(&users_from_ron_string)
                .expect("Failed to read users from \"data/users.ron\"");
            if !users_from_ron
                .iter()
                .any(|user_from_ron| user.id == user_from_ron.id)
            {
                users_from_ron.push(user.clone())
            }
            let username = user.username.clone();
            println!("Writing @{username} to \"data/users.ron\"");
            fs::write(
                "data/users.ron",
                ron::ser::to_string_pretty(&users_from_ron, ron::ser::PrettyConfig::new())
                    .expect("Failed to parse users_from_ron back into a string"),
            )
            .expect("Failed to write to \"data/users.ron\"");
        }
        Err(_error) => {
            println!("Creating new file \"data/users.ron\"");
            let users = vec![user];
            fs::write(
                "data/users.ron",
                ron::ser::to_string_pretty(&users, ron::ser::PrettyConfig::new())
                    .expect("Failed to parse user into a ron string"),
            )
            .expect("Failed to create a new \"data/users.ron\"")
        }
    }
}
