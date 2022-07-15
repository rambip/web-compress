FROM osomahe/rust-trunk:22.05 

RUN mkdir /project
ENV PROJECT=/project
WORKDIR /project

RUN mkdir /cache
ENV CACHE=/cache 

COPY ./entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]
