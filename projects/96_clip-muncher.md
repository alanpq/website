+++
id = "clip-muncher"
category = "main"
title = "Clip Muncher"
year = "2023 - now"
url = "https://clips.alanp.me/"
github = "https://github.com/alanpq/clip-server/"
description = "Discord gameplay clip aggregator."
+++
![Clip Muncher](/static/img/clip-muncher/grid.png)
Easily browse, search, and share videos uploaded to your Discord servers.

> **NOTE**: Clip Muncher is currently closed source and only active on a handful of private Discord servers—without being in one of them, you will not be able to log in.

### How does it work?

- The Discord bot is added to a server, and a channel assigned as the clip upload channel.
- The bot will then start scraping & downloading all videos uploaded to that channel.
- By logging in to the [site](https://clips.alanp.me) with Discord, you can then browse clips from scraped servers you are in.
- You can search by video name, message text associated with the clip, uploader, or by tagged users.
  - You can only tag featured users in clips you uploaded.

![Clip Muncher Video](/static/img/clip-muncher/video.png)

### Technologies

- Frontend made using [SvelteKit](https://kit.svelte.dev/)
- Backend wrapper over [PostgreSQL](https://www.postgresql.org/) written in Rust, with TypeScript bindings generated via [ts-rs](https://github.com/Aleph-Alpha/ts-rs)
- Scraper service written in Rust
  - Scrapes from Discord using [serenity](https://github.com/serenity-rs/serenity), a wrapper for the Discord API
  - Generates video thumbnails using [ffmpeg](https://ffmpeg.org/)
- Deployed as Docker containers, served via an [nginx](https://nginx.org/) reverse proxy

