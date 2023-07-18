FROM rust:1.67

WORKDIR /usr/src/rel-service
COPY . .

RUN cargo install --path .

RUN sudo cp postgres.crt /usr/local/share/ca-certificates/postgres.crt
RUN sudo update-ca-certificates

CMD ["service"]