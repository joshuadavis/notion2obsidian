# NodeJS Kotlin

# Developing Web Front End with Kotlin

[Kotlin/kotlin-frontend-plugin](https://github.com/Kotlin/kotlin-frontend-plugin)

[Kotlin/JS configuration made simple - Kt. Academy](https://blog.kotlin-academy.com/kotlin-js-configuration-made-simple-ef0e361fcd4)

[Kotlin and Javascript | Baeldung](https://www.baeldung.com/kotlin-javascript)

# The Gradle way

[Getting Started with Kotlin and JavaScript with Gradle - Kotlin Programming Language](https://kotlinlang.org/docs/tutorials/javascript/getting-started-gradle/getting-started-with-gradle.html)

1. Add gradle wrapper `<localgradle>/bin/gradle wrapper --gradle-version <version> --distribution-type all`
2. Gradle build file `build.gradle` :
    
    ```groovy
    plugins {
        id 'kotlin2js' version '1.3.21'
    }
    
    group 'com.wework.redtech'
    version '1.0-SNAPSHOT'
    
    repositories {
        mavenCentral()
    }
    
    dependencies {
        implementation "org.jetbrains.kotlin:kotlin-stdlib-js"
        testImplementation "org.jetbrains.kotlin:kotlin-test-js"
    }
    
    compileKotlin2Js.kotlinOptions {
        moduleKind = "commonjs"
        outputFile = "node/hello.js"
    }
    ```
    
3. Add source directory `src/main/kotlin`, add code.
    
    `src/main/kotlin/Hello.kt`
    
    ```groovy
    fun main(args: Array<String>) {
        println("Hello JavaScript!")
    }
    ```
    
4. Build it with `./gradlew build`.  You'll see the output JavaScript in  `node/hello.js`.   However, we can't run it yet because NodeJS isn't set up.
5. Set up node for running Kotlin programs:
    1. Initialize node in the directory: `npm init --save`
        1. Answer the prompts.
    2. Add Kotlin: `npm install kotlin --save`  this will create the `node-modules` directory and add the Kotlin JS runtime to it.
6. Run the hello world program: `node node/hello.js`

# The Non-Gradle (command line) way

Install the command line compiler:

[Working with the Command Line Compiler - Kotlin Programming Language](https://kotlinlang.org/docs/tutorials/command-line.html)

1. Start project: `npm init --yes`
2. Add kotlin: `npm install kotlin --save`