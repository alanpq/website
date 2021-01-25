FROM rust:1.48

WORKDIR /website
RUN chmod -R +rwx .
COPY dummy.rs .
COPY Cargo.toml .
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN cargo build --release
RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml
COPY . .
RUN chmod -R +rwx .

RUN cargo install --path .

EXPOSE 8080
CMD ["website"]