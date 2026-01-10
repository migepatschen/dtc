FROM debian:13

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt upgrade -y
RUN apt-get install -y --no-install-recommends \
    # base system. psmisc for killall. openssh-client for git over SSH. \
    bash curl wget psmisc ca-certificates lsb-release openssh-client \
    # dev tools. dnsutils for dig. bsdmainutils for hexdump. \
    git make binutils coreutils build-essential pkg-config linux-perf ripgrep \
    zip htop wrk zstd gdb dnsutils jq b3sum gnupg libimage-exiftool-perl bsdmainutils lsof net-tools iproute2 sqlite3 xxd \
    # Rust toolchain prerequisits \
    libfindbin-libs-perl lld mold gcc g++ musl musl-dev musl-tools libc6-dev cmake clang libclang-dev \
    openssl libssl-dev \
    # useful data. mailcap for mimetypes. tzdata for timezones. \
    mailcap tzdata

# Setup dev user
ARG USERNAME=dev
ARG USER_UID=10001
ARG USER_GID=$USER_UID

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/home/${USERNAME}" \
    --shell "/bin/bash" \
    --uid "${USER_UID}" \
    "${USERNAME}"


USER $USERNAME

COPY --chown=$USERNAME bashrc /home/$USERNAME/.bashrc

WORKDIR /home/$USERNAME

# setup git
RUN git config --global push.autoSetupRemote true
RUN git config --global init.defaultBranch main
RUN git config user.name "migepatschen"
RUN git config user.email "13597235+migepatschen@users.noreply.github.com"

# expose ports as needed
# EXPOSE 8000 8001 8002 8003

# setup Rust toolchain
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -y | sh -s -- --default-toolchain stable -y
ENV PATH=/home/$USERNAME/.cargo/bin:$PATH