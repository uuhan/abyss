FROM ubuntu:groovy

COPY entrypoint.sh /entrypoint.sh

ENV DEBIAN_FRONTEND "noninteractive"
ENV TZ "Asia/Shanghai"

RUN echo $TZ > /etc/timezone
# 国内源
# RUN sed -i s@/archive.ubuntu.com/@/mirrors.aliyun.com/@g /etc/apt/sources.list
RUN apt update
RUN apt install -y --fix-missing build-essential clang-10 llvm-10 curl tzdata
RUN rm /etc/localtime && ln -snf /usr/share/zoneinfo/$TZ /etc/localtime
RUN dpkg-reconfigure -f noninteractive tzdata
RUN apt-get clean

RUN curl -o- https://raw.githubusercontent.com/creationix/nvm/v0.34.0/install.sh | bash
RUN ". /root/.nvm/nvm.sh; nvm install v12.16.2; nvm alias default v12.16.2"

VOLUME /data
WORKDIR /data

ENTRYPOINT ["/entrypoint.sh"]
CMD ["bash"]
