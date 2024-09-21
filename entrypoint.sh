#!/usr/bin/env bash

export PATH="/root/.nix-profile/bin:$PATH"

pm2 start --name backend /usr/bin/accorde-server
pm2 start --name frontend "cd /app && pnpm run start"

pm2-runtime start all
