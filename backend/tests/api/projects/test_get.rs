use crate::api::projects::helper::{create_test_project, create_test_projects};
use crate::common::test_app::TestApp;
use actix_web::{http::StatusCode, test};
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde_json::{json, Value};
use std::collections::HashSet;
use url::form_urlencoded;

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

        // タイトルでの検索（URLエンコード）
        let encoded_title =
            form_urlencoded::byte_serialize("アプリ".as_bytes()).collect::<String>();
        let request_url = format!("{}?title={}", PROJECTS_ENDPOINT, encoded_title);

        let response = context
            .authenticated_request(
                test::TestRequest::get(), // URIを設定しない
                &request_url,             // クエリパラメータ付きのURLを渡す
            )
            .await;

        assert_eq!(response.status(), StatusCode::OK);
        let body: Value = test::read_body_json(response).await;
        let projects = body.as_array().unwrap();

        assert_eq!(projects.len(), 2);

        // プロジェクト名の確認
        let titles: Vec<&str> = projects
            .iter()
            .map(|p| p["title"].as_str().unwrap())
            .collect();
        assert!(titles.contains(&"Webアプリケーション開発"));
        assert!(titles.contains(&"モバイルアプリ開発"));

        // ステータスでの検索
        let status_url = format!("{}?status={}", PROJECTS_ENDPOINT, "Planning");
        let response = context
            .authenticated_request(test::TestRequest::get(), &status_url)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
        let body: Value = test::read_body_json(response).await;
        let projects = body.as_array().unwrap();
        assert_eq!(projects.len(), 2);
        for project in projects.iter() {
            assert_eq!(project["status"], "Planning");
        }

        // スキルラベルでの検索
        let skills_url = format!("{}?skill_labels={}", PROJECTS_ENDPOINT, "Rust");
        let response = context
            .authenticated_request(test::TestRequest::get(), &skills_url)
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
        let sort_title_url = format!("{}?sort={}", PROJECTS_ENDPOINT, "title:asc");
        println!("Request URL: {}", sort_title_url);

        let response = context
            .authenticated_request(test::TestRequest::get(), &sort_title_url)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
        let body: Value = test::read_body_json(response).await;
        println!(
            "Response body: {}",
            serde_json::to_string_pretty(&body).unwrap()
        );

        let projects = body.as_array().unwrap();
        assert!(projects.len() >= 3);

        // タイトルが正しくソートされているか確認
        let titles: Vec<&str> = projects
            .iter()
            .map(|p| p["title"].as_str().unwrap())
            .collect();

        // 期待される順序を明示的に確認
        assert_eq!(
            titles,
            vec![
                "Webアプリケーション開発",
                "インフラ構築",
                "モバイルアプリ開発"
            ]
        );

        // 作成日時での降順ソート
        let sort_date_url = format!("{}?sort={}", PROJECTS_ENDPOINT, "created_at:desc");
        let response = context
            .authenticated_request(test::TestRequest::get(), &sort_date_url)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
        let body: Value = test::read_body_json(response).await;
        let projects = body.as_array().unwrap();

        // 日付が正しくソートされているか確認
        let dates: Vec<DateTime<Utc>> = projects
            .iter()
            .map(|p| {
                DateTime::parse_from_rfc3339(p["created_at"].as_str().unwrap())
                    .unwrap()
                    .with_timezone(&Utc)
            })
            .collect();

        // 日付の降順を確認
        for i in 1..dates.len() {
            assert!(
                dates[i - 1] >= dates[i],
                "Dates not properly sorted: {:?} should be >= {:?}",
                dates[i - 1],
                dates[i]
            );
        }
    })
    .await;
}

