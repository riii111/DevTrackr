FROM node:18.17.0

# 作業ディレクトリを設定
WORKDIR /app

# パッケージマネージャーとしてYarnを使用
# Yarnの特定バージョンをインストール
RUN npm install -g yarn@1.22.19

# package.json と yarn.lock をコピー
COPY package.json yarn.lock ./

# 依存関係のインストール
RUN yarn install --frozen-lockfile

# ソースコードをコピー
COPY . .

# アプリケーションをビルド
RUN yarn build

# ポート3000を開放
EXPOSE 3000

# アプリケーションを起動
CMD ["yarn", "start"]