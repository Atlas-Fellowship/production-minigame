#!/bin/bash

./target/debug/production-minigame-service \
  --port=8080 \
  --database-url=postgres://postgres:toor@localhost/production_minigame \
  --auth-service-url=http://localhost:8079 \
  --site-external-url=http://localhost:3000 \
