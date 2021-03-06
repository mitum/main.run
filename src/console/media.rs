use std::str::FromStr;

use sincere::app::context::Context;
use sincere::app::Group;
use sincere::http::plus::server::FilePart;
use sincere::http::plus::random_alphanumeric;

use mongors::object_id::ObjectId;
use mongors::collection::options::FindOptions;
use mongors::{doc, bson};

use chrono::{Utc};

use serde_derive::{Serialize, Deserialize};
use serde_json::json;

use qiniu::{Config, PutPolicy};

use reqwest::multipart::{Form, Part};
use reqwest::StatusCode;

use crate::HTTP_CLIENT;
use crate::error::Result;
use crate::common::{Response, Empty};
use crate::model;
use crate::struct_document::StructDocument;
use crate::error::ErrorCode;

const STATIC_DOMINE: &str = "https://cdn2.danclive.com/";

pub struct Media;

impl Media {
    hand!(medias, {|context: &mut Context| {
        let page = context.request.query("page").unwrap_or("1".to_owned());
        let per_page = context.request.query("per_page").unwrap_or("10".to_owned());

        let page = i64::from_str(&page)?;
        let per_page = i64::from_str(&per_page)?;

        let media_find = doc!{

        };

        let mut media_find_option = FindOptions::default();

        media_find_option.sort = Some(doc!{
            "_id": -1
        });

        media_find_option.limit = Some(per_page);
        media_find_option.skip = Some((page - 1) * per_page);

        let medias = model::Media::find(media_find.clone(), Some(media_find_option))?;

        let media_count = model::Media::count(media_find, None)?;

        let mut medias_json = Vec::new();

        for media in medias {
            medias_json.push(json!({
                "id": media.id.to_hex(),
                "filename": media.filename,
                "filesize": media.filesize,
                "width": media.width,
                "height": media.height,
                "url": format!("{}{}/{}{}", STATIC_DOMINE, "upload", media.hash, media.extension)
            }));
        }

        let return_json = json!({
            "medias": medias_json,
            "count": media_count
        });

        Ok(Response::success(Some(return_json)))
    }});

    hand!(detail, {|context: &mut Context| {
        let media_id = context.request.param("id").unwrap();

        let media = model::Media::find_by_id(ObjectId::with_string(&media_id)?, None, None)?;

        match media {
            None => return Err(ErrorCode(10004).into()),
            Some(doc) => {
                let return_json = json!({
                    "id": doc.id.to_hex(),
                    "filename": doc.filename,
                    "filesize": doc.filesize,
                    "width": doc.width,
                    "height": doc.height,
                    "url": format!("{}{}/{}{}", STATIC_DOMINE, "upload", doc.hash, doc.extension)
                });

                Ok(Response::success(Some(return_json)))
            }
        }
    }});

    hand!(upload, {|context: &mut Context| {

        if context.request.has_file() {
            let files = upload_file(context.request.files())?;

            let mut return_json = Vec::new();

            for file in files {
                let media = model::Media {
                    id: ObjectId::new()?,
                    filename: file.filename,
                    filesize: file.filesize,
                    mime_type: file.mime_type,
                    extension: file.extension,
                    width: file.width,
                    height: file.height,
                    hash: file.hash
                };

                media.save()?;

                return_json.push(json!({
                    "id": media.id.to_hex(),
                    "url": STATIC_DOMINE.to_owned() + &file.key
                }));
            }

            return Ok(Response::success(Some(json!(return_json))));
        }

        return Err(ErrorCode(10006).into())
    }});

    pub fn handle() -> Group {

        let mut group = Group::new("/console/media");

        group.get("/", Self::medias);
        group.get("/{id:[a-z0-9]{24}}", Self::detail);
        group.post("/", Self::upload);

        group
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    key: String,
    hash: String,
    filename: String,
    filesize: i32,
    mime_type: String,
    extension: String,
    width: i32,
    height: i32
}

fn upload_file(files: &Vec<FilePart>) -> Result<Vec<File>> {

    const ACCESS_KEY: &str = "RHigUX-2wovAQf-SSc_9U5mGw9BdKcSGImGbXXHU";
    const SECRET_KEY: &str = "Rqdb_n9sMAhIGlSGolUAgbzWT39ypgM_ulTwDY4N";

    // get timestamp
    let now = Utc::now();
    let timestamp = now.timestamp();

    // new config and put policy
    let config = Config::new(ACCESS_KEY, SECRET_KEY);
    let mut put_policy = PutPolicy::new("cdn2", (timestamp + 3600) as u32);

    // set return body
    let return_body = r#"{"key": $(key), "hash": $(etag), "filename": $(fname), "filesize": $(fsize), "mime_type": $(mimeType), "extension": $(ext), "width": $(imageInfo.width), "height": $(imageInfo.height)}"#;
    put_policy.return_body = Some(return_body.to_owned());
    put_policy.save_key = Some("upload/$(etag)$(ext)".to_owned());
    put_policy.mime_limit = Some("image/*".to_owned());

    // generate upload token
    let token = put_policy.generate_uptoken(&config);

    let mut uploads = Vec::new();

    for file_part in files {

        let reader = ::std::io::Cursor::new(file_part.data.clone());
        let filename = file_part.filename.clone().unwrap_or(random_alphanumeric(32));
        let part = Part::reader_with_length(reader, file_part.data.len() as u64).mime(file_part.content_type.clone()).file_name(filename);

        // new form and set file
        let form = Form::new().text("token", token.clone()).part("file", part);
        // send file
        let mut response = HTTP_CLIENT.post("http://upload.qiniup.com/").multipart(form).send()?;

        if response.status() != StatusCode::Ok {
            //return Err(ErrorCode(10007).into())
            continue
        }

        let file: File = response.json()?;

        uploads.push(file);
    }

    Ok(uploads)
}
