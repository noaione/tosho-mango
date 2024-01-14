use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResultHeader {
    /// The result of the request.
    pub result: bool,
    /// Error message.
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Result<R> {
    /// The result of the request.
    pub header: ResultHeader,
    /// The content of the request.
    #[serde(bound(
        deserialize = "R: Deserialize<'de>, R: Clone",
        serialize = "R: Serialize, R: Clone"
    ))]
    pub content: Option<R>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APIResult<R> {
    /// The content of the request.
    #[serde(bound(
        deserialize = "R: Deserialize<'de>, R: Clone",
        serialize = "R: Serialize, R: Clone"
    ))]
    pub result: Result<R>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Serialize, Deserialize)]
    struct TestObject {
        pub value: String,
        pub magic: u32,
    }

    #[test]
    fn test_common_result_deserialize_optional() {
        let data: APIResult<ResultHeader> = serde_json::from_str(
            r#"{
                "result": {
                    "header": {
                        "result": true,
                        "message": null
                    },
                    "content": null
                }
            }"#,
        )
        .unwrap();

        assert_eq!(data.result.header.result, true);
        assert_eq!(data.result.header.message, None);
        assert_eq!(data.result.content, None);
    }

    #[test]
    fn test_common_result_deserialize() {
        let data: APIResult<TestObject> = serde_json::from_str(
            r#"{
                "result": {
                    "header": {
                        "result": true
                    },
                    "content": {
                        "magic": 123,
                        "value": "magic value"
                    }
                }
            }"#,
        )
        .unwrap();

        assert_eq!(data.result.header.result, true);
        assert_eq!(data.result.header.message, None);

        let content_unwrap = data.result.content.unwrap();
        assert_eq!(content_unwrap.magic, 123);
        assert_eq!(content_unwrap.value, "magic value");
    }
}
