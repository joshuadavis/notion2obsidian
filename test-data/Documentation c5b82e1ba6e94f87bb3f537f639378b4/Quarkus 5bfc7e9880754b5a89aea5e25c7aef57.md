# Quarkus

# Building Native Images

## Tuning Docker Memory

Simple 'hello world' app - Gradle, Kotlin, JAX-RS, Obs, Health, SwaggerUI

```bash
./gradlew clean build -Dquarkus.package.type=native -Dquarkus.native.container-build=true
```

2G Docker memory: 7m 9s

4G Docker memory

- First time 3m 26s, 3.85G high water mark
- Second time 3m 27s, 3.85G high water mark

6G Docker memory

- 2m 51s, 5.2G high water mark
- 2m 52s

8G Docker memory

- 2m 48s, 6.68G high water mark
- 2m 40s