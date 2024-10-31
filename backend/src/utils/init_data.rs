// use reqwest::Client;
// use serde_json::{json, Value};
// use std::error::Error;
// use std::fs::File;
// use std::io::Read;

// const BASE_URL: &str = "http://localhost/api";

// // TODO: 途中まで。未適用
// async fn load_fixture_data() -> Result<Value, Box<dyn Error>> {
//     let mut file = File::open("local_fixture.json")?;
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?;
//     let data: Value = serde_json::from_str(&contents)?;
//     Ok(data)
// }

// async fn register_and_login(client: &Client) -> Result<String, Box<dyn Error>> {
//     let register_url = format!("{}/auth/register/", BASE_URL);
//     let login_url = format!("{}/auth/login/", BASE_URL);

//     // テストユーザーの登録
//     let user_data = json!({
//         "email": "test@example.com",
//         "password": "testpassword123",
//         "username": "testuser"
//     });

//     let response = client.post(&register_url).json(&user_data).send().await?;
//     if response.status() != 201 {
//         println!("ユーザー登録に失敗しました。Status code: {}", response.status());
//         return Err("ユーザー登録失敗".into());
//     }

//     // ログイン
//     let login_data = json!({
//         "email": "test@example.com",
//         "password": "testpassword123"
//     });

//     let response = client.post(&login_url).json(&login_data).send().await?;
//     if response.status() != 200 {
//         println!("ログインに失敗しました。Status code: {}", response.status());
//         return Err("ログイン失敗".into());
//     }

//     // アクセストークンを取得
//     let access_token = response.headers()
//         .get("Set-Cookie")
//         .and_then(|cookie| cookie.to_str().ok())
//         .and_then(|cookie_str| cookie_str.split(';').next())
//         .and_then(|cookie_value| cookie_value.strip_prefix("access_token="))
//         .ok_or("アクセストークンを取得できませんでした")?
//         .to_string();

//     Ok(access_token)
// }

// async fn insert_companies(client: &Client, companies: &[Value], token: &str) -> Result<std::collections::HashMap<String, String>, Box<dyn Error>> {
//     let api_url = format!("{}/companies/", BASE_URL);
//     let mut company_id_map = std::collections::HashMap::new();

//     for company in companies {
//         let company_name = company["company_name"].as_str().unwrap();
//         let response = client.post(&api_url)
//             .header("Authorization", format!("Bearer {}", token))
//             .json(company)
//             .send()
//             .await?;

//         if response.status() == 201 {
//             let company_id = response.json::<Value>().await?["id"].as_str().unwrap().to_string();
//             company_id_map.insert(company_name.to_string(), company_id);
//             println!("Successfully inserted company: {}", company_name);
//         } else {
//             println!("Failed to insert company: {}. Status code: {}", company_name, response.status());
//         }
//     }

//     Ok(company_id_map)
// }

// async fn insert_projects(client: &Client, projects: &[Value], company_id_map: &std::collections::HashMap<String, String>, token: &str) -> Result<(), Box<dyn Error>> {
//     let api_url = format!("{}/projects/", BASE_URL);

//     for mut project in projects.to_vec() {
//         let company_name = project["company_name"].as_str().unwrap();
//         if let Some(company_id) = company_id_map.get(company_name) {
//             project["company_id"] = json!(company_id);
//             project.as_object_mut().unwrap().remove("company_name");

//             let response = client.post(&api_url)
//                 .header("Authorization", format!("Bearer {}", token))
//                 .json(&project)
//                 .send()
//                 .await?;

//             if response.status() == 201 {
//                 println!("Successfully inserted project: {}", project["title"]);
//             } else {
//                 println!("Failed to insert project: {}. Status code: {}", project["title"], response.status());
//             }
//         } else {
//             println!("Company not found for project: {}. Skipping.", project["title"]);
//         }
//     }

//     Ok(())
// }

// pub async fn initialize_data() -> Result<(), Box<dyn Error>> {
//     let client = Client::new();

//     let token = register_and_login(&client).await?;
//     let fixture_data = load_fixture_data().await?;

//     let companies = fixture_data["companies"].as_array().unwrap();
//     // let projects = fixture_data["projects"].as_array().unwrap();

//     let company_id_map = insert_companies(&client, companies, &token).await?;
//     // insert_projects(&client, projects, &company_id_map, &token).await?;

//     println!("Data initialization completed.");
//     Ok(())
// }
