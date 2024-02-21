//! A module containing information about images.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

/// A struct containing each image source.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ImageSource {
    /// The URL of the image.
    pub url: String,
    /// The width of the image.
    pub width: i32,
    /// The height of the image.
    pub height: i32,
}

impl ImageSource {
    /// The file name of the image.
    ///
    /// # Examples
    /// ```
    /// use tosho_rbean::models::ImageSource;
    ///
    /// let page = ImageSource {
    ///     width: 800,
    ///     height: 1200,
    ///     url: "https://example.com/image.jpg?ignore=me".to_string(),
    /// };
    ///
    /// assert_eq!(page.file_name(), "image.jpg");
    /// ```
    pub fn file_name(&self) -> String {
        let url = self.url.as_str();
        let index = url.rfind('/').unwrap();
        let file_part = &url[index + 1..];
        // remove ?v=...
        let index = file_part.find('?').unwrap_or(file_part.len());
        file_part[..index].to_owned()
    }

    /// The file extension of the image.
    ///
    /// # Examples
    /// ```
    /// use tosho_rbean::models::ImageSource;
    ///
    /// let page = ImageSource {
    ///     width: 800,
    ///     height: 1200,
    ///     url: "https://example.com/image.jpg?ignore=me".to_string(),
    /// };
    ///
    /// assert_eq!(page.extension(), "jpg");
    /// ```
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
    /// # Examples
    /// ```
    /// use tosho_rbean::models::ImageSource;
    ///
    /// let page = ImageSource {
    ///     width: 800,
    ///     height: 1200,
    ///     url: "https://example.com/image.jpg?ignore=me".to_string(),
    /// };
    ///
    /// assert_eq!(page.file_stem(), "image");
    /// ```
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    /// WEBP images.
    pub webp: Vec<ImageSource>,
    /// JPEG images.
    pub jpg: Vec<ImageSource>,
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
