FROM rust:1.82

COPY . .

RUN cargo build --release

EXPOSE 8080

CMD ["./target/release/blogpost_web"]