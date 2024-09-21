# Backend build stage
FROM clux/muslrust:stable AS build-backend
  COPY . /src
  WORKDIR /src
  RUN SQLX_OFFLINE=true cargo build --locked --bin accorde-server --release --target x86_64-unknown-linux-musl

# Frontend build stage
FROM node:20-alpine AS build-frontend
  RUN apk add --no-cache libc6-compat
  WORKDIR /app
  RUN corepack enable pnpm
  COPY frontend .
  RUN pnpm install --frozen-lockfile
  RUN pnpm run build

FROM nixpkgs/nix-flakes:latest
  RUN nix-channel --add https://nixos.org/channels/nixos-unstable nixpkgs
  RUN nix-channel --update

  # ffmpeg and yt-dlp are command line utilties being used often
  RUN nix-env -iA nixpkgs.ffmpeg-full nixpkgs.yt-dlp

  COPY --from=build-backend /src/target/x86_64-unknown-linux-musl/release/accorde-server /usr/bin/accorde-server

  WORKDIR /app
  COPY --from=build-frontend /app/package.json /app/pnpm-lock.yaml ./
  COPY --from=build-frontend /app/.next ./.next
  COPY --from=build-frontend /app/public ./public
  COPY --from=build-frontend /app/next.config.js ./

  USER 1000
  WORKDIR /accorde
  RUN mkdir /accorde
  RUN chown -R 1000 /accorde && chmod -R u+w /accorde
  ENV ACCORDE_HOME /accorde

  COPY entrypoint.sh /entrypoint.sh
  RUN chmod +x /entrypoint.sh

  EXPOSE 3000 8765
  CMD ["/entrypoint.sh"]