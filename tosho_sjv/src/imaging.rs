use std::io::Cursor;

use image::{GenericImage, GenericImageView, ImageEncoder};

const CUT_WIDTH: u32 = 90;
const CUT_HEIGHT: u32 = 140;
const CELL_WIDTH_COUNT: u32 = 10;
const CELL_HEIGHT_COUNT: u32 = 15;

#[derive(Debug)]
struct DrawTarget {
    dest_x: u32,
    dest_y: u32,
    dest_width: u32,
    dest_height: u32,
    src_x: u32,
    src_y: u32,
    src_width: u32,
    src_height: u32,
}

fn draw_image(
    dest: &mut image::RgbImage,
    src: &image::DynamicImage,
    target: DrawTarget,
) -> anyhow::Result<()> {
    let src_rect = src
        .crop_imm(
            target.src_x,
            target.src_y,
            target.src_width,
            target.src_height,
        )
        .resize_exact(
            target.dest_width,
            target.dest_height,
            image::imageops::FilterType::CatmullRom,
        );
    let src_rect_binding = src_rect.as_rgb8();
    if src_rect_binding.is_none() {
        anyhow::bail!("Failed to convert to RGB8");
    }
    let result = dest.copy_from(src_rect_binding.unwrap(), target.dest_x, target.dest_y);
    if result.is_err() {
        anyhow::bail!(
            "Failed to copy from source image to canvas. source_x: {}, source_y: {}, dest_x: {}, dest_y: {}",
            target.src_x,
            target.src_y,
            target.dest_x,
            target.dest_y
        );
    }

    Ok(())
}

/// Descramble image bytes, and return descrambled image bytes.
///
/// # Arguments
/// * `img_bytes` - Image bytes to descramble.
///
/// # Example
/// ```no_run
/// use tosho_sjv::imaging::descramble_image;
///
/// let img_bytes = [0_u8; 100];
///
/// let descrambled_img_bytes = descramble_image(&img_bytes).unwrap();
/// ```
pub fn descramble_image(img_bytes: &[u8]) -> anyhow::Result<Vec<u8>> {
    let mut cursor = Cursor::new(img_bytes);
    let exif_meta = exif::Reader::new().read_from_container(&mut cursor)?;

    let metadata = exif_meta.get_field(exif::Tag::ImageUniqueID, exif::In::PRIMARY);

    if metadata.is_none() {
        anyhow::bail!("ImageUniqueID not found in EXIF metadata");
    }

    let img_unique_id = metadata
        .unwrap()
        .value
        .display_as(exif::Tag::ImageUniqueID)
        .to_string();
    let img_unique_id = img_unique_id.replace('"', "");

    let keys: Vec<u32> = img_unique_id
        .split(':')
        .map(|x| {
            u32::from_str_radix(x, 16).unwrap_or_else(|_| {
                panic!("Failed to parse ImageUniqueID: {} ({})", img_unique_id, x)
            })
        })
        .collect();

    let img = image::load_from_memory(img_bytes)?;
    let (width, height) = img.dimensions();

    let x = width - CUT_WIDTH;
    let v = height - CUT_HEIGHT;
    let b = x / CELL_WIDTH_COUNT;
    let w = v / CELL_HEIGHT_COUNT;

    let mut descrambled_img = image::RgbImage::new(x, v);

    // Borders
    draw_image(
        &mut descrambled_img,
        &img,
        DrawTarget {
            dest_x: 0,
            dest_y: 0,
            dest_width: x,
            dest_height: w,
            src_x: 0,
            src_y: 0,
            src_width: x,
            src_height: w,
        },
    )?;

    draw_image(
        &mut descrambled_img,
        &img,
        DrawTarget {
            dest_x: 0,
            dest_y: w,
            dest_width: b,
            dest_height: v - 2 * w,
            src_x: 0,
            src_y: w + 10,
            src_width: b,
            src_height: v - 2 * w,
        },
    )?;

    draw_image(
        &mut descrambled_img,
        &img,
        DrawTarget {
            dest_x: 0,
            dest_y: 14 * w,
            dest_width: x,
            dest_height: height - 14 * (w + 10),
            src_x: 0,
            src_y: 14 * (w + 10),
            src_width: x,
            src_height: height - 14 * (w + 10),
        },
    )?;

    draw_image(
        &mut descrambled_img,
        &img,
        DrawTarget {
            dest_x: 9 * b,
            dest_y: w,
            dest_width: b + (x - 10 * b),
            dest_height: v - 2 * w,
            src_x: 9 * (b + 10),
            src_y: w + 10,
            src_width: b + (x - 10 * b),
            src_height: v - 2 * w,
        },
    )?;

    for (idx, key) in keys.iter().enumerate() {
        draw_image(
            &mut descrambled_img,
            &img,
            DrawTarget {
                dest_x: ((key % 8 + 1) * b) - 1,
                dest_y: (key / 8 + 1) * w - 1,
                dest_width: b,
                dest_height: w,
                src_x: (idx as u32 % 8 + 1) * (b + 10) - 1,
                src_y: (idx as u32 / 8 + 1) * (w + 10) - 1,
                src_width: b,
                src_height: w,
            },
        )?;
    }

    let mut buf = Cursor::new(Vec::new());

    image::codecs::png::PngEncoder::new_with_quality(
        &mut buf,
        image::codecs::png::CompressionType::Best,
        image::codecs::png::FilterType::Adaptive,
    )
    .write_image(
        &descrambled_img,
        descrambled_img.width(),
        descrambled_img.height(),
        image::ColorType::Rgb8,
    )?;

    buf.set_position(0);

    let data = buf.into_inner();
    drop(img);
    drop(descrambled_img);

    Ok(data)
}
