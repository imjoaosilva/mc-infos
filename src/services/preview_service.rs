use image;
use std::io::Cursor;

pub fn cape_manipulation(img: Vec<u8>) -> Vec<u8> {
    let imgdata = image::load_from_memory(&img).unwrap();
    let mut zoomed = imgdata.resize(512, 512, image::imageops::FilterType::Nearest);
    let cutted = zoomed.crop(0, 0, 95, 135);
    let mut bytes: Vec<u8> = Vec::new();
    cutted.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png).unwrap();

    return bytes;
}

pub fn head2d_manipulation(img: Vec<u8>) -> Vec<u8> {
    let imgdata = image::load_from_memory(&img).unwrap();
    let mut zoomed = imgdata.resize(512, 512, image::imageops::FilterType::Nearest);
    let cutted = zoomed.crop(65, 65, 65, 63);
    let mut bytes: Vec<u8> = Vec::new();
    cutted.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png).unwrap();

    return bytes;
}

pub fn head3d_manipulation(img: Vec<u8>) -> Vec<u8> {
    let imgdata = image::load_from_memory(&img).unwrap();
    let mut zoomed = imgdata.resize(512, 512, image::imageops::FilterType::Nearest);
    let cutted = zoomed.crop(65, 65, 65, 63);
    let mut bytes: Vec<u8> = Vec::new();
    cutted.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png).unwrap();

    return bytes;
}