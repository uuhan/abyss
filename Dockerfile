FROM ubuntu:groovy

VOLUME /data
WORKDIR /data

ENV DEBIAN_FRONTEND "noninteractive"
ENV TZ "Asia/Shanghai"

RUN echo $TZ > /etc/timezone
RUN sed -i s@/archive.ubuntu.com/@/mirrors.aliyun.com/@g /etc/apt/sources.list
RUN apt update
RUN apt install -y tzdata libsqlite3-dev postgresql-server-dev-12
RUN rm /etc/localtime && ln -snf /usr/share/zoneinfo/$TZ /etc/localtime
RUN dpkg-reconfigure -f noninteractive tzdata
RUN apt-get clean

COPY ctp/ctp-sys/ctp/v6_3_19/lib/* /lib/
COPY docker/builder/entrypoint.sh /
COPY target/debug/market /bin/
COPY target/debug/position /bin/

VOLUME /data
WORKDIR /data

# ENTRYPOINT ["/entrypoint.sh", "env", "RUST_LOG=debug", "RUST_BACKTRACE=1 ", "/bin/market"]
ENTRYPOINT ["/entrypoint.sh"]
CMD ["bash"]
