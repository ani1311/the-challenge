# Project Structure

```txt
crates/
  common/
    src/
      lib.rs
      dto/
      api/
      error.rs

  server/
    src/
      main.rs
      app/
        config.rs
        state.rs
        router.rs

      domain.rs
      domain/
        challenges.rs
        challenges/
          entity.rs
          value_objects.rs
          repository.rs
          errors.rs

      use_cases.rs
      use_cases/
        challenges.rs
        challenges/
          create_challenge.rs
          list_challenges.rs
          get_challenge.rs

      presentation.rs
      presentation/
        http.rs
        http/
          routes.rs
          error.rs
          handlers.rs
          handlers/
            challenges.rs

      persistence.rs
      persistence/
        db.rs
        repositories.rs
        repositories/
          challenges_sqlx.rs
        migrations/

  web/
    src/
      main.rs
      app.rs
      routes/
      components/
      api/
      state/
```

## Boundaries

- `common`: shared API DTOs/types only.
- `server/domain`: pure business logic; no Axum, SQLx, HTTP, or Leptos.
- `server/use_cases`: application workflows.
- `server/presentation`: Axum routes, handlers, request/response mapping.
- `server/persistence`: SQLx, database setup, repository implementations.
- `web`: Leptos UI, API client, frontend state.

## Dependency Direction

```txt
web -> common
server/presentation -> common
server/presentation -> server/use_cases -> server/domain
server/persistence -> server/domain
```

Domain should not depend on anything outer.
