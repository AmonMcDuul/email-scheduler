Email Scheduler.

add config.toml for secrets.

    ######################################

    [credentials]
    username = "your-email@gmail.com"
    password = "your-app-password"

    ######################################

Get
curl -s http://localhost:8080/api/messages

Post
curl -X POST -H "Content-Type: application/json" -d "{\"email\": \"email@email.com\", \"message_body\": \"dit is een message yo!\", \"send_at\": \"2024-01-17T19:52:06.515205Z\"}" http://localhost:8080/api/message
