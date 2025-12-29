#!/bin/bash

DOMAIN="$DOMAIN_NAME"
EMAIL="$EMAIL_ADDRESS"
mkdir -p .certbot_temp/config .certbot_temp/work .certbot_temp/logs
~/certbot-work/venv/bin/certbot certonly \
  --dns-cloudflare \
  --dns-cloudflare-credentials ~/.secrets/certbot/cloudflare.ini \
  --config-dir ./.certbot_temp/config \
  --work-dir ./.certbot_temp/work \
  --logs-dir ./.certbot_temp/logs \
  -d "$DOMAIN" \
  --email "$EMAIL" \
  --agree-tos --no-eff-email
cp -L ./.certbot_temp/config/live/$DOMAIN/fullchain.pem ./crates/ddns-server/certs
cp -L ./.certbot_temp/config/live/$DOMAIN/privkey.pem ./crates/ddns-server/certs
rm -rf .certbot_temp
