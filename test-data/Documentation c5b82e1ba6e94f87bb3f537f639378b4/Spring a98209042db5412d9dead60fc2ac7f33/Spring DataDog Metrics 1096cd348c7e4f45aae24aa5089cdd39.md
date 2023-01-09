# Spring DataDog Metrics

This is what we're using inside the app:

[57. Metrics](https://docs.spring.io/spring-boot/docs/current/reference/html/production-ready-metrics.html#production-ready-metrics-getting-started)

Here's the configuration that activates Spring Boot → Datadog integration

In `application-ecs.yml`:

```jsx
# ECS spring properties, automatically enabled by 'entrypoint.sh' if it detects ECS
management:
  # Metrics - enable basic metrics, enable datadog in the ECS profile.
  metrics:
    export:
      statsd:
        enabled: true
        flavor: datadog
        host: 172.17.0.1
```

How is the datadog client loaded?

- The autoconfiguration built in to spring boot will see that metrics are enabled and automatically start the appropriate components.

What's with the weird IP address?

- Each ECS Instance (host) in the ECS cluster has a DataDog agent container running.
- This container is listening on the statsd port on the ECS Instance (host).   Other containers on the host can access it using the special ECS host IP address: `172.17.0.1`
- The agent has our DataDog account key, and it collects metrics from the statsd port and sends them off to DataDog's servers.