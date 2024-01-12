use std::{
    fs::File,
    io::{Read, Seek},
};

use tosho_kmkc::imaging::descramble_image;

#[test]
fn test_descramble_image() {
    let seed = 749191485_u32;
    let rectbox = 4_u32;

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut img_file =
        File::open(format!("{}/tests/descramble_src.tmfxture", manifest_dir)).unwrap();

    // Check if file exists, if not skip the test (lfs not fetched or something)
    if img_file.metadata().is_err() {
        assert!(true, "File not found, skipping test");

        return;
    }

    // check if it a valid image file and not lfs file
    let mut header = [0_u8; 3];
    img_file
        .read_exact(&mut header)
        .expect("Failed to read image file");

    // header should be JPEG
    if header != [0xff, 0xd8, 0xff] {
        assert!(
            true,
            "File found but is a LFS file, please fetch all the LFS files"
        );

        return;
    }

    // seek back to the beginning
    img_file
        .seek(std::io::SeekFrom::Start(0))
        .expect("Failed to seek to the beginning of the file");

    let mut buf = vec![];
    img_file
        .read_to_end(&mut buf)
        .expect("Failed to read image file");

    let descrambled = descramble_image(buf.as_ref(), rectbox, seed).unwrap();

    // Test with the reference image
    let mut ref_file =
        File::open(format!("{}/tests/descramble_out.tmfxture", manifest_dir)).unwrap();
    let mut ref_buf = vec![];
    ref_file
        .read_to_end(&mut ref_buf)
        .expect("Failed to read reference image file");

    assert_eq!(descrambled, ref_buf);
}

#[test]
#[should_panic]
fn test_1x1_image_with_4_rectbox() {
    let one_by_one = include_bytes!("1x1.png");

    descramble_image(one_by_one, 4, 749191485).unwrap();
}
