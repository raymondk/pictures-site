use candid::{CandidType, Deserialize};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

type ImageId = u64;

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct ImageData {
    pub id: ImageId,
    pub name: String,
    pub content_type: String,
    pub data: Vec<u8>,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct ImageInfo {
    pub id: ImageId,
    pub name: String,
    pub content_type: String,
}

thread_local! {
    static IMAGES: std::cell::RefCell<HashMap<ImageId, ImageData>> = std::cell::RefCell::new(HashMap::new());
    static NEXT_ID: std::cell::RefCell<ImageId> = std::cell::RefCell::new(1);
}

#[ic_cdk::update]
fn upload_image(name: String, content_type: String, data: Vec<u8>) -> ImageId {
    let id = NEXT_ID.with(|next_id| {
        let mut next_id = next_id.borrow_mut();
        let current_id = *next_id;
        *next_id += 1;
        current_id
    });

    let image = ImageData {
        id,
        name,
        content_type,
        data,
    };

    IMAGES.with(|images| {
        images.borrow_mut().insert(id, image);
    });

    id
}

#[ic_cdk::query]
fn get_image(id: ImageId) -> Option<ImageData> {
    IMAGES.with(|images| images.borrow().get(&id).cloned())
}

#[ic_cdk::query]
fn list_images() -> Vec<ImageInfo> {
    IMAGES.with(|images| {
        images
            .borrow()
            .iter()
            .map(|(id, image)| ImageInfo {
                id: *id,
                name: image.name.clone(),
                content_type: image.content_type.clone(),
            })
            .collect()
    })
}

#[ic_cdk::query]
fn http_request(request: HttpRequest) -> HttpResponse {
    let path = request.url.trim_start_matches('/');

    // Handle image requests like /image/123.jpg or /image/123
    if let Some(image_path) = path.strip_prefix("image/") {
        // Extract image ID, ignoring any file extension
        let image_id_str = image_path.split('.').next().unwrap_or(image_path);
        if let Ok(image_id) = image_id_str.parse::<ImageId>() {
            if let Some(image) = IMAGES.with(|images| images.borrow().get(&image_id).cloned()) {
                return HttpResponse {
                    status: 200,
                    headers: vec![
                        ("Content-Type".to_string(), image.content_type.clone()),
                        (
                            "Cache-Control".to_string(),
                            "public, max-age=31536000, immutable".to_string(),
                        ),
                        ("ETag".to_string(), format!("\"{}\"", image_id)),
                    ],
                    body: image.data,
                };
            }
        }
    }

    // Return 404 for unknown paths
    HttpResponse {
        status: 404,
        headers: vec![("Content-Type".to_string(), "text/plain".to_string())],
        body: b"Not Found".to_vec(),
    }
}
