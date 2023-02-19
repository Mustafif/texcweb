FROM rust:latest
WORKDIR /home
COPY . .
RUN cargo build texcweb --release
EXPOSE 8000
CMD ["target/release/texcweb"]