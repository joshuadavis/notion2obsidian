# Spring Boot Notes

# General docos

[Spring Guides](https://spring.io/guides)

# Debug health check endpoint

```yaml
management:
  endpoint:
    health:
      show-details: always  # Enable health check debugging info
```

# Info endpoint

[https://mrhaki.blogspot.com/2016/12/spring-sweets-add-git-info-to-info.html](https://mrhaki.blogspot.com/2016/12/spring-sweets-add-git-info-to-info.html)

[Injecting Git Information Into Spring Beans | Baeldung](https://www.baeldung.com/spring-git-information)

[Custom Information in Spring Boot Info Endpoint | Baeldung](https://www.baeldung.com/spring-boot-info-actuator-custom)

[Spring Boot's info endpoint, Git and Gradle - Sourced Blog](https://blog.sourced-bvba.be/article/2014/08/15/spring-boot-info-git/)

# Modularization

[Modularizing a Spring Boot Application](https://reflectoring.io/spring-boot-modules/)

# Registering filters, Spring Security

Filters

1. Request log filter
2. Masquerade filter
3. PlatformAuthTokenFilter

Authentication manager, authentication provider → PlatformAuthenticationProvider

Custom error responses

[Custom Error Message Handling for REST API | Baeldung](https://www.baeldung.com/global-error-handler-in-a-spring-rest-api)

# Using spring boot for a command line app

Could be useful for:

- Running liquibase migrations for a given app
- Elastic search indexing
- Spring batch job launch

[Spring Boot Console Application | Baeldung](https://www.baeldung.com/spring-boot-console-app)

[Spring Boot non-web application example - Mkyong.com](https://www.mkyong.com/spring-boot/spring-boot-non-web-application-example/)

# Multiple spring boot apps in one Jar

Not quite the same, running multiple apps simultaneously

[Deploying multiple spring boot applications to a single application server - PRETIUS](http://pretius.com/deploying-multiple-spring-boot-applications-to-single-application-server/)