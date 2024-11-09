use crate::common::test_app::TestApp;
use actix_web::{http::StatusCode, test};
use rstest::rstest;
use serde_json::json;

const REGISTER_ENDPOINT: &str = "/api/auth/register/";

#[actix_web::test]
async fn test_register_success() {
    /*
    ユーザー登録が成功することを確認するテスト
    */
    let test_app = TestApp::new().await;
    let app = test_app.build_test_app().await;

    let payload = json!({
        "email": "newuser@example.com",
        "password": "newpassword123",
        "username": "newuser"
    });

    let res = test::call_service(
        &app,
        test::TestRequest::post()
            .uri(REGISTER_ENDPOINT)
            .set_json(&payload)
            .to_request(),
    )
    .await;

    assert_eq!(res.status(), StatusCode::CREATED);

    let cookies: Vec<_> = res
        .headers()
        .get_all(actix_web::http::header::SET_COOKIE)
        .map(|v| v.to_str().unwrap())
        .collect();

    // Cookie検証用の構造体
    struct CookieCheck<'a> {
        name: &'a str,
        should_be_http_only: bool,
    }

    let cookie_checks = vec![
        CookieCheck {
            name: "access_token",
            should_be_http_only: false,
        },
        CookieCheck {
            name: "refresh_token",
            should_be_http_only: true,
        },
        CookieCheck {
            name: "firstLogin",
            should_be_http_only: false,
        },
    ];

    for check in cookie_checks {
        let cookie = cookies
            .iter()
            .find(|c| c.starts_with(&format!("{}=", check.name)))
            .unwrap_or_else(|| panic!("{} cookie not found", check.name));

        assert!(cookie.contains("Path=/"));
        assert_eq!(
            cookie.contains("HttpOnly"),
            check.should_be_http_only,
            "Unexpected HttpOnly flag for {} cookie",
            check.name
        );
    }
}

#[actix_web::test]
async fn test_register_duplicate_email() {
    /*
    同じメールアドレスで登録できないことを確認するテスト
    */
    let test_app = TestApp::new().await;
    let app = test_app.build_test_app().await;

    // 1人目のユーザーを登録
    test_app
        .create_new_user("duplicate@example.com", "password123", "firstuser")
        .await
        .expect("Failed to create first user");

    // 2人目のユーザーを同じメールアドレスで登録
    let payload = json!({
        "email": "duplicate@example.com",
        "password": "different_password",
        "username": "seconduser"
    });

    let res = test::call_service(
        &app,
        test::TestRequest::post()
            .uri(REGISTER_ENDPOINT)
            .set_json(&payload)
            .to_request(),
    )
    .await;

    assert_eq!(res.status(), StatusCode::BAD_REQUEST);

    let body: serde_json::Value = test::read_body_json(res).await;
    assert_eq!(
        body,
        json!({
            "error": "不正なリクエスト",
            "message": "バリデーションに失敗したか、処理中にエラーが発生しました",
            "code": "BAD_REQUEST"
        })
    );
}

// バリデーションテスト用の構造体
#[derive(Debug)]
struct ValidationTestCase {
    name: &'static str,
    payload: serde_json::Value,
    expected_message: &'static str,
}

#[rstest]
// メールアドレスのバリデーション
#[case::invalid_email(
    ValidationTestCase {
        name: "無効なメールアドレス形式",
        payload: json!({
            "email": "invalid-email",
            "password": "ValidPass123!",
            "username": "testuser"
        }),
        expected_message: "有効なメールアドレスを入力してください"
    }
)]
#[case::empty_email(
    ValidationTestCase {
        name: "空のメールアドレス",
        payload: json!({
            "email": "",
            "password": "ValidPass123!",
            "username": "testuser"
        }),
        expected_message: "有効なメールアドレスを入力してください"
    }
)]
// パスワードのバリデーション
#[case::short_password(
    ValidationTestCase {
        name: "短すぎるパスワード",
        payload: json!({
            "email": "test@example.com",
            "password": "short",
            "username": "testuser"
        }),
        expected_message: "パスワードは8文字以上である必要があります"
    }
)]
#[case::empty_password(
    ValidationTestCase {
        name: "空のパスワード",
        payload: json!({
            "email": "test@example.com",
            "password": "",
            "username": "testuser"
        }),
        expected_message: "パスワードは8文字以上である必要があります"
    }
)]
// ユーザー名のバリデーション
#[case::empty_username(
    ValidationTestCase {
        name: "空のユーザー名",
        payload: json!({
            "email": "test@example.com",
            "password": "ValidPass123!",
            "username": ""
        }),
        expected_message: "名前は1文字以上である必要があります"
    }
)]
// 必須フィールドの欠落
#[case::missing_email(
    ValidationTestCase {
        name: "メールアドレス欠落",
        payload: json!({
            "password": "ValidPass123!",
            "username": "testuser"
        }),
        expected_message: "必須項目です"
    }
)]
#[case::missing_password(
    ValidationTestCase {
        name: "パスワード欠落",
        payload: json!({
            "email": "test@example.com",
            "username": "testuser"
        }),
        expected_message: "必須項目です"
    }
)]
#[case::missing_username(
    ValidationTestCase {
        name: "ユーザー名欠落",
        payload: json!({
            "email": "test@example.com",
            "password": "ValidPass123!"
        }),
        expected_message: "必須項目です"
    }
)]
#[actix_web::test]
async fn test_register_invalid_input(#[case] test_case: ValidationTestCase) {
    /*
    パラメータに不備がある場合、バリデーションエラーが発生することを確認するテスト
    */
    let test_app = TestApp::new().await;
    let app = test_app.build_test_app().await;

    let resp = test::call_service(
        &app,
        test::TestRequest::post()
            .uri(REGISTER_ENDPOINT)
            .set_json(&test_case.payload)
            .to_request(),
    )
    .await;

    assert_eq!(
        resp.status(),
        StatusCode::BAD_REQUEST,
        "バリデーションテスト失敗: {}",
        test_case.name
    );

    let body: serde_json::Value = test::read_body_json(resp).await;

    // 必須フィールドが欠落している場合
    if test_case.name.contains("欠落") {
        let field_name = if test_case.name.contains("メールアドレス") {
            "email"
        } else if test_case.name.contains("パスワード") {
            "password"
        } else {
            "username"
        };

        assert_eq!(
            body,
            json!({
                "error": "入力エラー",
                "field_errors": [{
                    "field": field_name,
                    "message": "必須項目です"
                }]
            })
        );
    } else {
        // その他のバリデーションエラーの場合
        let field_name = if test_case.name.contains("メールアドレス") {
            "email"
        } else if test_case.name.contains("パスワード") {
            "password"
        } else {
            "username"
        };

        assert_eq!(
            body,
            json!({
                "error": "バリデーションエラー",
                "field_errors": [{
                    "field": field_name,
                    "message": test_case.expected_message
                }]
            })
        );
    }
}
