FROM rust:1.38.0-buster as rust
RUN git clone https://github.com/hurwitzlab/uproc.git
RUN cd uproc/run_uproc && cargo build --release && cp target/release/run_uproc /usr/local/bin

WORKDIR /app
RUN wget -O uproc.tgz http://uproc.gobics.de/downloads/uproc/uproc-1.2.0.tar.gz && tar xvf uproc.tgz && cd uproc-1.2.0 && ./configure && make install && rm -rf *

FROM ubuntu:latest
RUN apt-get update -y
RUN apt-get install -y parallel libgomp1
RUN mkdir .parallel && touch .parallel/will-cite
COPY --from=rust /usr/local/bin /usr/local/bin
COPY --from=rust /usr/local/lib /usr/local/lib
ENV LD_LIBRARY_PATH /usr/local/lib

CMD ["run_uproc"]
