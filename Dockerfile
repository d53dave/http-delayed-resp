FROM scratch
COPY target/x86_64-unknown-linux-musl/release/http-delayed-resp /http-delayed-resp
CMD ["/http-delayed-resp"]
