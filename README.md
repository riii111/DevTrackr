# DevTrackr
エンジニア用ダッシュボード（作成途中）。

## アプリケーション構成
- フロントエンド: Next.js(app router) + shadcn/ui
- バックエンド: actix-web(Rust)
- データベース: MongoDB
- プロキシ: Traefik

## 構築
`make build`

`make up`

## アクセス先
1. http://api.localhost/
   バックエンド
2. http://db-admin.localhost/
   MongoDB Express（MongoDBのGUI）
3. http://traefik.localhost/
   Traefikのダッシュボード
