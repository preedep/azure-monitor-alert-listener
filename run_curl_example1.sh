#!/bin/bash

URL="http://localhost:8888/alert"
JSON_FILE="dataset/example3.json"

if [ ! -f "$JSON_FILE" ]; then
  echo "❌ JSON file '$JSON_FILE' not found!"
  exit 1
fi

echo "🚀 Sending alert to $URL"
curl -X POST "$URL" \
  -H "Content-Type: application/json" \
  --data "@$JSON_FILE"

echo -e "\n✅ Done"