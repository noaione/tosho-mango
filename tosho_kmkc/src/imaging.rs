use std::io::Cursor;

use image::{GenericImage, GenericImageView, ImageEncoder};

fn u32_to_f32(n: u32) -> f32 {
    if n > i32::MAX as u32 {
        panic!("u32_to_i32: u32 is too big");
    }

    n as f32
}

fn f32_to_u32(n: f32) -> u32 {
    n as u32
}

fn calc_block_size(width: u32, height: u32, rectbox: u32) -> (Option<u32>, Option<u32>) {
    if width < rectbox || height < rectbox {
        return (None, None);
    }

    let n = 8.0_f32;
    let rectbox = u32_to_f32(rectbox);
    // div width/n, floor and mul n
    let block_width = (u32_to_f32(width) / n).floor() * n;
    let block_height = (u32_to_f32(height) / n).floor() * n;

    let width_rect = (block_width / rectbox).floor();
    let width_rect = f32_to_u32(width_rect);
    let height_rect = (block_height / rectbox).floor();
    let height_rect = f32_to_u32(height_rect);

    (Some(width_rect), Some(height_rect))
}

fn seed_generator(seed: u32) -> impl Iterator<Item = u32> {
    let mut tdata = seed;
    std::iter::from_fn(move || {
        tdata = tdata ^ (tdata << 13);
        tdata = tdata ^ (tdata >> 17);
        tdata = tdata ^ (tdata << 5);
        Some(tdata)
    })
}

fn generate_copy_targets(rectbox: u32, seed: u32) -> Vec<((u32, u32), (u32, u32))> {
    let mut targets: Vec<((u32, u32), (u32, u32))> = Vec::new();
    let mut seed_gen = seed_generator(seed);

    let mut seed_arrays = Vec::new();
    for i in 0..rectbox * rectbox {
        let next = seed_gen.next().unwrap();
        seed_arrays.push((next, i))
    }
    drop(seed_gen);

    // sort by first element
    seed_arrays.sort_by(|a, b| a.0.cmp(&b.0));
    let index_only: Vec<u32> = seed_arrays.iter().map(|x| x.1).collect::<Vec<u32>>();

    for (i, idx_seed) in index_only.iter().enumerate() {
        let source_x = *idx_seed % rectbox;
        let target_x = i as u32 % rectbox;

        let source_y = f32_to_u32((u32_to_f32(*idx_seed) / rectbox as f32).floor());
        let target_y = f32_to_u32((u32_to_f32(i as u32) / rectbox as f32).floor());

        targets.push(((source_x, source_y), (target_x, target_y)));
    }

    targets
}

/// Descramble image bytes, and return descrambled image bytes.
///
/// # Arguments
/// * `img_bytes` - Image bytes to descramble.
/// * `rectbox` - How much block that divide the images, usually `4`.
/// * `scramble_seed` - The seed used to scramble the image. Available in the [`models::WebEpisodeViewerResponse`]
///                     response.
///
/// # Example
/// ```no_run
/// use tosho_kmkc::imaging::descramble_image;
///
/// let img_bytes = [0_u8; 100];
///
/// let descrambled_img_bytes = descramble_image(&img_bytes, 4, 749191485).unwrap();
/// ```
pub fn descramble_image(
    img_bytes: &[u8],
    rectbox: u32,
    scramble_seed: u32,
) -> anyhow::Result<Vec<u8>> {
    // read img_source as image
    let img = image::load_from_memory(img_bytes)?;

    let (width, height) = img.dimensions();
    let (width_rect, height_rect) = calc_block_size(width, height, rectbox);

    match (width_rect, height_rect) {
        (Some(width_rect), Some(height_rect)) => {
            let mut canvas =
                image::DynamicImage::new(width_rect * rectbox, height_rect * rectbox, img.color());

            for ((source_x, source_y), (dest_x, dest_y)) in
                generate_copy_targets(rectbox, scramble_seed)
            {
                let source_x = source_x * width_rect;
                let source_y = source_y * height_rect;
                let dest_x = dest_x * width_rect;
                let dest_y = dest_y * height_rect;

                let cropped_img = img.crop_imm(source_x, source_y, width_rect, height_rect);
                canvas.copy_from(&cropped_img, dest_x, dest_y).unwrap_or_else(|_| {
                    panic!("Failed to copy from source image to canvas. source_x: {}, source_y: {}, dest_x: {}, dest_y: {}", source_x, source_y, dest_x, dest_y)
                });
            }

            // output image to Vec<u8>
            let mut buf = Cursor::new(Vec::new());

            image::codecs::png::PngEncoder::new_with_quality(
                &mut buf,
                image::codecs::png::CompressionType::Best,
                image::codecs::png::FilterType::Adaptive,
            )
            .write_image(
                canvas.as_bytes(),
                canvas.width(),
                canvas.height(),
                canvas.color(),
            )?;

            buf.set_position(0);

            let data = buf.into_inner();
            // Close the file handle
            drop(img);
            drop(canvas);

            Ok(data)
        }
        _ => {
            anyhow::bail!("Image is too small!")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_block_size() {
        let width = 960_u32;
        let height = 1378_u32;

        let (width_rect, height_rect) = calc_block_size(width, height, 4);
        assert_eq!(width_rect, Some(240));
        assert_eq!(height_rect, Some(344));
    }

    #[test]
    fn test_calc_block_size_small() {
        let width = 1_u32;
        let height = 1_u32;

        let (width_rect, height_rect) = calc_block_size(width, height, 4);
        assert_eq!(width_rect, None);
        assert_eq!(height_rect, None);

        let width = 1_u32;
        let height = 10_u32;
        let (width_rect, height_rect) = calc_block_size(width, height, 4);
        assert_eq!(width_rect, None);
        assert_eq!(height_rect, None);
    }

    #[test]
    fn test_seed_generator() {
        let rectbox = 4_u32;
        let mut seeder = seed_generator(749191485_u32);

        let mut seed_gen = vec![];
        for _ in 0..rectbox {
            seed_gen.push(seeder.next().unwrap());
        }

        assert_eq!(
            seed_gen,
            vec![1149330653, 1799678672, 902605375, 2984793402]
        );
    }

    #[test]
    fn test_generate_copy_targets() {
        let copy_targets = generate_copy_targets(4, 749191485);

        let expect_targets: Vec<((u32, u32), (u32, u32))> = vec![
            ((1, 1), (0, 0)),
            ((2, 0), (1, 0)),
            ((3, 1), (2, 0)),
            ((0, 0), (3, 0)),
            ((3, 2), (0, 1)),
            ((0, 2), (1, 1)),
            ((1, 3), (2, 1)),
            ((1, 0), (3, 1)),
            ((0, 3), (0, 2)),
            ((3, 0), (1, 2)),
            ((2, 1), (2, 2)),
            ((0, 1), (3, 2)),
            ((3, 3), (0, 3)),
            ((2, 3), (1, 3)),
            ((2, 2), (2, 3)),
            ((1, 2), (3, 3)),
        ];

        assert_eq!(copy_targets, expect_targets);
    }

    #[test]
    #[should_panic]
    fn test_u32_to_f32_panic() {
        u32_to_f32(u32::MAX);
    }
}
