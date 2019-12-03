FROM rustlang/rust:nightly
WORKDIR /code

ENV ROCKET_PORT 8000
EXPOSE 8000

COPY . .
CMD ["cargo", "build"]
CMD ["cargo", "run"]