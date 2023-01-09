# JWT Spring Boot

# JWT Basics

Three parts

1. header
2. payload - Set of claims (fields)
    - `iss` - Issuer
    - `sub` - Subject
    - `aud` - Audience
    - `exp` - Expiry, current time must be *before* this time
    - `nbf` - Not before.   Current time must be *after* this time
    - `iat` - Issued at
    - `jti` - JWT ID
3. signature

The three parts are base64 encoded and the signature is generated with a secret key and an algorithm. 

Auth0's website describe this well:

[JSON Web Tokens (JWT) in Auth0](https://auth0.com/docs/jwt)

## Server types

There are two basic types of servers: 

1. Authorization Server - this provides signed JWTs.  For Redtech  this is Auth0
2. Resource Server - this serves up resources based on the claims (fields) in the JWT.   Verifies the signature. This is basically all backend services in Redtech.  

## Redtech JWT Conventions

*We should select an algorithm for Redtech.   Can be parameterized, but we need a default.*

Select which standard claims (fields) we want to use and / or require.

Example JWT body:

```yaml
{
  "iat": 1540309744,
  "exp": 1559596144,
  "iss": "dealtrack-develop",
  "sub": "deal-sync-user",
  "tokenId": "deal-sync-user",
  "type": "SERVICE",
  "user": {
    "firstName": "Deal",
    "lastName": "Sync",
    "id": "deal-sync-user",
    "email": "dealtrack@wework.com"
  }
}
```

Select cookie / header / either?

Select libraries:

- Kotlin/Spring
    - Auth0 libraries currently in use by Materials
    - Any JWT library should work
    - Cornerstone components for:
        - JWT reading / validation / spring security integration
        - JWT generation (optional)
- NodeJS

# Spring Security / Web Integration

Configure in `WebSecurityConfigurerAdapter`

Or...

- Use Spring Security OAuth
- Auth0 Spring Security Integration

## Spring Boot Oauth2 - Resource Server

Spring Boot Application has:

1. A dependency on `org.springframework.security.oauth.boot:spring-security-oauth2-autoconfigure`
2. A `@EnableResourceServer` configuration that is `ResourceServerConfigurationAdapter` instead of the usual `@EnableWebMvc` and `WebMvcConfigurer`

```kotlin
@Configuration
@EnableResourceServer
class OAuth2ResourceServerConfig : ResourceServerConfigurerAdapter() {
	override fun configure(resources: ResourceServerSecurityConfigurer?) {
        resources?.resourceId("some-resource")
    }
    override fun configure(http: HttpSecurity?) {
        http!!.authorizeRequests()
                .antMatchers("/actuator/health").permitAll()
                .anyRequest().authenticated()
    }
}
```

- The `resourceId` , if it's not null, is matched against the `aud` claim in the JWT.
**If you want to accept all 'aud' values, just set it to null.**
- Use `.antMatchers("/actuator/health").permitAll()` to add public paths
- The rest of the requests require authentication: `.anyRequest().authenticated()`
- The resource server config will automatically install a filter and other beans that will pull out the JWT header and verify the signature.

Configuration needs to have:

```yaml
security:
  oauth2:
    resource:
      jwt:
        key-value: <some jwt signing key>
```

- Config prefix is `security.oauth2.resource` (ResourceServerProperties)
- `key-value` can be a public or symmetric key (it somehow figures this out)
- Default signing type is HMACSHA256

Details:

- JwtTokenStore will automatically be configured, this is what will parse and validate the signature.
    - JwtHelper.decodeAndVerify(token, verifier)
    - Verifier is HMACSHA256 by default.
    

# Links

[Using JWT with Spring Security OAuth | Baeldung](https://www.baeldung.com/spring-security-oauth-jwt)

[Implementing JWT Authentication on Spring Boot APIs](https://auth0.com/blog/implementing-jwt-authentication-on-spring-boot/)

[Implementing JWT Authentication on Spring Boot APIs - DZone Security](https://dzone.com/articles/implementing-jwt-authentication-on-spring-boot-api)

Also auth0

[JWT authentication with Spring Web - Part 4](https://sdqali.in/blog/2016/07/07/jwt-authentication-with-spring-web---part-4/)

Shows how to verify the token and extract the claims / information.

[JWT authentication with Spring Web - Part 1](https://sdqali.in/blog/2016/07/02/jwt-authentication-with-spring-web---part-1/)

[Use JWT The Right Way!](https://stormpath.com/blog/jwt-the-right-way)

[JWT.IO](https://jwt.io/#debugger-io)

JWT Debugger

# Libraries

List of libraries:

[JWT.IO](https://jwt.io/#libraries-io)

Spring Libraries:

[auth0/auth0-spring-security-api](https://github.com/auth0/auth0-spring-security-api)

- This is good, but it is based on an older version of Spring Security than what we are using.   Cornerstone uses 5.0.8, and the latest version of this library uses 4.2.9 (big difference)

[spring-projects/spring-security-oauth](https://github.com/spring-projects/spring-security-oauth/tree/master/spring-security-jwt)

- This is a very comprehensive system for supporting Oauth in general, and JWT.

Low level libraries:

[jwtk/jjwt](https://github.com/jwtk/jjwt)

okta / stormpath

[auth0/java-jwt](https://github.com/auth0/java-jwt)

- Reading a token → `Jwts.parser().setSigningKey(key).parseClaimsJws(compactJws)`
- Generating / signing a token → `Jwts.builder().setSubject("Joe").signWith(key).compact()`