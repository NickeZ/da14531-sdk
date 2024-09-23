#!/usr/bin/env sh

gpg --batch --passphrase "${SDK_ENC_KEY}" --symmetric --cipher-algo AES256 --output vendor/SDK_6.0.22.1401.zip.gpg vendor/SDK_6.0.22.1401.zip
