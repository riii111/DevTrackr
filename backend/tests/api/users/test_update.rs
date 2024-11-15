use crate::common::test_app::TestApp;
use crate::api::helper::validation::{assert_validation_error_with_custom_error, ValidationTestCase};
use actix_web::{http::StatusCode, test};
use lazy_static::lazy_static;
use rstest::rstest;
use serde_json::json;

lazy_static! {
    static ref UPDATE_PAYLOAD: serde_json::Value = json!({
        "email": "updated@example.com",
        "username": "Updated User",
        "role": "FrontEnd",
        "password": "newpassword123"
    });
}

const USERS_ENDPOINT: &str = "/api/users/me/";

#[actix_web::test]
async fn test_update_user_success() {
    /*
    ユーザー情報の更新が成功することを確認するテスト
     */
    TestApp::run_authenticated_test(|context| async move {
        // ユーザー情報を更新
        let update_response = context
            .authenticated_request(
                test::TestRequest::put().set_json(&*UPDATE_PAYLOAD),
                USERS_ENDPOINT,
            )
            .await;

        assert_eq!(update_response.status(), StatusCode::NO_CONTENT);

        // 更新後のユーザー情報を取得して検証
        let get_response = context
            .authenticated_request(test::TestRequest::get(), USERS_ENDPOINT)
            .await;

        assert_eq!(get_response.status(), StatusCode::OK);
        let body: serde_json::Value = test::read_body_json(get_response).await;

        assert_eq!(body["email"], "updated@example.com");
        assert_eq!(body["username"], "Updated User");
        assert_eq!(body["role"], "FrontEnd");
    })
    .await;
}


#[actix_web::test]
async fn test_update_user_with_optional_fields() {
    /*
    オプショナルフィールド（role, avatar）を含むユーザー情報の更新が成功することを確認するテスト
     */
    TestApp::run_authenticated_test(|context| async move {
        let update_payload = json!({
            "email": "updated@example.com",
            "username": "Updated User",
            "role": "BackEnd",
            "password": "newpassword123",
            "avatar": "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAACklEQVR4nGMAAQAABQABDQottAAAAABJRU5ErkJggg=="
        });

        // ユーザー情報を更新
        let update_response = context
            .authenticated_request(
                test::TestRequest::put().set_json(&update_payload),
                USERS_ENDPOINT,
            )
            .await;

        assert_eq!(update_response.status(), StatusCode::NO_CONTENT);

        // 更新後のユーザー情報を取得して検証
        let get_response = context
            .authenticated_request(test::TestRequest::get(), USERS_ENDPOINT)
            .await;

        assert_eq!(get_response.status(), StatusCode::OK);
        let body: serde_json::Value = test::read_body_json(get_response).await;
        
        assert_eq!(body["email"], "updated@example.com");
        assert_eq!(body["username"], "Updated User");
        assert_eq!(body["role"], "BackEnd");
        assert!(body["avatar_url"].is_string()); // アバターURLが生成されていることを確認
    })
    .await;
}

#[actix_web::test]
async fn test_update_user_without_optional_fields() {
    /*
    オプショナルフィールドを含まないユーザー情報の更新が成功することを確認するテスト
     */
    TestApp::run_authenticated_test(|context| async move {
        let update_payload = json!({
            "email": "updated@example.com",
            "username": "Updated User"
        });

        let update_response = context
            .authenticated_request(
                test::TestRequest::put().set_json(&update_payload),
                USERS_ENDPOINT,
            )
            .await;

        assert_eq!(update_response.status(), StatusCode::NO_CONTENT);

        // 更新後のユーザー情報を取得して検証
        let get_response = context
            .authenticated_request(test::TestRequest::get(), USERS_ENDPOINT)
            .await;

        assert_eq!(get_response.status(), StatusCode::OK);
        let body: serde_json::Value = test::read_body_json(get_response).await;
        
        assert_eq!(body["email"], "updated@example.com");
        assert_eq!(body["username"], "Updated User");
        // オプショナルフィールドは既存の値が維持されているはず
    })
    .await;
}


#[actix_web::test]
async fn test_update_user_unauthorized() {
    /*
    認証なしでアクセスした場合は401エラーが返ることを確認するテスト
     */
    TestApp::run_test(|context| async move {
        let response = test::call_service(
            context.service(),
            test::TestRequest::put()
                .uri(USERS_ENDPOINT)
                .set_json(&*UPDATE_PAYLOAD)
                .to_request(),
        )
        .await;

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    })
    .await;
}

#[rstest]
#[case(ValidationTestCase {
    name: "無効なメールアドレス",
    payload: json!({
        "email": "invalid-email",
        "username": "Test User",
        "role": "FrontEnd",
        "password": "password123"
    }),
    field: "email",
    expected_message: "有効なメールアドレスを入力してください"
})]
#[case(ValidationTestCase {
    name: "ユーザー名が空",
    payload: json!({
        "email": "test@example.com",
        "username": "",
        "role": "FrontEnd",
        "password": "password123"
    }),
    field: "username",
    expected_message: "名前は1文字以上である必要があります"
})]
#[case(ValidationTestCase {
    name: "パスワードが短すぎる",
    payload: json!({
        "email": "test@example.com",
        "username": "Test User",
        "password": "short",
        "role": "FrontEnd"
    }),
    field: "password",
    expected_message: "パスワードは8文字以上である必要があります"
})]
#[case(ValidationTestCase {
    name: "無効なロール",
    payload: json!({
        "email": "test@example.com",
        "username": "Test User",
        "password": "password123",
        "role": "InvalidRole"
    }),
    field: "unknown",
    expected_message: "入力形式が正しくありません"  // デシリアライズエラー
})]
#[case(ValidationTestCase {
    name: "無効なアバター画像URL",
    payload: json!({
        "email": "test@example.com",
        "username": "Test User",
        "password": "password123",
        "role": "FrontEnd",
        "avatar": "invalid-base64-data"
    }),
    field: "unknown",
    expected_message: "Invalid base64 data format"  // デシリアライズエラー
})]
#[actix_web::test]
async fn test_update_user_validation(#[case] test_case: ValidationTestCase) {
    TestApp::run_authenticated_test(|context| async move {
        let response = context
            .authenticated_request(
                test::TestRequest::put().set_json(&test_case.payload),
                USERS_ENDPOINT,
            )
            .await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let error_body: serde_json::Value = test::read_body_json(response).await;

        if test_case.name == "無効なアバター画像URL" {
            // デシリアライズエラー
            assert_eq!(
                error_body,
                json!({
                    "error": "不正なリクエスト",
                    "message": test_case.expected_message,
                    "code": "BAD_REQUEST"
                })
            );
        } else if test_case.name == "無効なロール" {
            // デシリアライズエラー
            assert_eq!(
                error_body,
                json!({
                    "error": "入力エラー",
                    "field_errors": [{
                        "field": test_case.field,
                        "message": test_case.expected_message
                    }]
                })
            );
        } else {
            assert_validation_error_with_custom_error(
                &error_body,
                test_case.field,
                test_case.expected_message,
            );
        }
    })
    .await;
}
