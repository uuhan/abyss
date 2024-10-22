FROM ubuntu:focal

ENV DEBIAN_FRONTEND "noninteractive"
ENV TZ "Asia/Shanghai"
RUN echo $TZ > /etc/timezone

RUN sed -i s@/archive.ubuntu.com/@/mirrors.aliyun.com/@g /etc/apt/sources.list
RUN apt update
RUN apt install -y --fix-missing linux-generic-hwe-20.04 tzdata

RUN rm /etc/localtime && ln -snf /usr/share/zoneinfo/$TZ /etc/localtime
RUN dpkg-reconfigure -f noninteractive tzdata
RUN apt-get clean

COPY lib/* /usr/lib/
COPY bin/* /usr/bin/

VOLUME /data
WORKDIR /data

ENTRYPOINT ["/usr/bin/catfish"]
