FROM docker.io/library/nginx:1.27-alpine
COPY crates/web/public /usr/share/nginx/html
EXPOSE 80
