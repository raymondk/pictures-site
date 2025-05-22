use candid::{CandidType, Deserialize};
use serde::Serialize;
use std::collections::HashMap;

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
