# envkeydiff

Compare the keys in a `.env` file with a committed `.env.example` and report missing or undocumented variables. Values are never printed, so it is safe to run in CI and deploy logs. No dependencies.

## Install

```bash
cargo install envkeydiff
```

## Usage

```bash
envkeydiff                         # compares .env.example with .env
envkeydiff .env.example .env.prod
envkeydiff --strict .env.example .env.prod
```

Output lists `missing:` keys (in the example, not in the real file) and `extra:` keys (in the real file, not documented). Exit code `1` when keys are missing, or when there are extras in `--strict` mode. Exit code `2` for bad arguments or unreadable files.

## Where it fits

- In CI, to fail a pull request that adds a new variable to code but forgets `.env.example`.
- In a container entrypoint, to stop early with a clear message instead of crashing on the first request.
- Before a deploy, to compare the production env file with the example.

How Compose resolves `.env`, `env_file` and `environment` is explained in [environment variables in Docker Compose](https://peon.sh/blogs/environment-variables-docker-compose). Built by a contributor to [Peon](https://peon.sh/), an open-source self-hosted deployment platform.

MIT licensed.
