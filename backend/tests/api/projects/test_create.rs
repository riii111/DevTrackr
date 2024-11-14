use crate::api::helper::validation::assert_validation_error_with_custom_error;
use crate::common::test_app::TestApp;
use actix_web::{http::StatusCode, test};
use bson::oid::ObjectId;
use lazy_static::lazy_static;
use rstest::rstest;
use serde_json::{json, Value};

const PROJECTS_ENDPOINT: &str = "/api/projects/";

lazy_static! {
    static ref TEST_PAYLOAD: Value = json!({
        "title": "テストプロジェクト",
        "description": "これはテストプロジェクトです",
        "status": "Active",
        "start_date": "2024-01-01",
        "end_date": "2024-12-31",
        "skill_labels": ["Rust", "MongoDB"],
        "company_id": ObjectId::new().to_string(),
        "team_members": [
            {
                "name": "テストメンバー1",
                "role": "Developer"
            }
        ]
    });
}

#[actix_web::test]
async fn test_create_project_success() {
    /*
    プロジェクト作成が成功することを確認するテスト
     */
    TestApp::run_authenticated_test(|context| async move {
        // プロジェクト作成APIの実行
        let create_response = context
            .authenticated_request(
                test::TestRequest::post().set_json(&*TEST_PAYLOAD),
                PROJECTS_ENDPOINT,
            )
            .await;

        // 正常に作成されたことを確認
        assert_eq!(create_response.status(), StatusCode::CREATED);
        let create_body: Value = test::read_body_json(create_response).await;
        let project_id = create_body["id"]
            .as_str()
            .expect("Project ID not found in response");

        // プロジェクト取得APIの実行
        let get_response = context
            .authenticated_request(
                test::TestRequest::get(),
                &format!("{}{}/", PROJECTS_ENDPOINT, project_id),
            )
            .await;

        // 正常に取得できることを確認
        assert_eq!(get_response.status(), StatusCode::OK);
        let get_body: Value = test::read_body_json(get_response).await;
        assert_eq!(get_body["title"], "テストプロジェクト");
    })
    .await;
}

// バリデーションテスト用の構造体
#[derive(Debug)]
struct ValidationTestCase {
    name: &'static str,
    payload: serde_json::Value,
    field: &'static str,
    expected_message: &'static str,
}

#[rstest]
// タイトルのバリデーション
#[case::title_too_short(
    ValidationTestCase {
        name: "タイトルが空",
        payload: json!({
            "title": "",
            "description": "テストプロジェクトの説明",
            "status": "Planning",
            "skill_labels": ["Rust", "MongoDB"],
            "company_id": ObjectId::new().to_string(),
            "hourly_pay": 3000
        }),
        field: "title",
        expected_message: "タイトルは1〜100文字である必要があります"
    }
)]
#[case::title_too_long(
    ValidationTestCase {
        name: "タイトルが長すぎる",
        payload: json!({
            "title": "a".repeat(101),
            "description": "テストプロジェクトの説明",
            "status": "Planning",
            "skill_labels": ["Rust", "MongoDB"],
            "company_id": ObjectId::new().to_string(),
            "hourly_pay": 3000
        }),
        field: "title",
        expected_message: "タイトルは1〜100文字である必要があります"
    }
)]
// 説明のバリデーション
#[case::description_too_long(
    ValidationTestCase {
        name: "説明が長すぎる",
        payload: json!({
            "title": "テストプロジェクト",
            "description": "a".repeat(1001),
            "status": "Planning",
            "skill_labels": ["Rust", "MongoDB"],
            "company_id": ObjectId::new().to_string(),
            "hourly_pay": 3000
        }),
        field: "description",
        expected_message: "説明は1000文字以内である必要があります"
    }
)]
// スキルラベルのバリデーション
#[case::too_many_skill_labels(
    ValidationTestCase {
        name: "スキルラベルが多すぎる",
        payload: json!({
            "title": "テストプロジェクト",
            "description": "テストプロジェクトの説明",
            "status": "Planning",
            "skill_labels": [
                "Rust", "MongoDB", "AWS", "Docker", "Kubernetes",
                "React", "TypeScript", "Python", "Go", "Java", "C++"
            ],
            "company_id": ObjectId::new().to_string(),
            "hourly_pay": 3000
        }),
        field: "skill_labels",
        expected_message: "スキルラベルは最大10個まで登録できます"
    }
)]
// 時給のバリデーション
#[case::negative_hourly_pay(
    ValidationTestCase {
        name: "時給がマイナス",
        payload: json!({
            "title": "テストプロジェクト",
            "description": "テストプロジェクトの説明",
            "status": "Planning",
            "skill_labels": ["Rust", "MongoDB"],
            "company_id": ObjectId::new().to_string(),
            "hourly_pay": -1
        }),
        field: "hourly_pay",
        expected_message: "時給は0以上である必要があります"
    }
)]
// ステータスの無効な値
#[case::invalid_status(
    ValidationTestCase {
        name: "無効なステータス",
        payload: json!({
            "title": "テストプロジェクト",
            "description": "テストプロジェクトの説明",
            "status": "InvalidStatus",
            "skill_labels": ["Rust", "MongoDB"],
            "company_id": ObjectId::new().to_string(),
            "hourly_pay": 3000
        }),
        field: "unknown",
        expected_message: "入力形式が正しくありません"
    }
)]
#[actix_web::test]
async fn test_create_project_validation(#[case] test_case: ValidationTestCase) {
    /*
    プロジェクト作成時のバリデーションをテストする
    各バリデーションルールに違反するデータを送信し、適切なエラーメッセージが返されることを確認する
     */
    TestApp::run_authenticated_test(|context| async move {
        let response = context
            .authenticated_request(
                test::TestRequest::post().set_json(&test_case.payload),
                PROJECTS_ENDPOINT,
            )
            .await;

        assert_eq!(
            response.status(),
            StatusCode::BAD_REQUEST,
            "バリデーションテスト失敗: {}",
            test_case.name
        );

        let error_body: serde_json::Value = test::read_body_json(response).await;

        if test_case.field == "unknown" {
            assert_eq!(
                error_body,
                json!({
                    "error": "入力エラー",
                    "field_errors": [{
                        "field": "unknown",
                        "message": "入力形式が正しくありません"
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
