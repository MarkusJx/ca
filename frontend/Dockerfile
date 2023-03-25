FROM node:18-alpine as build
WORKDIR /app
COPY . .
RUN npm install
RUN npm run prepare
RUN npm run build

FROM node:18-alpine as prod

WORKDIR /app
COPY . .
RUN npm install --omit=dev
COPY --from=build /app/build build

EXPOSE 3000
ENTRYPOINT ["node", "build"]