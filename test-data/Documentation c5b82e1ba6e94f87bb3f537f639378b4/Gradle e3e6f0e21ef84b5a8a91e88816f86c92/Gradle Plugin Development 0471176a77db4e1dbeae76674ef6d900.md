# Gradle Plugin Development

# The Docos

[Developing Custom Gradle Plugins](https://docs.gradle.org/current/userguide/custom_plugins.html)

[Testing Gradle plugins](https://guides.gradle.org/testing-gradle-plugins/)

# Kotlin Project Setup

Plugins

```kotlin
plugins {
    kotlin("jvm") version Version.KOTLIN
    groovy
    `java-gradle-plugin`
}
```

Dependencies

```kotlin
dependencies {
    implementation(kotlin("stdlib-jdk8"))
    implementation(gradleApi())
    implementation(localGroovy())
    testImplementation(group = "org.testng", name = "testng", version = "6.8.21")
}
```

# Basics

## Adding Tasks

One of the most basic things you can do with a plugin is to add some tasks.  Tasks can be added right in the plugin class 'apply' method, but this probably isn't what you want.  You probably want to make a class for each task, and then register those as tasks in the project.

# Depending on another plugin

For Kotlin / Java plugin development, you will need the types in the other plugin in order to compile.

This is relatively simple, as you just depend on the plugin as an `implementation` dependency.

## Add the plugin

To add that plugin from your plugin:

[How to apply a Gradle plugin from another plugin?](https://stackoverflow.com/questions/27318144/how-to-apply-a-gradle-plugin-from-another-plugin)

For example:

```kotlin
class MyPlugin : Plugin<Project> {
    override fun apply(project: Project) {
        project.pluginManager.apply("io.spring.dependency-management")
				...
```

# Testing

Unit tests can be very simple.

Functional tests, you can use `GradleRunner` to actually spawn an instance of gradle and run a real build script.