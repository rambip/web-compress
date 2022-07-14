FROM osomahe/rust-trunk:22.05 

RUN mkdir /project
WORKDIR /project

RUN ls -la /root

RUN echo $HOME
RUN ls -la $HOME

RUN cargo install --list

ENTRYPOINT ["/root/.cargo/bin/trunk"]
