FROM node:18.17.0-alpine AS base
WORKDIR /app

FROM base AS dependencies
COPY package.json yarn.lock ./
RUN yarn install --frozen-lockfile

FROM base AS development
COPY --from=dependencies /app/node_modules ./node_modules
COPY . .

EXPOSE 3000

CMD ["yarn", "dev"]