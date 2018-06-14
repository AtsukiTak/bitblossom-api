// #[derive(Queryable)]
pub struct InstaPost {
    pub id: u32,
    pub post_id: String,
    pub user_name: String,
    pub image_url: String,
    pub hashtag: String,
}

// #[derive(Insertable)]
// #[table_name="insta_posts"]
pub struct NewInstaPost<'a> {
    pub post_id: &'a str,
    pub user_name: &'a str,
    pub image_url: &'a str,
    pub hashtag: &'a str,
}
