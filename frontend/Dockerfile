FROM node:18.17.0

# 作業ディレクトリを設定
WORKDIR /app

# ソースコードをコピー
COPY . .

# アプリケーションをインストール&ビルド
RUN yarn install
RUN yarn build

# ポート3000を開放
EXPOSE 3000

# アプリケーションを起動
CMD ["yarn", "start"]