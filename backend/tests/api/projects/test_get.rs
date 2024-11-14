use crate::api::projects::helper::{create_test_project, create_test_projects};
use crate::common::test_app::TestApp;
use actix_web::{http::StatusCode, test};
use bson::oid::ObjectId;
use serde_json::{json, Value};
use std::collections::HashSet;

const PROJECTS_ENDPOINT: &str = "/api/projects/";

#[actix_web::test]
async fn test_get_all_projects_success() {
    /*
    プロジェクト一覧取得が成功することを確認するテスト
     */
    TestApp::run_authenticated_test(|context| async move {
        let response = context
            .authenticated_request(test::TestRequest::get(), PROJECTS_ENDPOINT)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
        let body: Value = test::read_body_json(response).await;
        assert!(body.is_array());
    })
    .await;
}

#[actix_web::test]
async fn test_get_all_projects_unauthorized() {
    /*
    認証なしでアクセスした場合は401エラーが返ることを確認するテスト
     */
    TestApp::run_test(|context| async move {
        let response = test::call_service(
            context.service(),
            test::TestRequest::get().uri(PROJECTS_ENDPOINT).to_request(),
        )
        .await;

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    })
    .await;
}

#[actix_web::test]
async fn test_get_all_projects_with_filter() {
    /*
    フィルターによる検索が成功することを確認するテスト
     */
    TestApp::run_authenticated_test(|context| async move {
        // テストデータの作成
        create_test_projects(&context).await;

        // タイトルでの検索
        let response = context
            .authenticated_request(
                test::TestRequest::get().uri(&format!("{}?title={}", PROJECTS_ENDPOINT, "アプリ")),
                "",
            )
            .await;

        assert_eq!(response.status(), StatusCode::OK);
        let body: Value = test::read_body_json(response).await;
        let projects = body.as_array().unwrap();
        assert_eq!(projects.len(), 2); // Webアプリ、モバイルアプリの2件がヒット

        // ステータスでの検索
        let response = context
            .authenticated_request(
                test::TestRequest::get()
                    .uri(&format!("{}?status={}", PROJECTS_ENDPOINT, "Planning")),
                "",
            )
            .await;

        assert_eq!(response.status(), StatusCode::OK);
        let body: Value = test::read_body_json(response).await;
        let projects = body.as_array().unwrap();
        assert_eq!(projects.len(), 1);
        assert_eq!(projects[0]["status"], "Planning");

        // スキルラベルでの検索
        let response = context
            .authenticated_request(
                test::TestRequest::get()
                    .uri(&format!("{}?skill_labels[]={}", PROJECTS_ENDPOINT, "Rust")),
                "",
            )
            .await;

        assert_eq!(response.status(), StatusCode::OK);
        let body: Value = test::read_body_json(response).await;
        let projects = body.as_array().unwrap();
        assert_eq!(projects.len(), 1);
        let skill_labels: HashSet<String> = projects[0]["skill_labels"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap().to_string())
            .collect();
        assert!(skill_labels.contains("Rust"));
    })
    .await;
}

#[actix_web::test]
async fn test_get_all_projects_with_sort() {
    /*
    ソートによる検索が成功することを確認するテスト
     */
    TestApp::run_authenticated_test(|context| async move {
        // テストデータの作成
        create_test_projects(&context).await;

        // タイトルでの昇順ソート
        let response = context
            .authenticated_request(
                test::TestRequest::get()
                    .uri(&format!("{}?sort[]={}", PROJECTS_ENDPOINT, "title:asc")),
                "",
            )
            .await;

        assert_eq!(response.status(), StatusCode::OK);
        let body: Value = test::read_body_json(response).await;
        let projects = body.as_array().unwrap();
        assert!(projects.len() >= 3);

        // タイトルが正しくソートされているか確認
        let titles: Vec<&str> = projects
            .iter()
            .map(|p| p["title"].as_str().unwrap())
            .collect();
        let mut sorted_titles = titles.clone();
        sorted_titles.sort();
        assert_eq!(titles, sorted_titles);

        // 開始日での降順ソート
        let response = context
            .authenticated_request(
                test::TestRequest::get().uri(&format!(
                    "{}?sort[]={}",
                    PROJECTS_ENDPOINT, "start_date:desc"
                )),
                "",
            )
            .await;

        assert_eq!(response.status(), StatusCode::OK);
        let body: Value = test::read_body_json(response).await;
        let projects = body.as_array().unwrap();
        assert!(projects.len() >= 3);

        // 日付が正しくソートされているか確認
        let dates: Vec<&str> = projects
            .iter()
            .map(|p| p["start_date"].as_str().unwrap())
            .collect();
        let mut sorted_dates = dates.clone();
        sorted_dates.sort_by(|a, b| b.cmp(a));
        assert_eq!(dates, sorted_dates);
    })
    .await;
}

#[actix_web::test]
async fn test_get_project_by_id_success() {
    /*
    プロジェクトIDによる取得が成功することを確認するテスト
     */
    TestApp::run_authenticated_test(|context| async move {
        let project_id = create_test_project(&context).await;
        let url = format!("{}{}/", PROJECTS_ENDPOINT, project_id);
        println!("Request URL: {}", url);

        let response = context
            .authenticated_request(test::TestRequest::get(), &url)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
        let body: Value = test::read_body_json(response).await;

        assert!(body.is_object());
        assert_eq!(body["id"], project_id);
        assert_eq!(body["title"], "テストプロジェクト");
    })
    .await;
}

#[actix_web::test]
async fn test_get_project_by_id_unauthorized() {
    /*
    認証なしでアクセスした場合は401エラーが返ることを確認するテスト
     */
    TestApp::run_authenticated_test(|context| async move {
        let project_id = create_test_project(&context).await;

        let response = test::call_service(
            context.service(),
            test::TestRequest::get()
                .uri(&format!("{}{}/", PROJECTS_ENDPOINT, project_id))
                .to_request(),
        )
        .await;

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    })
    .await;
}

#[actix_web::test]
async fn test_get_project_by_id_not_found() {
    /*
    存在しないプロジェクトIDの場合は404エラーが返ることを確認するテスト
     */
    TestApp::run_authenticated_test(|context| async move {
        let non_existent_id = ObjectId::new();
        let url = format!("{}{}/", PROJECTS_ENDPOINT, non_existent_id);
        println!("Request URL: {}", url);

        let response = context
            .authenticated_request(test::TestRequest::get(), &url)
            .await;

        let response_body = test::read_body(response).await;
        println!(
            "Raw response body: {:?}",
            String::from_utf8_lossy(&response_body)
        );

        assert!(!response_body.is_empty(), "Response body is empty");

        let error_body: Value =
            serde_json::from_slice(&response_body).expect("Failed to parse response body as JSON");

        assert_eq!(
            error_body,
            json!({
                "error": "リソースが見つかりません",
                "message": "プロジェクトが見つかりません",
                "code": "NOT_FOUND"
            })
        );
    })
    .await;
}

#[actix_web::test]
async fn test_get_project_by_id_invalid_id() {
    /*
    無効なIDフォーマットの場合は400エラーが返ることを確認するテスト
     */
    TestApp::run_authenticated_test(|context| async move {
        let url = format!("{}{}/", PROJECTS_ENDPOINT, "invalid-id");
        println!("Request URL: {}", url);

        let response = context
            .authenticated_request(test::TestRequest::get(), &url)
            .await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let error_body: Value = test::read_body_json(response).await;
        assert_eq!(
            error_body,
            json!({
                "error": "不正なリクエスト",
                "message": "無効なIDです",
                "code": "BAD_REQUEST"
            })
        );
    })
    .await;
}
