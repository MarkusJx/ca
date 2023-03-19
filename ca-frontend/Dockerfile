FROM node:18 as build
WORKDIR /app
COPY . .
RUN npm install
RUN npm run build

FROM nginx:1-alpine as prod

WORKDIR /usr/share/nginx/html
COPY --from=build /app/build .
ENTRYPOINT ["nginx", "-g", "daemon off;"]