FROM rust:1.74
RUN apt-get update -yqq && apt-get install -yqq cmake g++

COPY . .

WORKDIR .

RUN cargo clean
RUN cargo build --release
RUN cp ./target/release/warkawik ./warkawik
RUN rm -rf ./target 
RUN rm -rf ./src
RUN rm -rf ./config.yml
RUN chmod +x ./warkawik  

EXPOSE 3030

CMD ["./warkawik"]
