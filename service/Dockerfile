FROM rust:1.67

WORKDIR /usr/src/rel-service
COPY . .

RUN cargo install --path .

RUN cp postgres.crt /usr/local/share/ca-certificates/postgres.crt
RUN update-ca-certificates

CMD ["service"]