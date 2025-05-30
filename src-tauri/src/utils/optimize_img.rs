use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use image::{self, GenericImageView};
use imagequant::Attributes;
use lodepng;

// 图片压缩
pub fn optimize_img(img_bytes: &[u8]) -> Result<(String, String), String> {
    let img: image::DynamicImage =
        image::load_from_memory(&img_bytes).map_err(|e| e.to_string())?;
    let (resized_img, img_size, (new_width, new_height)) = resize_img(img);

    let pixels: Vec<imagequant::RGBA> = resized_img
        .as_raw()
        .chunks(4)
        .map(|chunk| imagequant::RGBA {
            r: chunk[0],
            g: chunk[1],
            b: chunk[2],
            a: chunk[3],
        })
        .collect();

    // 创建 imagequant 的属性
    let attrs = Attributes::new();
    // 创建 imagequant 的图像实例
    let mut image = attrs
        .new_image(pixels, new_width, new_height, 0.0)
        .expect("Failed to create imagequant instance");
    // 执行量化
    let mut quantized = attrs
        .quantize(&mut image)
        .expect("Failed to quantize image");
    // 获取量化后的像素数据
    let (palette, pixels) = quantized
        .remapped(&mut image)
        .expect("Failed to remap image");

    // 将量化后的数据转换为 PngImage
    let mut encoder = lodepng::Encoder::new();
    encoder
        .set_palette(palette.as_slice())
        .expect("encoder failed to set palette");
    let png_vec: Vec<u8> = encoder
        .encode(pixels.as_slice(), new_width, new_height)
        .expect("encoder failed to encode image");

    Ok((STANDARD.encode(&png_vec), img_size))
}

const MAX_EDGE_SIZE: usize = 240;

// 生成一张小尺寸的图
fn resize_img(
    img: image::DynamicImage,
) -> (
    image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    String,
    (usize, usize),
) {
    let (width, height) = img.dimensions();
    let uw = width as usize;
    let uh = height as usize;

    let img_size = format!("{}x{}", width, height);

    let mut new_width = uw;
    let mut new_height = uh;

    if uw > MAX_EDGE_SIZE || uh > MAX_EDGE_SIZE {
        if uw > uh {
            new_width = MAX_EDGE_SIZE;
            new_height = new_width * uh / uw;
        } else {
            new_height = MAX_EDGE_SIZE;
            new_width = new_height * uw / uh;
        }

        let resized_img = image::imageops::resize(
            &img,
            new_width as u32,
            new_height as u32,
            image::imageops::FilterType::Lanczos3,
        );

        return (resized_img, img_size, (new_width, new_height));
    }

    let rgba_img = img.to_rgba8();
    (rgba_img, img_size, (new_width, new_height))
}
