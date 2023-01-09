# Enforce Java Version

```groovy
def javaVersion = JavaVersion.VERSION_1_7;
sourceCompatibility = javaVersion;
targetCompatibility = javaVersion; // defaults to sourceCompatibility

task enforceVersion << {
    def foundVersion = JavaVersion.current();
    if (foundVersion != javaVersion) 
        throw new IllegalStateException("Wrong Java version; required is "
            + javaVersion + ", but found " + foundVersion);
}
```