+++
id = "panoptes"
category = "main"
title = "Panoptes"
year = "2026 - now"
url = "https://panoptes.alanp.me"
description = "Discord bot to automatically detect and remove image-based spam"
+++

Written in Rust, using [`serenity-rs`](https://github.com/serenity-rs/serenity/), it scans image attachments against a dataset of known bad images, using perceptual hashing & image embeddings.

Currently deployed on ~7 Discord servers at time of writing, with ~160k total members across those servers.
