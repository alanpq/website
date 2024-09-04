---
id: klipr
category: main
title: klipr
year: "2022 - 2023"
github: https://github.com/alanpq/klipr
description: "Video editor with a focus on gameplay footage."
---

Originally a [flutter project](https://github.com/alanpq/klipr-flutter) - klipr is an (unfinished) video editor with an opinionated workflow for quickly editing and exporting gameplay footage. Written in rust, using [iced](https://iced.rs/) for the GUI, [gstreamer](https://gstreamer.freedesktop.org/) for playback, and [ffmpeg](https://ffmpeg.org/) for all video processing/encoding.

The typical (planned) workflow is as follows:
  - Klipr imports video files from recorded gameplay via pre-existing software such as [OBS](https://obsproject.com/).
  - The user then defines one or more "clips" from the source videos, where they can also change the audio track mixing, and define any additional metadata (who is in the video, what game was played, etc).
  - The clips can then be exported non-destructively and encoded to meet target file sizes/bitrates/etc. (useful for exporting to social media platform with file size restrictions)
    - Optionally, to save disk space, the clips can be "baked", deleting the source video - leaving only the desired clips.