#[actix_web::test]
async fn test_get_project_by_id_success() {
    /*
    プロジェクトIDによる取得が成功することを確認するテスト
     */
    TestApp::run_authenticated_test(|context| async move {
        let test_project = create_test_project(&context).await;
        let url = format!("{}{}/", PROJECTS_ENDPOINT, test_project.id);

        let response = context
            .authenticated_request(test::TestRequest::get(), &url)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
        let body: Value = test::read_body_json(response).await;

        assert!(body.is_object());
        assert_eq!(body["id"], test_project.id);
        assert_eq!(body["company_id"], test_project.company_id);
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
        let test_project = create_test_project(&context).await;

        let response = test::call_service(
            context.service(),
            test::TestRequest::get()
                .uri(&format!("{}{}/", PROJECTS_ENDPOINT, test_project.id))
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

#[actix_web::test]
async fn test_get_all_projects_with_multiple_filters() {
    /*
    複数条件での検索が成功することを確認するテスト
     */
    TestApp::run_authenticated_test(|context| async move {
        // テストデータの作成
        create_test_projects(&context).await;

        // 複数条件での検索（ステータスとスキルラベル）
        let url = format!(
            "{}?status={}&skill_labels={}",
            PROJECTS_ENDPOINT, "Planning", "React"
        );
        let response = context
            .authenticated_request(test::TestRequest::get(), &url)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
        let body: Value = test::read_body_json(response).await;
        let projects = body.as_array().unwrap();

        for project in projects.iter() {
            assert_eq!(project["status"], "Planning");
            let skills = project["skill_labels"].as_array().unwrap();
            assert!(skills.contains(&json!("React")));
        }
    })
    .await;
}

#[actix_web::test]
async fn test_get_all_projects_with_pagination() {
    /*
    ページネーションが成功することを確認するテスト
     */
    TestApp::run_authenticated_test(|context| async move {
        // 2セット分のテストデータを作成（計6件）
        create_test_projects(&context).await;
        create_test_projects(&context).await;

        // ケース1: 最初の2件を取得
        let url = format!("{}?limit={}&offset={}", PROJECTS_ENDPOINT, 2, 0);
        let response = context
            .authenticated_request(test::TestRequest::get(), &url)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
        let body: Value = test::read_body_json(response).await;
        let projects = body.as_array().unwrap();
        assert_eq!(projects.len(), 2, "First page should contain 2 items");

        // ケース2: 次の2件を取得
        let url = format!("{}?limit={}&offset={}", PROJECTS_ENDPOINT, 2, 2);
        let response = context
            .authenticated_request(test::TestRequest::get(), &url)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
        let body: Value = test::read_body_json(response).await;
        let projects = body.as_array().unwrap();
        assert_eq!(projects.len(), 2, "Second page should contain 2 items");

        // ケース3: 最後のページ（残り2件）
        let url = format!("{}?limit={}&offset={}", PROJECTS_ENDPOINT, 2, 4);
        let response = context
            .authenticated_request(test::TestRequest::get(), &url)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
        let body: Value = test::read_body_json(response).await;
        let projects = body.as_array().unwrap();
        assert_eq!(projects.len(), 2, "Last page should contain 2 items");

        // ケース4: 範囲外のオフセット
        let url = format!("{}?limit={}&offset={}", PROJECTS_ENDPOINT, 2, 10);
        let response = context
            .authenticated_request(test::TestRequest::get(), &url)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
        let body: Value = test::read_body_json(response).await;
        let projects = body.as_array().unwrap();
        assert_eq!(projects.len(), 0, "Out of range offset should return empty array");

        // ケース5: limitとoffsetを組み合わせてソート順の一貫性を確認
        let url = format!(
            "{}?limit={}&offset={}&sort={}",
            PROJECTS_ENDPOINT, 2, 0, "title:asc"
        );
        let response = context
            .authenticated_request(test::TestRequest::get(), &url)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
        let body: Value = test::read_body_json(response).await;
        let first_page = body.as_array().unwrap();
        let first_page_titles: Vec<&str> = first_page
            .iter()
            .map(|p| p["title"].as_str().unwrap())
            .collect();

        // 2ページ目を取得
        let url = format!(
            "{}?limit={}&offset={}&sort={}",
            PROJECTS_ENDPOINT, 2, 2, "title:asc"
        );
        let response = context
            .authenticated_request(test::TestRequest::get(), &url)
            .await;

        let body: Value = test::read_body_json(response).await;
        let second_page = body.as_array().unwrap();
        let second_page_titles: Vec<&str> = second_page
            .iter()
            .map(|p| p["title"].as_str().unwrap())
            .collect();

        // ページをまたいでもソート順が維持されていることを確認
        assert!(
            first_page_titles[first_page_titles.len() - 1] <= second_page_titles[0],
            "Sort order should be maintained across pages. Last item of first page: {:?}, First item of second page: {:?}",
            first_page_titles[first_page_titles.len() - 1],
            second_page_titles[0]
        );
    })
    .await;
}

#[actix_web::test]
async fn test_get_all_projects_with_multiple_sorts() {
    /*
    複数条件でのソートが成功することを確認するテスト
     */
    TestApp::run_authenticated_test(|context| async move {
        create_test_projects(&context).await;

        // 複数条件でのソート（カンマ区切りで指定）
        let url = format!(
            "{}?sort={}",
            PROJECTS_ENDPOINT,
            "status:asc,created_at:desc" // カンマ区切りで指定
        );
        let response = context
            .authenticated_request(test::TestRequest::get(), &url)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
        let body: Value = test::read_body_json(response).await;
        let projects = body.as_array().unwrap();

        // ステータスと作成日時の順序を確認
        let mut prev_status = "";
        let mut prev_date = "";
        for project in projects.iter() {
            let current_status = project["status"].as_str().unwrap();
            let current_date = project["created_at"].as_str().unwrap();

            if !prev_status.is_empty() {
                // ステータスが同じ場合は日付の降順をチェック
                if prev_status == current_status {
                    // 日付文字列を DateTime に変換して比較
                    let prev_datetime = DateTime::parse_from_rfc3339(prev_date).unwrap();
                    let current_datetime = DateTime::parse_from_rfc3339(current_date).unwrap();
                    assert!(
                        prev_datetime >= current_datetime,
                        "Dates not properly sorted within same status. Previous: {}, Current: {}",
                        prev_date,
                        current_date
                    );
                } else {
                    assert!(prev_status <= current_status);
                }
            }

            prev_status = current_status;
            prev_date = current_date;
        }
    })
    .await;
}

#[actix_web::test]
async fn test_get_all_projects_with_invalid_params() {
    /*
    無効なパラメータの場合は無視され、400エラーが返ることを確認するテスト
    （検索ワードは無視して良いが、sort, limit, offsetはプログラム上のエラーとなりうる）
     */
    TestApp::run_authenticated_test(|context| async move {
        // 無効なソート順
        let url = format!("{}?sort={}", PROJECTS_ENDPOINT, "title:invalid");
        let response = context
            .authenticated_request(test::TestRequest::get(), &url)
            .await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        // 無効なlimit値
        let url = format!("{}?limit={}", PROJECTS_ENDPOINT, -1);
        let response = context
            .authenticated_request(test::TestRequest::get(), &url)
            .await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        // 無効なoffset値
        let url = format!("{}?offset={}", PROJECTS_ENDPOINT, "invalid");
        let response = context
            .authenticated_request(test::TestRequest::get(), &url)
            .await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    })
    .await;
}
