# Spring Boot Metrics

app optics

management.metrics.export.appoptics.api-token=YOUR_TOKEN

```java
management:
  metrics:
    export:
      appoptics:
        api-token: ${APP_OPTICS_SERVICE_KEY:}
```

via actuator

/actuator/metrics

```java
management:
  endpoints:
    web:
      exposure:
        include: health, info, metrics
```