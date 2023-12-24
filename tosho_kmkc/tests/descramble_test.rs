use std::{fs::File, io::Read};

use tosho_kmkc::imaging::descramble_image;

#[test]
fn test_descramble_image() {
    let seed = 749191485_u32;
    let rectbox = 4_u32;

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut img_file =
        File::open(format!("{}/tests/descramble_src.tmfxture", manifest_dir)).unwrap();

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
