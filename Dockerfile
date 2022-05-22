FROM rust:1.61.0 as builder

# First, an empty Rust project is created using cargo 
# and then all dependencies (in the form of Cargo.toml)
# are copied into that project.
RUN USER=root cargo new --bin service
WORKDIR /service

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
# Then a release-build is triggered before removing 
# everything in the src folder. With Docker, for each 
# command inside the Dockerfile, a layer is created. 
RUN cargo build --release
RUN rm src/*.rs 

# A after removing all files in src, copy all files of 
# the project into the Docker working directory, remove 
# the binary built from the dependencies, and trigger 
# another release build — in this case, with the whole code.
ADD . ./

RUN rm ./target/release/deps/service*
RUN cargo build --release


FROM debian:buster-slim
ARG APP=/usr/src/app

# install some required packages using apt, ca-certificates and 
# tzdata, which is a good baseline for web applications.
RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8080

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /service/target/release/service ${APP}/service

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./service"]