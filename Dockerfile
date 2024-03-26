FROM rust:1.74.1

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=3005
# ENV ELASTICSEARCH_URL=http://elastic:password@search-db:9200
ENV ELASTICSEARCH_URL=http://search-db:9200
ENV ES_USERNAME=elastic
ENV ES_PASSWORD=password

WORKDIR /app
COPY . .

RUN rustup default nightly
RUN cargo build

CMD ["cargo", "run"]