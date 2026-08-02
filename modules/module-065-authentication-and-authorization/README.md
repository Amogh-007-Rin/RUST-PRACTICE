# Module 065: Authentication & Authorization

**Block:** Block G — Backend Web Development
**Estimated time:** 60–90 min
**Prerequisites:** Module 064 (database operations), Module 063 (REST API patterns with extractors and state). This module replaces the in-memory store with a user store + JWT auth layer.

## Learning Objectives

- Hash passwords securely with argon2id, the current state-of-the-art password hashing algorithm.
- Sign and verify JSON Web Tokens (JWTs) with HS256 symmetric keys.
- Build a custom Axum extractor (`AuthUser`) that decodes and validates JWTs from the `Authorization` header.
- Protect routes so only authenticated requests can reach them.
- Understand the difference between authentication (who you are) and authorization (what you can do).

## Why This Matters

Authentication is the gatekeeper of every real backend service. Password hashing, JWT signing, and token-based auth middleware are the standard trifecta for user authentication in Rust — argon2 for storage, `jsonwebtoken` for sessionless tokens, and a custom `FromRequestParts` impl to tie them into axum. In production, you use the exact same approach (often with refresh tokens and rotation layered on top), and the extractor pattern you build here is reusable across any axum project that needs protected routes.

## Concept

### Password hashing: argon2id

Passwords must never be stored in plain text. If your database leaks, plain-text passwords expose every user. Hashing solves this: a hash function is *one-way* — you can go from password to hash, but not from hash back to password.

argon2id is the current recommendation from the Password Hashing Competition and OWASP. It's deliberately memory-hard and CPU-hard to make brute-force attacks (trying billions of passwords per second) expensive even with specialized hardware.

**Hashing** creates a "PHC string" (Password Hashing Competition format) containing the algorithm, parameters, salt, and hash — everything needed to verify later:

```rust
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

let salt = SaltString::generate(&mut OsRng);
let hash = Argon2::default()
    .hash_password(password.as_bytes(), &salt)?
    .to_string();
// hash looks like: "$argon2id$v=19$m=19456,t=2,p=1$<salt>$<hash>"
```

**Verification** parses the stored hash string and checks if the provided password produces the same output:

```rust
use argon2::password_hash::{PasswordHash, PasswordVerifier};

let parsed = PasswordHash::new(stored_hash)?;
Argon2::default()
    .verify_password(password.as_bytes(), &parsed)
    .is_ok()
```

Key security properties:
- The salt is random per password — identical passwords produce different hashes, preventing rainbow-table attacks.
- Verification runs in constant time — no timing side-channel leaks about which characters matched.
- A malformed hash string simply returns `false` rather than panicking — the verifier is a pure boolean check.

### JWT: stateless authentication

A JSON Web Token (JWT) is a compact, URL-safe token with three dot-separated parts:

```
header.payload.signature

eyJhbGciOiJIUzI1NiJ9  .  eyJzdWIiOiJhbGljZSJ9  .  HMACSHA256(header+"."+payload)
```

| Part | Content | Purpose |
|---|---|---|
| Header | `{"alg": "HS256"}` | Tells the verifier which algorithm to use |
| Payload | `{"sub": "alice", "exp": 1717000000}` | The *claims* — data about the user and token |
| Signature | Binary HMAC | Proves the token wasn't tampered with |

The `sub` (subject) claim holds the username; `exp` (expiry) is a Unix timestamp after which the token is invalid. Both are standard registered claims from RFC 7519.

**Signing** creates the signature by HMAC-ing the header + payload with a secret key:

```rust
let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
let claims = Claims { sub: username, exp: now + 3600 };
let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))?;
```

**Verification** recomputes the signature and checks it matches:

```rust
let data = decode::<Claims>(token, &DecodingKey::from_secret(secret.as_bytes()), &Validation::new(Algorithm::HS256))?;
// data.claims is your Claims struct — signature and expiry both checked
```

The secret is a symmetric key — the same string used to sign must be used to verify. In production, use a strong random secret (e.g., `openssl rand -hex 32`) and keep it out of source control.

### The trust model

With JWTs, you don't need a server-side session store. The server signs a token, hands it to the client, and the client sends it back on every request in the `Authorization` header:

```
Authorization: Bearer eyJhbGciOiJI...
```

When the server receives the token, it verifies the signature. If the signature is valid, the server *knows*:
1. The token was issued by *this server* (or someone with the secret).
2. The claims haven't been modified (signature wouldn't match).

