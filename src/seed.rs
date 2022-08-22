use super::app;
use ron;
use std::fs;

use sea_orm::DatabaseConnection;
pub async fn all_tweets(db: &DatabaseConnection, user_id: i64) {
    let skip = 0;
    let id_vec_ron =
        fs::read_to_string("yudapearl_tweet_id_vec.ron").expect("Failed to read ron file");
    let id_vec: Vec<i64> = ron::from_str(&id_vec_ron).expect("Failed to parse ids from ron");
    for (i, id) in id_vec.into_iter().enumerate().skip(skip) {
        println!("{i} Loading tweet {id}");
        app::load_tweet_from_id(db, id, user_id).await;
        println!("{i} Loaded tweet {id}");
    }
}
