//! Provides image descrambling functionality.
//!
//! The original image must be scrambled and contains
//! relevant EXIF metadata to descramble the image.
//!
//! ```rust,no_run
//! use tosho_sjv::imaging::descramble_image;
//!
//! let img_bytes = [0_u8; 100];
//!
//! let descrambled_img_bytes = descramble_image(&img_bytes).unwrap();
//! ```

use std::io::Cursor;

use image::{GenericImage, GenericImageView, ImageEncoder};
use tosho_common::{ToshoResult, make_error};

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
    dest: &mut image::DynamicImage,
    src: &image::DynamicImage,
    target: DrawTarget,
) -> ToshoResult<()> {
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
    match dest.copy_from(&src_rect, target.dest_x, target.dest_y) {
        Ok(_) => {
            Ok(())
        }
        Err(e) => {
            Err(tosho_common::ToshoImageError::ImageError(tosho_common::ToshoDetailedImageError::new(
                e,
                format!("Failed to copy from source image to canvas. source_x: {}, source_y: {}, dest_x: {}, dest_y: {}", target.src_x, target.src_y, target.dest_x, target.dest_y)
            )).into())
        }
    }
}

/// Descramble image bytes, and return descrambled image bytes.
///
/// # Arguments
/// * `img_bytes` - Image bytes to descramble.
///
/// # Example
/// ```rust,no_run
/// use tosho_sjv::imaging::descramble_image;
///
/// let img_bytes = [0_u8; 100];
///
/// let descrambled_img_bytes = descramble_image(&img_bytes).unwrap();
/// ```
pub fn descramble_image(img_bytes: &[u8]) -> ToshoResult<Vec<u8>> {
    let mut cursor = Cursor::new(img_bytes);
    let exif_meta = exif::Reader::new()
        .read_from_container(&mut cursor)
        .map_err(|e| make_error!("Failed to read EXIF metadata from image: {}", e))?;

    let metadata = exif_meta.get_field(exif::Tag::ImageUniqueID, exif::In::PRIMARY);

    // Guaranteed to be Some because of the check above
    let img_unique_id = metadata
        .ok_or_else(|| make_error!("ImageUniqueID not found in EXIF metadata"))?
        .value
        .display_as(exif::Tag::ImageUniqueID)
        .to_string();
    let img_unique_id = img_unique_id.replace('"', "");

    let mut keys: Vec<u32> = vec![];
    for key in img_unique_id.split(':') {
        // Do it like this so we can get a better error message
        let key = u32::from_str_radix(key, 16).map_err(|e| {
            make_error!(
                "Failed to parse ImageUniqueID: {} at {} ({})",
                key,
                img_unique_id,
                e
            )
        })?;
        keys.push(key);
    }

    let img = image::load_from_memory(img_bytes)?;
    let (width, height) = img.dimensions();

    let x = width - CUT_WIDTH;
    let v = height - CUT_HEIGHT;
    let b = x / CELL_WIDTH_COUNT;
    let w = v / CELL_HEIGHT_COUNT;

    let mut descrambled_img = image::DynamicImage::new(x, v, img.color());

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
                dest_x: ((key % 8 + 1) * b),
                dest_y: (key / 8 + 1) * w,
                dest_width: b,
                dest_height: w,
                src_x: (idx as u32 % 8 + 1) * (b + 10),
                src_y: (idx as u32 / 8 + 1) * (w + 10),
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
        descrambled_img.as_bytes(),
        descrambled_img.width(),
        descrambled_img.height(),
        descrambled_img.color().into(),
    )?;

    buf.set_position(0);

    let data = buf.into_inner();
    drop(img);
    drop(descrambled_img);

    Ok(data)
}
