# YTDLP-CLI
Little wrapper around [ytdlp](https://github.com/yt-dlp/yt-dlp) to download one or more videos from YouTube with audio only or video.

## Installation
Prerequisites:
- [ytdlp](https://github.com/yt-dlp/yt-dlp) (Tested with `2023.07.06`)
- [ffmpeg](https://ffmpeg.org/) (Tested with `6.0`)

The only solution, atm, is to build from source:
```bash
$ git clone https://github.com/JockeRider199/ytdlp-cli
$ cd ytdlp-cli
$ cargo build --release
```

## Roadmap
- [ ] Add support for downloading playlists
- [ ] Add proper binaries