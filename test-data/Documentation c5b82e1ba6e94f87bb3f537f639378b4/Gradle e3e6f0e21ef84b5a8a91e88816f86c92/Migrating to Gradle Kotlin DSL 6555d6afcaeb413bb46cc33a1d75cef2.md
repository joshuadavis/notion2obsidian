# Migrating to Gradle Kotlin DSL

Script for migrating from Gradle Groovy to Kotlin DSL:

Tools:

- Kotlin and `kscript`
    
    [holgerbrandl/kscript](https://github.com/holgerbrandl/kscript)
    
- Gradle Kotlin Converter
    
    [bernaferrari/GradleKotlinConverter](https://github.com/bernaferrari/GradleKotlinConverter)
    

Steps:

1. Install SDKMAN
2. Use SDKMAN to install `kotlin` and `kscript`
    
    ```yaml
    sdk install kotlin
    sdk install kscript
    ```
    
3. Verify kscript:
    
    ```yaml
    kscript --help
    ```
    
4. Get the GraldeKotlinConverter script
5. Switch to the directory where your `build.gradle` script is, and run the converter on it
    
    ```yaml
    $ ~/gradlekotlinconverter.kts build.gradle
    ```
    

Notes:

- `implementation` - Compile time dependency, but *NOT* transitive.
- `api` - Compile time, and transitive