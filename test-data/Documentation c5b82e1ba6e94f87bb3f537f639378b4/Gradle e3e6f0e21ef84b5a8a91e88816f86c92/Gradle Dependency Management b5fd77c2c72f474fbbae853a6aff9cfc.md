# Gradle Dependency Management

Generating / Consuming BOM

[Gradle scripts to generate a BOM and then consume that BOM](https://gist.github.com/jlafourc/379da4500d205ca88e151c3803f06ade)

Uses spring dependency management plugin:

[spring-gradle-plugins/dependency-management-plugin](https://github.com/spring-gradle-plugins/dependency-management-plugin)

Conditional dependency

```groovy
plugins {
    id "java"
}

repositories {
    jcenter()
}

dependencies {
    if (project.hasProperty("gson")) {
        implementation "com.google.gson:gson:2.8.5"
    } else {
        implementation "org.fasterxml.jackson.core:jackson-core:2.9.0"
    }
}
```