use rand::Rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Image {
    id: i32,
    pid: Uuid,
    name: String,
    alt: String,
    url: String,
    content_type: String,
    pub is_favorite: bool,
    pub is_deleted: bool,
}

impl Image {
    pub fn new(
        id: i32,
        name: &str,
        alt: &str,
        url: &str,
        is_favorite: bool,
        is_deleted: bool,
    ) -> Self {
        Image {
            id,
            pid: Uuid::new_v4(),
            name: name.to_string(),
            url: url.to_string(),
            alt: alt.to_string(),
            content_type: "jpg".to_string(),
            is_favorite,
            is_deleted,
        }
    }

    pub fn test(num: i32) -> Vec<Image> {
        let mut images = Vec::new();
        images.push(Image::new(
            num,
            format!("Image {}", num).as_str(),
            format!("Personal Image {}", num).as_str(),
            format!(
                "https://flowbite.s3.amazonaws.com/docs/gallery/square/image-{}.jpg",
                num
            )
            .as_str(),
            rand::rng().random_bool(0.5),
            rand::rng().random_bool(0.5),
        ));
        images
    }

    pub fn generate(num: i32) -> Vec<Image> {
        let mut images = Vec::new();
        for _ in 0..num {
            images.push(Image::new(
                num,
                format!("Image {}", num).as_str(),
                format!("Personal Image {}", num).as_str(),
                "https://picsum.photos/400/500",
                rand::rng().random_bool(0.5),
                rand::rng().random_bool(0.5),
            ));
        }
        images
    }
}

impl Image {
    pub fn get_mock_images(is_full: bool) -> Vec<Image> {
        let mut list = Vec::with_capacity(33);

        if is_full {
            for i in 0..=2 {
                for index in 1..=11 {
                    list.push(Image {
                        id: index,
                        pid: Uuid::new_v4(),
                        name: format!("Image {}", index),
                        url: format!(
                            "https://flowbite.s3.amazonaws.com/docs/gallery/square/image-{}.jpg",
                            index
                        ),
                        alt: "".to_string(),
                        content_type: "jpg".to_string(),
                        is_favorite: rand::rng().random_bool(0.5),
                        is_deleted: rand::rng().random_bool(0.5),
                    });
                }
            }
        }

        list
    }
}
