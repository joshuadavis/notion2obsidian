# Gradle Dependency Troubleshooting

A specific dependency: `./gradlew dependencyInsight --dependency log4j-core`

List all dependencies: `./gradlew dependencies`

Except this doesn't work for multimodule builds.  For multi-module builds, use:

```jsx
subprojects {
    task allDeps(type: DependencyReportTask) {}
}
```

Also `./gradlew dependencyUpdates`

[Inspecting Dependencies](https://docs.gradle.org/current/userguide/inspecting_dependencies.html)