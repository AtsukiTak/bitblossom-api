use std::{str::FromStr, time::{SystemTime, UNIX_EPOCH}};
use mongodb::{Client, ThreadedClient, coll::{Collection, options::FindOptions},
              db::ThreadedDatabase};
use bson::{Bson, Document, spec::BinarySubtype};
use hyper::Uri;

use images::{FetchedImage, Image, Size};
use insta::{InstaPost, InstaPostId};

pub struct Mongodb {
    insta_post: Collection,
}

impl Mongodb {
    pub fn new(host: &str, port: u16, db: &str) -> Mongodb {
        debug!(
            "Create new mongodb client with host({}), port({}), db({})",
            host, port, db
        );
        let client = Client::connect(host, port).expect("Fail to create mongodb client");
        let db = client.db(db);
        Mongodb {
            insta_post: db.collection("insta_post"),
        }
    }

    pub fn insert_one_post<S: Size>(&self, post: &InstaPost<S>) {
        debug!("Insert new insta post into mongodb");
        let doc = doc! {
            "id": post.get_id_str(),
            "username": post.get_username(),
            "image": {
                "url": post.get_image_source_str(),
                "binary": (BinarySubtype::Generic, post.get_image().to_png_bytes()),
            },
            "hashtag": post.get_hashtag(),
            "inserted_time": SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        };
        self.insta_post
            .insert_one(doc, None)
            .expect("Should delegate this error");
    }

    pub fn find_posts_by_hashtags<S: Size>(
        &self,
        hashtags: &[String],
        limit: i64,
    ) -> Vec<InstaPost<S>> {
        debug!("Find posts by hashtags : {:?}", hashtags);
        let hashtags_filter: Vec<Bson> = hashtags
            .iter()
            .map(|h| bson!(doc!{ "hashtag": h }))
            .collect();
        let filter = doc! {
            "$or": hashtags_filter,
        };
        let option = {
            let mut op = FindOptions::new();
            op.limit = Some(limit);
            op.sort = Some(doc!{"inserted_time": -1});
            op
        };
        self.insta_post
            .find(Some(filter), Some(option))
            .expect("Fail to execute find operation")
            .map(|res| doc_2_post(res.expect("Invalid document")))
            .collect()
    }
}

fn doc_2_post<S: Size>(doc: Document) -> InstaPost<S> {
    let image = {
        let url_str = doc.get_document("image").unwrap().get_str("url").unwrap();
        let binary = doc.get_document("image")
            .unwrap()
            .get_binary_generic("binary")
            .unwrap();
        let img = ::image::load_from_memory(binary.as_slice()).unwrap();
        FetchedImage::new(img, Uri::from_str(url_str).unwrap())
    };
    let id = InstaPostId(doc.get_str("id").unwrap().into());
    let username = doc.get_str("username").unwrap().into();
    let hashtag = doc.get_str("hashtag").unwrap().into();
    InstaPost::new(id, username, image, hashtag)
}
