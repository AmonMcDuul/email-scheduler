Email Scheduler.

Get
curl -s http://localhost:8080/api/messages

Post
curl -X POST -H "Content-Type: application/json" -d "{\"email\": \"email@email.com\", \"message_body\": \"dit is een message yo!\", \"send_at\": \"2024-11-11T12:00:00Z\"}" http://localhost:8080/api/message
