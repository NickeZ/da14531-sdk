#!/usr/bin/env sh

gpg --batch --passphrase "${SDK_ENC_KEY}" --decrypt --output vendor/SDK_6.0.22.1401.zip vendor/SDK_6.0.22.1401.zip.gpg
