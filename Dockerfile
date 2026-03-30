FROM rust:latest
ENV DATABASE_URL="postgres://postgres:keyboard_cat@db:5432/postgres"
VOLUME /code
EXPOSE 3000
WORKDIR /code
CMD ["cargo", "run"]
