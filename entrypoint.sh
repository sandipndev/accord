#!/usr/bin/env bash

echo "[ ---------> ] Starting accorde-server"
/usr/bin/accorde-server &

echo "[ ---------> ] Starting accorde-frontend"
cd /app && pnpm run start
