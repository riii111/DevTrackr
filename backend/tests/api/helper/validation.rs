use serde_json::{json, Value};

#[derive(Debug)]
pub struct ValidationTestCase {
    pub name: &'static str,
    pub payload: Value,
    pub field: &'static str,
    pub expected_message: &'static str,
}

pub fn assert_validation_error(body: &Value, field_name: &str, message: &str) {
    assert_eq!(
        body,
        &json!({
            "error": "入力エラー",
            "field_errors": [{
                "field": field_name,
                "message": message
            }]
        })
    );
}

pub fn assert_validation_error_with_custom_error(body: &Value, field_name: &str, message: &str) {
    assert_eq!(
        body,
        &json!({
            "error": "バリデーションエラー",
            "field_errors": [{
                "field": field_name,
                "message": message
            }]
        })
    );
}
