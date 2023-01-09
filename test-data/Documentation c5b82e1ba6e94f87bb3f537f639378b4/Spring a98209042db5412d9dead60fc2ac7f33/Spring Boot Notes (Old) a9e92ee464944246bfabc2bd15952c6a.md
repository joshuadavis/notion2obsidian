# Spring Boot Notes (Old)

[http://www.nurkiewicz.com/2018/01/spring-boot-2-migrating-from-dropwizard.html](http://www.nurkiewicz.com/2018/01/spring-boot-2-migrating-from-dropwizard.html)

[http://www.baeldung.com/spring-boot-rest-client-swagger-codegen](http://www.baeldung.com/spring-boot-rest-client-swagger-codegen)

JPA

- Data source - Just define a DataSource bean. By default, HikariCP is used.
- properties [https://docs.spring.io/spring-boot/docs/current/reference/html/howto-data-access.html#howto-configure-jpa-properties](https://docs.spring.io/spring-boot/docs/current/reference/html/howto-data-access.html#howto-configure-jpa-properties)
    - set with spring.jpa.properties.*, all passed through with the prefix ‘spring.jpa.properties.’ stripped off.
    - add JPA properties with HibernatePropertiesCustomizer, just register a bean that implements this interface

Docker

- [https://springframework.guru/running-spring-boot-in-a-docker-container/](https://springframework.guru/running-spring-boot-in-a-docker-container/)
- [https://developers.redhat.com/blog/2017/03/14/java-inside-docker/](https://developers.redhat.com/blog/2017/03/14/java-inside-docker/)
- [https://docs.docker.com/compose/compose-file/#volumes-for-services-swarms-and-stack-files](https://docs.docker.com/compose/compose-file/#volumes-for-services-swarms-and-stack-files)
- [https://stories.amazee.io/docker-on-mac-performance-docker-machine-vs-docker-for-mac-4c64c0afdf99](https://stories.amazee.io/docker-on-mac-performance-docker-machine-vs-docker-for-mac-4c64c0afdf99)
- [https://docs.docker.com/docker-for-mac/osxfs-caching/](https://docs.docker.com/docker-for-mac/osxfs-caching/)

Add spring-boot plugin:

buildscript {

ext {

springBootVersion = '1.5.2.RELEASE’

…

}

….

dependencies {

classpath("org.springframework.boot:spring-boot-gradle-plugin:${springBootVersion}")

}

}

apply plugin: 'spring-boot’

dependencies {

compile("org.springframework.boot:spring-boot-starter:${springBootVersion}")

compile("org.springframework.boot:spring-boot-starter-actuator:${springBootVersion}")

compile("org.springframework.boot:spring-boot-starter-data-jpa:${springBootVersion}")

compile("org.springframework.boot:spring-boot-starter-security:${springBootVersion}")

compile("org.springframework.boot:spring-boot-starter-web:${springBootVersion}")

}

When not using Logback:

```jsx
configurations {
	all*.exclude group: 'org.springframework.boot', module: 'spring-boot-starter-logging’
}
```

Using Log4j2:

[http://docs.spring.io/spring-boot/docs/current/reference/html/howto-logging.html#howto-configure-log4j-for-logging](http://docs.spring.io/spring-boot/docs/current/reference/html/howto-logging.html#howto-configure-log4j-for-logging)

[https://springframework.guru/using-log4j-2-spring-boot/](https://springframework.guru/using-log4j-2-spring-boot/)

compile("org.springframework.boot:spring-boot-starter-log4j2:${springBootVersion}”)

Adding XML files: