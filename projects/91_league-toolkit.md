+++
id = "league-toolkit"
category = "main"
title = "league-toolkit"
year = "2024 - now"
github = "https://github.com/LeagueToolkit/league-toolkit"
description = "Rust library for handling League of Legends file formats"
+++

Based on the original [C# project](https://github.com/LeagueToolkit/LeagueToolkit), league-toolkit powers the rest of our [org](https://github.com/LeagueToolkit)'s modding stack - notably our [mod manager](https://github.com/LeagueToolkit/ltk-manager) for League of Legends, and other modding related tools like [hexbelt](https://github.com/alanpq/hexbelt/).

The library is responsible for idiomatic and performant APIs to read/write and otherwise manipulate the proprietary file formats and structures found in the *League of Legends* game engine.

It also implements the parsing/printing of [ritobin](https://github.com/moonshadow565/ritobin), a custom DSL to represent League's `.bin` files in a human readable format. This also powers [ritobin-lsp](/projects/ritobin-lsp), my language server implementation.

