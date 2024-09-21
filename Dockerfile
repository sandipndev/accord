FROM clux/muslrust:stable AS build
  COPY . /src
  WORKDIR /src
  RUN SQLX_OFFLINE=true cargo build --locked --bin accorde-server --release --target x86_64-unknown-linux-musl

FROM nixpkgs/nix-flakes:latest
  RUN nix-channel --add https://nixos.org/channels/nixos-unstable nixpkgs
  RUN nix-channel --update
  RUN nix-env -iA nixpkgs.ffmpeg-full nixpkgs.yt-dlp

  COPY --from=build /src/target/x86_64-unknown-linux-musl/release/accorde-server /usr/bin/accorde-server
  RUN mkdir /accorde
  RUN chown -R 1000 /accorde && chmod -R u+w /accorde
  USER 1000
  WORKDIR /accorde
  ENV ACCORDE_HOME /accorde
  CMD ["/usr/bin/accorde-server"]