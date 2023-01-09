# Wiremock

Docker compose

```bash
wiremock:
    image: rodolpheche/wiremock
    ports:
      - 8088:8080
    volumes:
      - ./wiremock:/home/wiremock
    command:
      - --verbose
    # Go to  http://localhost:8080/__admin/recorder to start recording something.
```