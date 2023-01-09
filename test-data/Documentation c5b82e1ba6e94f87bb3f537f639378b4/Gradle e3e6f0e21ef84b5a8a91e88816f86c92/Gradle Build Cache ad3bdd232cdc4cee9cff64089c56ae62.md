# Gradle Build Cache

Cache node server documentation:

[Build Cache Node User Manual | Gradle Enterprise Docs](https://docs.gradle.com/build-cache-node/)

```bash
$ docker run --detach --publish 5071:5071 gradle/build-cache-node
```

Docker compose:

```yaml
# Gradle cache node.
  gradle-cache:
    image: gradle/build-cache-node
    ports:
      - 5071:5071
```

You should be able to see the web ui at `http://localhost:5071`

In `gralde.poperties`

```
org.gradle.caching=true
build_cache_url=http://localhost:5071/
```

Enable in `settings.gradle`:

```jsx
String remoteCacheUrl = System.getenv().get("BUILD_CACHE_URL")

buildCache {
    local {
        enabled = remoteCacheUrl == null
    }
    remote(HttpBuildCache) {
        url = remoteCacheUrl
        push = remoteCacheUrl != null
    }
}
```

```jsx
ext.isCiServer = System.getenv().containsKey("CI")

buildCache {
    local {
        enabled = !isCiServer
    }
    remote(HttpBuildCache) {
        url = 'http://192.168.0.102:8885/cache/'
        push = isCiServer
        
        credentials {
            username = 'some-user'
            password = 'some-complicated-password'
        }
    }
}
```