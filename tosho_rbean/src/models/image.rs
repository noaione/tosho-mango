//! A module containing information about images.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

use std::cmp::Ordering;

use serde::{Deserialize, Serialize};
use tosho_macros::AutoGetter;

/// A struct containing each image source.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize, PartialEq, Eq)]
pub struct ImageSource {
    /// The URL of the image.
    url: String,
    /// The width of the image.
    width: i32,
    /// The height of the image.
    height: i32,
}

impl ImageSource {
    /// The file name of the image.
    ///
    /// When you have the URL of `https://example.com/image.jpg?ignore=me`,
    /// the filename would become `image.jpg` including the extension.
    pub fn file_name(&self) -> String {
        let url = self.url.as_str();
        match url.rfind('/') {
            Some(index) => {
                let file_part = &url[index + 1..];
                // remove ?v=...
                let index = file_part.find('?').unwrap_or(file_part.len());
                file_part[..index].to_owned()
            }
            // Can't find? Just return the whole URL
            None => url.to_string(),
        }
    }

    /// The file extension of the image.
    ///
    /// When you have the URL of `https://example.com/image.jpg?ignore=me`,
    /// the extension would become `jpg`, when there is no extension it
    /// would return an empty string.
    pub fn extension(&self) -> String {
        let file_name = self.file_name();
        let split: Vec<&str> = file_name.rsplitn(2, '.').collect();

        if split.len() == 2 {
            split[0].to_owned()
        } else {
            String::new()
        }
    }

    /// The file stem of the image.
    ///
    /// When you have the URL of `https://example.com/image.jpg?ignore=me`,
    /// the file stem would become `image`.
    pub fn file_stem(&self) -> String {
        let file_name = self.file_name();
        let split: Vec<&str> = file_name.rsplitn(2, '.').collect();

        if split.len() == 2 {
            split[1].to_owned()
        } else {
            file_name
        }
    }
}

impl PartialOrd for ImageSource {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.height.cmp(&other.height))
    }

    fn lt(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Ordering::Less))
    }

    fn le(&self, other: &Self) -> bool {
        matches!(
            self.partial_cmp(other),
            Some(Ordering::Less | Ordering::Equal)
        )
    }

    fn gt(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Ordering::Greater))
    }

    fn ge(&self, other: &Self) -> bool {
        matches!(
            self.partial_cmp(other),
            Some(Ordering::Greater | Ordering::Equal)
        )
    }
}

impl Ord for ImageSource {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.height.cmp(&other.height)
    }
}

/// A struct containing collection of images.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct Image {
    /// WEBP images.
    webp: Vec<ImageSource>,
    /// JPEG images.
    jpg: Vec<ImageSource>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_image_source_sorting() {
        let mut images = vec![
            super::ImageSource {
                url: "https://example.com/image1.jpg".to_string(),
                width: 800,
                height: 1200,
            },
            super::ImageSource {
                url: "https://example.com/image2.jpg".to_string(),
                width: 800,
                height: 800,
            },
            super::ImageSource {
                url: "https://example.com/image3.jpg".to_string(),
                width: 800,
                height: 1600,
            },
        ];

        images.sort();

        assert_eq!(
            images,
            vec![
                super::ImageSource {
                    url: "https://example.com/image2.jpg".to_string(),
                    width: 800,
                    height: 800,
                },
                super::ImageSource {
                    url: "https://example.com/image1.jpg".to_string(),
                    width: 800,
                    height: 1200,
                },
                super::ImageSource {
                    url: "https://example.com/image3.jpg".to_string(),
                    width: 800,
                    height: 1600,
                },
            ]
        );
    }
}
