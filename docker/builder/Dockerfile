FROM ubuntu:focal

RUN mkdir -p /root/.cargo
COPY entrypoint.sh /entrypoint.sh
COPY config /root/.cargo/config

ENV LLVM_CONFIG_PATH "/usr/bin/llvm-config-10"
ENV PATH "/root/.cargo/bin:${PATH}"
ENV DEBIAN_FRONTEND "noninteractive"
ENV TZ "Asia/Shanghai"
RUN echo $TZ > /etc/timezone

# ===== 国内源 =====
# RUN sed -i s@/archive.ubuntu.com/@/mirrors.aliyun.com/@g /etc/apt/sources.list
# ===== 国内源 =====

RUN apt update
RUN apt install -y --fix-missing \
      linux-generic-hwe-20.04 build-essential cmake clang-10 llvm-10 \
      curl tzdata postgresql-server-dev-12 libsqlite3-dev
RUN rm /etc/localtime && ln -snf /usr/share/zoneinfo/$TZ /etc/localtime
RUN dpkg-reconfigure -f noninteractive tzdata
RUN apt-get clean

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | \
    sh -s -- --default-toolchain nightly -y

# RUN curl -o- https://raw.githubusercontent.com/creationix/nvm/v0.34.0/install.sh | bash
# RUN bash -c ". /root/.nvm/nvm.sh; nvm install v12.16.2; nvm alias default v12.16.2"
ENV RUSTUP_UPDATE_ROOT "https://mirrors.ustc.edu.cn/rust-static/rustup"
ENV RUSTUP_DIST_SERVER "https://mirrors.tuna.tsinghua.edu.cn/rustup"

VOLUME /data
WORKDIR /data

# CTP 共享库
ENV LD_LIBRARY_PATH "/data/ctp/ctp-sys/ctp/v6_3_19/linux/lib:$LD_LIBRARY_PATH"

ENTRYPOINT ["/entrypoint.sh"]
CMD ["bash"]
