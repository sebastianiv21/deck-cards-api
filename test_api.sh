#!/bin/bash

echo "🧪 Testing Deck Cards API"
echo "========================="

BASE_URL="http://localhost:3000"

echo "1. Health check..."
curl -s "$BASE_URL/health" | jq .

echo -e "\n2. Create a deck..."
DECK_RESPONSE=$(curl -s -X POST "$BASE_URL/decks" \
  -H "Content-Type: application/json" \
  -d '{"name": "Spanish Vocabulary", "description": "Basic Spanish words"}')
echo $DECK_RESPONSE | jq .
DECK_ID=$(echo $DECK_RESPONSE | jq -r .id)

echo -e "\n3. Create cards for the deck..."
curl -s -X POST "$BASE_URL/cards" \
  -H "Content-Type: application/json" \
  -d "{\"question\": \"What is hello in Spanish?\", \"answer\": \"Hola\", \"deck_id\": $DECK_ID}" | jq .

curl -s -X POST "$BASE_URL/cards" \
  -H "Content-Type: application/json" \
  -d "{\"question\": \"What is goodbye in Spanish?\", \"answer\": \"Adiós\", \"deck_id\": $DECK_ID}" | jq .

echo -e "\n4. Get all decks..."
curl -s "$BASE_URL/decks" | jq .

echo -e "\n5. Get cards from specific deck..."
curl -s "$BASE_URL/decks/$DECK_ID/cards" | jq .

echo -e "\n6. Update a deck..."
curl -s -X PUT "$BASE_URL/decks/$DECK_ID" \
  -H "Content-Type: application/json" \
  -d '{"name": "Spanish Vocabulary - Updated"}' | jq .

echo -e "\n7. Get all cards..."
curl -s "$BASE_URL/cards" | jq .

echo -e "\n✅ API Testing Complete!"
