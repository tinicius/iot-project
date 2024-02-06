FROM node AS build

WORKDIR /build
COPY . .
RUN yarn install && yarn start:prod

FROM node:alpine

WORKDIR /app
COPY package.json yarn.lock ./
RUN yarn install --production --frozen-lockfile
COPY --from=build /build/dist ./

EXPOSE 8080

CMD ["node", "src/main.js"]