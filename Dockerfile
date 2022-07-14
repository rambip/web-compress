FROM osomahe/rust-trunk:22.05 

RUN mkdir /project
WORKDIR /project
ENTRYPOINT ["/root/.cargo/bin/trunk"]
