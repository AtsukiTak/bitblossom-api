use std::{str::FromStr, time::{SystemTime, UNIX_EPOCH}};
use mongodb::{Client, ThreadedClient, coll::{Collection, options::FindOptions},
              db::ThreadedDatabase};
use bson::{Document, spec::BinarySubtype, Bson};
use hyper::Uri;

use images::{FetchedImage, Image, Size};
use insta::{InstaPost, InstaPostId};

pub struct MongodbInstaPost {
    coll: Collection,
}

impl MongodbInstaPost {
    pub fn new(host: &str, port: u16, db: &str) -> MongodbInstaPost {
        let client = Client::connect(host, port).expect("Fail to connect mongodb");
        let db = client.db(db);
        MongodbInstaPost {
            coll: db.collection("INSTA_POST"),
        }
    }

    pub fn insert_one<S: Size>(&self, post: &InstaPost<S>) {
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
        self.coll
            .insert_one(doc, None)
            .expect("Should delegate this error");
    }

    pub fn find_by_hashtags<S: Size>(&self, hashtags: &[String], limit: i64) -> Vec<InstaPost<S>> {
        let hashtags_filter: Vec<Bson> = hashtags.iter().map(|h| bson!(doc!{ "hashtag": h })).collect();
        let filter = doc! {
            "$or": hashtags_filter,
        };
        debug!("Search mongodb with filter : {:?}", filter);
        let option = {
            let mut op = FindOptions::new();
            op.limit = Some(limit);
            op.sort = Some(doc!{"inserted_time": -1});
            op
        };
        self.coll
            .find(Some(filter), Some(option))
            .expect("Fail to find collection")
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
