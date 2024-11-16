use crate::api::companies::helper::create_test_company;
use crate::api::helper::validation::{
    assert_validation_error_with_custom_error, ValidationTestCase,
};
use crate::common::test_app::TestApp;
use actix_web::{http::StatusCode, test};
use bson::oid::ObjectId;
use rstest::rstest;
use serde_json::{json, Value};

const PROJECTS_ENDPOINT: &str = "/api/projects/";

#[actix_web::test]
async fn test_create_project_success() {
    /*
    プロジェクト作成が成功することを確認するテスト
     */
    TestApp::run_authenticated_test(|context| async move {
        // 企業作成のレスポンスを確認
        let company_id = create_test_company(&context).await;
        println!("Created company ID: {}", company_id);

        let payload = json!({
            "title": "テストプロジェクト",
            "description": "これはテストプロジェクトです",
            "status": "Planning",
            "skill_labels": ["Rust", "MongoDB"],
            "company_id": company_id,
            "hourly_pay": 3000
        });

        // リクエストの詳細をログ出力
        println!("Making request to: {}", PROJECTS_ENDPOINT);
        println!(
            "Request payload: {}",
            serde_json::to_string_pretty(&payload).unwrap()
        );

        let create_response = context
            .authenticated_request(
                test::TestRequest::post().set_json(&payload),
                PROJECTS_ENDPOINT,
            )
            .await;

        // レスポンスの詳細をログ出力
        println!("Response status: {}", create_response.status());
        println!("Response headers: {:?}", create_response.headers());

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

#[actix_web::test]
async fn test_create_project_unauthorized() {
    /*
    認証なしでアクセスした場合は401エラーが返ることを確認するテスト
     */
    TestApp::run_test(|context| async move {
        let response = test::call_service(
            context.service(),
            test::TestRequest::post()
                .uri(PROJECTS_ENDPOINT)
                .set_json(json!({
                    "title": "テストプロジェクト",
                    "description": "これはテストプロジェクトです",
                    "status": "Planning",
                    "skill_labels": ["Rust", "MongoDB"],
                    "company_id": ObjectId::new().to_string(),
                    "hourly_pay": 3000
                }))
                .to_request(),
        )
        .await;

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    })
    .await;
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
        // 先に企業を作成
        let company_id = create_test_company(&context).await;

        // ペイロードにcompany_idを追加
        let mut payload = test_case.payload.clone();
        if let Some(obj) = payload.as_object_mut() {
            obj.insert("company_id".to_string(), json!(company_id));
        }

        let response = context
            .authenticated_request(
                test::TestRequest::post().set_json(&payload),
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

#[actix_web::test]
async fn test_create_project_with_non_existent_company() {
    /*
    存在しない企業IDを指定してプロジェクト作成を試みた場合、404エラーが返されることを確認するテスト
    （MongoDBである以上、外部キーの連携は実装必要なのでテストで担保）
     */
    TestApp::run_authenticated_test(|context| async move {
        let payload = json!({
            "title": "テストプロジェクト",
            "description": "これはテストプロジェクトです",
            "status": "Planning",
            "skill_labels": ["Rust", "MongoDB"],
            "company_id": ObjectId::new().to_string(),  // 存在しない企業ID
            "hourly_pay": 3000
        });

        let response = context
            .authenticated_request(
                test::TestRequest::post().set_json(&payload),
                PROJECTS_ENDPOINT,
            )
            .await;

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let error_body: Value = test::read_body_json(response).await;
        assert_eq!(
            error_body,
            json!({
                "error": "リソースが見つかりません",
                "message": "プロジェクトに関連する企業が見つかりません",
                "code": "NOT_FOUND"
            })
        );
    })
    .await;
}
