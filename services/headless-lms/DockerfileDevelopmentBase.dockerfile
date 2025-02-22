# This dockerfile contains the base dependencies that we need for developing the headless-lms service. We build this manually to in order to effectively cache these build steps.
# You can rebuild this image by running bin/build-dockerfile-development-base in the repo root
# If you want to deploy the new image, you must push it to the image repository with docker push <image-name>
FROM rust:bullseye as dep-builder

RUN apt-get update \
  && apt-get install -yy build-essential git clang cmake libssl-dev zlib1g-dev gcc g++ file sudo \
  && rm -rf /var/lib/apt/lists/*

RUN git clone https://github.com/rui314/mold.git \
  && mkdir mold/build \
  && cd mold/build \
  && git checkout v1.7.0 \
  && cmake -DCMAKE_BUILD_TYPE=Release -DCMAKE_CXX_COMPILER=c++ .. \
  && cmake --build . -j $(nproc)

# Provides a mapping from ip to country
RUN curl https://packages.ipfire.org/79842AA7CDBA7AE3-pub.asc | apt-key add - \
  && echo "deb     https://packages.ipfire.org/location bullseye/" > /etc/apt/sources.list.d/location.list \
  && apt-get update \
  && apt-get install -yy location \
  && rm -rf /var/lib/apt/lists/*

# Fix location export. Ideally we would contribute this back to location but I can't find where they accept patches
COPY location-export.patch /location-export.patch
RUN patch /usr/lib/python3/dist-packages/location/export.py < /location-export.patch && rm /location-export.patch

RUN location update \
  && mkdir -p /ips-to-country \
  && location export --directory /ips-to-country \
  && gzip /ips-to-country/*

FROM rust:bullseye

COPY --from=dep-builder /mold/build /usr/local/bin/

# Switch to use the mold linker for better compile times
# Using workaround described in https://github.com/rui314/mold#how-to-use
RUN mkdir /mold-ld-workaround \
  && ln -s /usr/local/bin/mold /mold-ld-workaround/ld
ENV RUSTFLAGS='-C link-arg=-B/mold-ld-workaround'

RUN apt-get update \
  && apt-get install -yy wait-for-it postgresql-client \
  && rm -rf /var/lib/apt/lists/*

RUN cargo install sqlx-cli --no-default-features --features postgres,rustls && \
  cargo install cargo-watch && \
  cargo install systemfd && \
  cargo install cargo-chef --locked && \
  rustup component add clippy

COPY --from=dep-builder /ips-to-country /ips-to-country

WORKDIR /app

RUN useradd -ms /usr/sbin/nologin user
