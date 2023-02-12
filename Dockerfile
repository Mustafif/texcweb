FROM rust:latest
WORKDIR .
COPY . .
RUN cargo build --release
EXPOSE 8000
CMD ["target/release/texcweb"]