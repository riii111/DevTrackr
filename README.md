# DevTrackr
エンジニア用ダッシュボード（作成途中）。作業工数の提出時に使うかも。

## アプリケーション構成

[![Next.js](https://img.shields.io/badge/Next.js-000000?style=for-the-badge&logo=next.js&logoColor=white)](https://nextjs.org/)
[![TypeScript](https://img.shields.io/badge/TypeScript-3178C6?style=for-the-badge&logo=typescript&logoColor=white)](https://www.typescriptlang.org/)
[![shadcn/ui](https://img.shields.io/badge/shadcn%2Fui-000000?style=for-the-badge&logo=shadcnui&logoColor=white)](https://ui.shadcn.com/)
[![Actix Web](https://img.shields.io/badge/Actix_Web-000000?style=for-the-badge&logo=rust&logoColor=white)](https://actix.rs/)
[![Rust](https://img.shields.io/badge/Rust-BD081C?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![MongoDB](https://img.shields.io/badge/MongoDB-47A248?style=for-the-badge&logo=mongodb&logoColor=white)](https://www.mongodb.com/)
[![Traefik](https://img.shields.io/badge/Traefik-24A1C1?style=for-the-badge&logo=traefik&logoColor=white)](https://traefik.io/)

- フロントエンド: Next.js(app router) + shadcn/ui
- バックエンド: actix-web(Rust)
- データベース: MongoDB
- プロキシ: Traefik



## 構築
`make build`

`make up`

## アクセス先
1. http://api.localhost/api-docs/#/
   バックエンド（APIドキュメント）
2. http://db-admin.localhost/
   MongoDB Express（MongoDBのGUI）
3. http://traefik.localhost/
   Traefikのダッシュボード
