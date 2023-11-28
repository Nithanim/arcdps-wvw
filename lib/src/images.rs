pub struct ImageData {
    pub format: ImageFormat,
    pub size: Size,
    pub bytes: Vec<u8>,
}

pub struct Size {
    pub w: u32,
    pub h: u32,
}

pub enum ImageFormat {
    RGBA8
}

pub fn load_image_file(file: &[u8]) -> ImageData {
    let img = image::load_from_memory(file).unwrap();
    ImageData {
        format: ImageFormat::RGBA8,
        size: Size {
            w: img.width(),
            h: img.height(),
        },
        bytes: img.to_rgba8().into_raw(),
    }
}


