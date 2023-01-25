# [Reference](https://dev.to/rogertorres/first-steps-with-docker-rust-30oi)

#
# Dockerfile3/4/5
#

# Rust as the base build image
FROM rust:1.61 as build

# 1. Create a new empty shell project
RUN USER=root cargo new --bin infinityper
WORKDIR /infinityper

# 2. Copy our manifests
COPY ./Cargo.lock ./Cargo.lock 
COPY ./Cargo.toml ./Cargo.toml 

# 3. Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# 4. Now that the dependency is built, copy your source code
COPY ./src ./src

# 5. Build for release.
RUN rm ./target/release/deps/infinityper*
RUN cargo build --release

# # 6. Dockerfile3: Our final base (1.26GB)
# # FROM rust:1.61

# # 6. Dockerfile4: space-saving image variant (675MB)
# # FROM rust:1.61-slim-buster

# 6. Dockerfile5: Linux image without any rust (75.9MB)
FROM debian:buster-slim

# 7. Copy the build artifact from the build stage
COPY --from=build /infinityper/target/release/infinityper .

# 8. Set the startup command to run our binary
CMD ["./infinityper"]

# -------------------------------------------
# #
# # Dockerfile2
# #
#
# # Rust as the base image
# FROM rust:1.61
#
# # 1. Create a new empty shell project
# RUN USER=root cargo new --bin infinityper
# WORKDIR /infinityper
#
# # 2. Copy our manifests
# COPY ./Cargo.lock ./Cargo.lock 
# COPY ./Cargo.toml ./Cargo.toml 
#
# # 3. Build only the dependencies to cache them
# RUN cargo build --release
# RUN rm src/*.rs
#
# # 4. Now that the dependency is built, copy your source code
# COPY ./src ./src
#
# # 5. Build for release.
# RUN rm ./target/release/deps/infinityper*
# RUN cargo install --path .
#
# CMD ["infinityper"]

# -------------------------------------------

# #
# # Dockerfile1
# #
#
# # This tells docker to use the official Rust image
# FROM rust:1.61
#
# # Copy the files in your machine to the Docker image
# COPY ./ ./
#
# # Build your program for release
# RUN cargo build --release 
#
# # Run the binary
# CMD ["/target/release/infinityper"]
#
