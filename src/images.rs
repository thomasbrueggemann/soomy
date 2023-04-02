use std::collections::HashMap;

pub struct Image {
    url: String,
}

pub struct Thumbnail {
    pub category: String,
    pub image_url: String,
}

fn get_images() -> HashMap<String, Image> {
    HashMap::from([
        (
            "locomotive".to_string(),
            Image {
                url: "https://images.pexels.com/photos/2668755/pexels-photo-2668755.jpeg"
                    .to_string(),
            },
        ),
        (
            "airplane".to_string(),
            Image {
                url: "https://images.pexels.com/photos/358220/pexels-photo-358220.jpeg".to_string(),
            },
        ),
        (
            "excavator".to_string(),
            Image {
                url: "https://images.pexels.com/photos/6479995/pexels-photo-6479995.jpeg"
                    .to_string(),
            },
        ),
    ])
}

pub fn get_image_url(image: String) -> String {
    let images = get_images();
    let image = images.get(&image);

    match image {
        Some(i) => i.url.to_owned(),
        None => "https://images.pexels.com/photos/1806900/pexels-photo-1806900.jpeg".to_string(),
    }
}

pub fn get_all_image_urls() -> Vec<Thumbnail> {
    let images = get_images();

    let thumbnails: Vec<Thumbnail> = images
        .iter()
        .map(|(k, v)| Thumbnail {
            category: k.to_string(),
            image_url: v.url.to_owned(),
        })
        .collect();

    thumbnails
}
