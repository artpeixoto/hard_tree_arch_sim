FROM --platform=x86_64 rust
WORKDIR /usr/app
COPY . .
# RUN cargo build --release
CMD [ "bash" ]