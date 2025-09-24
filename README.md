# Clip
`clip` is a command line tool to manage your clipboard, you can pipe something into it and out of it.

### Usage
```
Usage: clip [OPTIONS] [VALUE]

Arguments:
  [VALUE]  Value to set the clipboard to

Options:
  -s, --share    If set a link to the clipboard will be created.
  -h, --help     Print help
  -V, --version  Print version
```
### Instalation
- Run the installer from the [latest release](https://github.com/Sushi-Mampfer/clip/releases/latest)
- You might need to move the path where it gets installed to above the preinstalled binarys path
- On Windows you can locate them with `where.exe clip.exe`
- On Linux you can locate them with `which clip`

### Roadmap
- [x] Linking to system clipboard and piping in and out
- [x] Sharing (Backend is available [here](https://github.com/Sushi-Mampfer/ClipWeb))
- [ ] History
- [ ] Syncing