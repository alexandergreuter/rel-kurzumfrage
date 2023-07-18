FROM rust:1.67

WORKDIR /usr/src/rel-service
COPY . .

RUN cargo install --path .

CMD ["service"]