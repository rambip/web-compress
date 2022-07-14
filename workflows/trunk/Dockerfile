FROM osomahe/rust-trunk:22.05 

RUN mkdir /project
WORKDIR /project


COPY ./entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]
