#!/bin/bash

# Exit immediately if a command exits with a non-zero status.
set -e

GEMINI_CRED_FILE="$HOME/.zos/oauth_creds.json"

echo "--- Testing Gemini OAuth Login ---"

if [ -f "$GEMINI_CRED_FILE" ]; then
  echo "Warning: Existing credentials file found at $GEMINI_CRED_FILE. It will be overwritten if OAuth is successful."
fi

rm -f "$GEMINI_CRED_FILE"

# 2. Initiate OAuth: Run aichat auth login in the background
echo "Initiating Gemini OAuth login flow..."
echo "Please complete the authentication in your browser."
cargo run -- auth login &

# Get the PID of the background process
OAUTH_PID=$!

# 3. Instruct User and Wait: Give user time to complete browser flow
echo "Waiting for 30 seconds for you to complete the browser authentication..."
echo "If the browser doesn't open automatically, please check the terminal for a URL."
sleep 60

# 4. Kill the background process if it's still running
if ps -p $OAUTH_PID > /dev/null; then
  echo "Killing background OAuth process (PID: $OAUTH_PID)..."
  kill $OAUTH_PID
  sleep 1 # Give it a moment to terminate
fi

# 5. Verify: Check if the credentials file exists and contains tokens
if [ -f "$GEMINI_CRED_FILE" ]; then
  if grep -q "access_token" "$GEMINI_CRED_FILE" && grep -q "refresh_token" "$GEMINI_CRED_FILE"; then
    echo "Gemini OAuth login test: SUCCESS! Credentials file created and contains tokens."
  else
    echo "Gemini OAuth login test: FAILED! Credentials file found but does not contain expected tokens."
    exit 1
  fi
else
  echo "Gemini OAuth login test: FAILED! Credentials file not found at $GEMINI_CRED_FILE"
  exit 1
fi

echo "--- Gemini OAuth Test Completed ---"