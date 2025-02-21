+++
id = "klipr"
category = "main"
title = "klipr"
year = "2022 - now"
github = "https://github.com/alanpq/klipr"
description = "Video editor with a focus on gameplay footage."
+++
Originally a [flutter project](https://github.com/alanpq/klipr-flutter) - klipr is an (in progress) video editor with an opinionated workflow for editing and exporting gameplay footage. Written in Rust, using [egui](https://github.com/emilk/egui) for the GUI, and [ffmpeg](https://ffmpeg.org/) for video playback, as well as processing/encoding.

![Klipr](/static/img/klipr/klipr_01.png)

The workflow is as follows:

- Klipr imports video files from recorded gameplay via pre-existing software such as [OBS](https://obsproject.com/).
- The user then defines one or more "clips" from the source videos, where they can also change the audio track mixing and define any additional metadata (who is in the video, what game was played, etc.).
- The clips can then be exported non-destructively and encoded to meet target file sizes/bitrates/etc. (useful for exporting to social media platforms with file size restrictions).
  - Optionally, to save disk space, the clips can be "baked," deleting the source video—leaving only the desired clips.
