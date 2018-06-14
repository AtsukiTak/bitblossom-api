use diesel::{pg::PgConnection, prelude::*};

use super::schema::insta_posts;

pub fn establish_connection(db_url: &str) -> PgConnection {
    PgConnection::establish(db_url).unwrap()
}

pub fn insert_new_post(db: &PgConnection, new_post: NewInstaPost) {
    diesel::insert_into(insta_posts::table)
        .values(&new_post)
        .execute(db)
        .expect("Error saving new post")
}

pub fn query_post_by_hashtag(db: &PgConnection, hashtag: &str) -> Vec<InstaPost> {
    insta_posts
        .table
        .filter(insta_posts::hashtag.eq(hashtag))
        .load(db)
        .expect("Error reading posts")
}
