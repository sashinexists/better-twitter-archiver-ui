use crate::utils::TweetData;
use futures::{executor::block_on, future::join_all, stream::FuturesUnordered, StreamExt};
use sea_orm::DatabaseConnection;

use twitter_v2::{Tweet, User};

pub mod data;
pub mod server;

pub async fn load_users_tweets_from_twitter_handle(
    db: &DatabaseConnection,
    twitter_handle: &str,
) -> Vec<TweetData> {
    let user = load_user_from_twitter_handle(db, twitter_handle).await;
    let user_tweets = data::read::users_tweets(db, twitter_handle).await;
    //need something better
    if user_tweets.len() == 0 {
        let user_tweets = server::get_tweets_from_twitter_handle(twitter_handle);
        data::write::tweets(db, &user_tweets).await;
        TweetData::from_vec_tweet(&user, user_tweets)
    } else {
        if has_new_tweets(db, twitter_handle).await {
            println!("Adding new tweets");
            let new_tweets = load_users_new_tweets(db, twitter_handle).await;
            data::write::tweets(db, &new_tweets).await;
            TweetData::from_vec_tweet(&user, data::read::users_tweets(db, twitter_handle).await)
        } else {
            println!("No new tweets to add");
            TweetData::from_vec_tweet(&user, user_tweets)
        }
    }
}

pub async fn load_user_from_id(db: &DatabaseConnection, id: i64) -> User {
    match data::read::user_by_id(db, id).await {
        Some(user) => user,
        None => {
            let user = server::get_user_by_id(id.try_into().expect("Failed to parse i64 from u64"));
            data::write::user(db, &user).await;
            user
        }
    }
}

pub async fn load_user_from_twitter_handle(db: &DatabaseConnection, twitter_handle: &str) -> User {
    match data::read::user_by_twitter_handle(db, twitter_handle).await {
        Some(user) => user,
        None => {
            let user = server::get_user_by_twitter_handle(twitter_handle);
            data::write::user(db, &user).await;
            user
        }
    }
}

pub async fn load_conversation_from_tweet_id(
    db: &DatabaseConnection,
    tweet_id: i64,
) -> Vec<TweetData> {
    let conversation = data::read::conversation(db, tweet_id).await;
    if &conversation.len() <= &1 {
        vec_tweet_data_from_vec_tweet(db, conversation).await
    } else {
        let conversation = server::get_conversation_by_tweet_id(tweet_id);
        data::write::tweets(db, &conversation).await;
        vec_tweet_data_from_vec_tweet(db, conversation).await
    }
}

pub async fn tweet_data_from_tweet(db: &DatabaseConnection, tweet: Tweet) -> TweetData {
    let user = load_user_from_id(
        db,
        tweet
            .author_id
            .expect("bad author id")
            .as_u64()
            .try_into()
            .expect("Failed to parse u64 into i64"),
    )
    .await;

    TweetData::new(&user, tweet)
}

pub async fn vec_tweet_data_from_vec_tweet(
    db: &DatabaseConnection,
    tweets: Vec<Tweet>,
) -> Vec<TweetData> {
    let future_tweets = join_all(
        tweets
            .into_iter()
            .map(|tweet| tweet_data_from_tweet(db, tweet)),
    );

    future_tweets.await
}

pub async fn load_tweet_from_id(db: &DatabaseConnection, tweet_id: i64, user_id: i64) -> TweetData {
    let user = load_user_from_id(db, user_id).await;
    match data::read::tweet_by_id(db, tweet_id).await {
        Some(tweet) => TweetData::new(&user, tweet),
        None => {
            let tweet = server::get_tweet_by_id(tweet_id);
            data::write::tweet(db, &tweet).await;
            TweetData::new(&user, tweet)
        }
    }
}

pub async fn has_new_tweets(db: &DatabaseConnection, twitter_handle: &str) -> bool {
    let since = data::read::latest_tweet_from_user_by_twitter_handle(db, twitter_handle)
        .await
        .expect("failed to get latest tweet")
        .created_at
        .expect("Failed to get created_at date");
    server::find_out_if_user_has_tweeted_since(twitter_handle, &since)
}

pub async fn load_users_new_tweets(db: &DatabaseConnection, twitter_handle: &str) -> Vec<Tweet> {
    let since = data::read::latest_tweet_from_user_by_twitter_handle(db, twitter_handle)
        .await
        .expect("failed to get latest tweet")
        .created_at
        .expect("Failed to get created_at date");
    server::get_users_tweets_since(twitter_handle, &since)
}
