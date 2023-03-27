use std::collections::HashMap;

pub struct Image {
    url: String,
}

fn get_images() -> HashMap<String, Image> {
    HashMap::from([(
        "locomotive".to_string(),
        Image {
            url: "https://images.pexels.com/photos/2668755/pexels-photo-2668755.jpeg".to_string(),
        },
    )])
}

pub fn get_image_url(image: String) -> String {
    let images = get_images();
    let image = images.get(&image);

    match image {
        Some(i) => i.url.to_owned(),
        None => "https://images.pexels.com/photos/1806900/pexels-photo-1806900.jpeg".to_string(),
    }
}
