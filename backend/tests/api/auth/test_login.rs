use crate::api::helper::validation::{
    assert_validation_error, assert_validation_error_with_custom_error, ValidationTestCase,
};
use crate::common::test_app::TestApp;
use actix_web::{http::StatusCode, test};
use rstest::rstest;
use serde_json::json;

const LOGIN_ENDPOINT: &str = "/api/auth/login/";

// Cookie検証用の構造体
struct CookieCheck<'a> {
    name: &'a str,
    should_be_http_only: bool,
}

const COOKIE_CHECKS: &[CookieCheck<'static>] = &[
    CookieCheck {
        name: "access_token",
        should_be_http_only: false,
    },
    CookieCheck {
        name: "refresh_token",
        should_be_http_only: true,
    },
];

#[actix_web::test]
async fn test_login_success() {
    /*
    ログインが成功することを確認するテスト
     */
    TestApp::run_test(|context| async move {
        let payload = json!({
            "email": context.app.test_user.email,
            "password": context.app.test_user.password
        });

        // 認証不要のAPIなのでtest::call_serviceを直接使用
        let response = test::call_service(
            context.service(),
            test::TestRequest::post()
                .uri(LOGIN_ENDPOINT)
                .set_json(&payload)
                .to_request(),
        )
        .await;

        assert_eq!(response.status(), StatusCode::OK);

        let cookies: Vec<_> = response
            .headers()
            .get_all(actix_web::http::header::SET_COOKIE)
            .map(|v| v.to_str().unwrap())
            .collect();

        for check in COOKIE_CHECKS {
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
    })
    .await;
}

#[actix_web::test]
async fn test_login_invalid_credentials() {
    /*
    無効なメールアドレスかパスワードの場合は400エラーが返ることを確認するテスト
     */
    TestApp::run_test(|context| async move {
        let payload = json!({
            "email": "test@example.com",
            "password": "wrongpassword"
        });
        // 認証不要のAPIなのでtest::call_serviceを直接使用
        let res = test::call_service(
            context.service(),
            test::TestRequest::post()
                .uri(LOGIN_ENDPOINT)
                .set_json(&payload)
                .to_request(),
        )
        .await;

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);

        let body: serde_json::Value = test::read_body_json(res).await;
        assert_eq!(body, json!({"error": "認証に失敗しました"}));
    })
    .await;
}

#[rstest]
// メールアドレスのバリデーション
#[case::invalid_email(
    ValidationTestCase {
        name: "無効なメールアドレス形式",
        payload: json!({
            "email": "invalid-email",
            "password": "ValidPass123!"
        }),
        field: "email",
        expected_message: "有効なメールアドレスを入力してください"
    }
)]
#[case::empty_email(
    ValidationTestCase {
        name: "空のメールアドレス",
        payload: json!({
            "email": "",
            "password": "ValidPass123!"
        }),
        field: "email",
        expected_message: "有効なメールアドレスを入力してください"
    }
)]
// パスワードのバリデーション
#[case::short_password(
    ValidationTestCase {
        name: "短すぎるパスワード",
        payload: json!({
            "email": "test@example.com",
            "password": "short"
        }),
        field: "password",
        expected_message: "パスワードは8文字以上である必要があります"
    }
)]
#[case::empty_password(
    ValidationTestCase {
        name: "空のパスワード",
        payload: json!({
            "email": "test@example.com",
            "password": ""
        }),
        field: "password",
        expected_message: "パスワードは8文字以上である必要があります"
    }
)]
// 必須フィールドの欠落
#[case::missing_email(
    ValidationTestCase {
        name: "メールアドレス欠落",
        payload: json!({
            "password": "ValidPass123!"
        }),
        field: "email",
        expected_message: "必須項目です"
    }
)]
#[case::missing_password(
    ValidationTestCase {
        name: "パスワード欠落",
        payload: json!({
            "email": "test@example.com"
        }),
        field: "password",
        expected_message: "必須項目です"
    }
)]
#[actix_web::test]
async fn test_login_invalid_input(#[case] test_case: ValidationTestCase) {
    /*
    パラメータに不備がある場合、バリデーションエラーが発生することを確認するテスト
    */
    TestApp::run_test(|context| async move {
        // 認証不要のAPIなのでtest::call_serviceを直接使用
        let resp = test::call_service(
            context.service(),
            test::TestRequest::post()
                .uri(LOGIN_ENDPOINT)
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

        if test_case.name.contains("欠落") {
            assert_validation_error(&body, test_case.field, "必須項目です");
        } else {
            assert_validation_error_with_custom_error(
                &body,
                test_case.field,
                test_case.expected_message,
            );
        }
    })
    .await;
}
