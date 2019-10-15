FROM ubuntu:latest as builder

RUN apt-get update -y
RUN apt-get install parallel build-essential libz-dev git wget libterm-readline-gnu-perl -y

WORKDIR /app
RUN wget -O uproc.tgz http://uproc.gobics.de/downloads/uproc/uproc-1.2.0.tar.gz && tar xvf uproc.tgz && cd uproc-1.2.0 && ./configure && make install && rm -rf *
ENV LD_LIBRARY_PATH /usr/local/lib

COPY run_uproc /usr/local/bin

#FROM rust:1.38.0-buster
#FROM rust:latest
#RUN git clone https://github.com/hurwitzlab/uproc.git
#RUN cd uproc/run_uproc && cargo build --release && cp target/release/run_uproc /usr/local/bin

#FROM alpine:latest

#FROM ubuntu:latest
#COPY --from=builder /usr/local/bin /usr/local/bin
#COPY --from=builder /usr/local/lib /usr/local/lib
#ENV LD_LIBRARY_PATH /usr/local/lib

RUN mkdir .parallel && touch .parallel/will-cite
CMD ["run_uproc"]