No database lookup needed — the user's identity is carried in the token itself. This is *stateless authentication*. The trade-off: you can't revoke individual JWTs without a revocation list (that's what refresh tokens and short expiry windows address).

### Custom extractor: AuthUser

Axum's extractor system lets you create domain-specific extractors by implementing `FromRequestParts`. This is the mechanism for auth middleware — instead of checking tokens in every handler, you write one extractor and use it as a handler parameter:

```rust
pub struct AuthUser(pub Claims);

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let header = parts.headers.get(AUTHORIZATION)...;
        let token = header.strip_prefix("Bearer ")...;
        let claims = verify_token(&state.jwt_secret, token)...;
        Ok(AuthUser(claims))
    }
}
```

The flow: extract the header → strip the `"Bearer "` prefix → verify the token → return `AuthUser(claims)`. If any step fails, return `(StatusCode::UNAUTHORIZED, message)` — axum turns this into a `401` response and the handler never runs.

Now any handler can simply take `AuthUser(claims): AuthUser` as a parameter, and it's *guaranteed* to only execute for authenticated requests:

```rust
pub async fn me(AuthUser(claims): AuthUser) -> String {
    claims.sub  // the username — no manual auth check needed
}
```

This is the Rust way of doing auth middleware: type-driven, no annotations, no macros — the extractor signature *is* the gate. If a handler doesn't take `AuthUser`, the route is public. If it does, the route is protected.

### Registration and login flow

Registration hashes the password with argon2, stores the hash (never the plain text), and returns a JWT. Login verifies the password against the stored hash, and if it matches, returns a JWT. Both follow the same shape:

```
Client                    Server
  │                          │
  │  POST /register          │
  │  {"username","password"} │
  │─────────────────────────►│
  │                          │  hash = argon2(password)
  │                          │  store (username, hash)
  │                          │  token = issue_jwt(username)
  │  201 {"token","username"}│
  │◄─────────────────────────│
  │                          │
  │  POST /login             │
  │  {"username","password"} │
  │─────────────────────────►│
  │                          │  lookup stored hash by username
  │                          │  verify_password(password, stored_hash)
  │                          │  token = issue_jwt(username)
  │  200 {"token","username"}│
  │◄─────────────────────────│
  │                          │
  │  GET /me                 │
  │  Authorization: Bearer X │
  │─────────────────────────►│
  │                          │  AuthUser extractor verifies token
  │                          │  handler returns calls.sub
  │  200 "alice"             │
  │◄─────────────────────────│
```

### Security boundaries

A few critical security points this module enforces:

- **Passwords are hashed before storage.** The handler never sees raw passwords after hashing.
- **Login errors are intentionally vague.** "Invalid credentials" covers both wrong password and unknown user — you never tell an attacker whether a username exists.
- **Token expiry is checked.** `decode` with `Validation::new(HS256)` automatically checks the `exp` claim — expired tokens are rejected.
- **Wrong secret = invalid token.** A token signed with a different key fails verification, preventing cross-service token reuse.

## Common Pitfalls

- **Using a weak or fast hash (MD5, SHA-256, bcrypt with low cost).** argon2id is the current best practice. SHA-256 is a general-purpose hash, not a password hash — it's designed for speed, not security against brute-force.
- **Forgetting to set `exp`.** A JWT without an expiry claim is valid forever. Always include `exp` and make it short (1 hour max for access tokens in production).
- **Leaking whether a username exists.** Returning distinct error messages for "user not found" vs. "wrong password" enables username enumeration attacks. Return the same message for both.
- **Storing secrets in source code.** The JWT secret in tests is fine; in production, load it from the environment or a secrets manager.
- **Not checking the `Alg: HS256` validation.** `jsonwebtoken::decode` with `Validation::new(HS256)` explicitly checks the algorithm. Without this, an attacker could craft a token with `"alg": "none"` and bypass verification entirely.

## Key Terms

- **Argon2id:** A memory-hard password hashing algorithm, winner of the Password Hashing Competition, resistant to both GPU and ASIC attacks.
- **Salt:** Random data mixed into a hash to ensure identical passwords produce different hashes.
- **JWT (JSON Web Token):** A self-contained token with a header, claims payload, and cryptographic signature.
- **Claims:** Key-value pairs in the JWT payload — `sub` for subject (username), `exp` for expiry.
- **HS256:** HMAC with SHA-256 — the symmetric signing algorithm used in this module.
- **`FromRequestParts`:** The trait for extractors that only read request metadata (headers, URI, method), not the body.
- **Bearer token:** An access token sent in the `Authorization: Bearer <token>` header.

## Exercise

Open `exercises/src/lib.rs`. The HTTP layer (router, handlers, state) is complete. Five security functions contain `// TODO(module-065)` stubs:

1. **`hash_password`** — Generate a salt with `SaltString::generate(&mut OsRng)`, hash the password with `Argon2::default().hash_password()`, and return the `.to_string()` of the result.

2. **`verify_password`** — Parse the stored hash with `PasswordHash::new()`, verify with `Argon2::default().verify_password()`, return `true` if it matches. Malformed hashes should return `false`, not panic.

3. **`issue_token`** — Build `Claims { sub: username, exp: now + TOKEN_TTL_SECONDS }` (using `SystemTime::now().duration_since(UNIX_EPOCH)`), then `encode` with `Header::default()` and `EncodingKey::from_secret(secret.as_bytes())`.

4. **`verify_token`** — Call `decode::<Claims>(token, &DecodingKey::from_secret(secret.as_bytes()), &Validation::new(Algorithm::HS256))` and return `.claims`.

5. **`AuthUser` extractor (the `from_request_parts` impl)** — Read the `Authorization` header, strip `"Bearer "`, verify the token against `state.jwt_secret`, and return `AuthUser(claims)`. Reject with `UNAUTHORIZED` for missing/malformed/invalid tokens.

The tests in `tests/module_065.rs` cover hashing, verification, registration, login, token expiry, wrong secrets, and protected route access. Run:

```bash
cargo test -p module-065-exercises
```

Compare with `solutions/` when all tests pass.

## Further Reading

- [The JWT specification (RFC 7519)](https://datatracker.ietf.org/doc/html/rfc7519)
- [argon2 crate documentation](https://docs.rs/argon2)
- [jsonwebtoken crate documentation](https://docs.rs/jsonwebtoken)
- [OWASP Password Storage Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html)
- [Module 064: Database Integration with sqlx](modules/module-064-database-integration-with-sqlx/README.md)
