CREATE TABLE insta_posts (
    id SERIAL PRIMARY KEY,
    post_id VARCHAR NOT NULL,
    user_name VARCHAR NOT NULL,
    image_url VARCHAR NOT NULL,
    hashtag VARCHAR NOT NULL,
)
