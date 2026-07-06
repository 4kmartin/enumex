_enumex_ is a simple commandline tool to enumerate the extensions installed on a machine and print the list. This was designed to be ran in incident response scenarios where a quick answer is what is needed.
This is a side project that gets attention when I have time.

# Usage

Windows:
 
```
Usage: enumex.exe [OPTIONS]

Options:
  -j, --json

  -L, --override-localappdata-path <OVERRIDE_LOCALAPPDATA_PATH>
          provide an override for the %LOCALAPPDATA% path in the event that environment variables cannot be relied on
  -A, --override-appdata-path <OVERRIDE_APPDATA_PATH>
          provide an override for the %APPDATA% path in the event that environment variables cannot be relied on
  -h, --help
          Print help
  -V, --version
          Print version
```
Linux:
  
```
Usage: enumex [OPTIONS]

Options:
  -j, --json

  -H, --override-home-dir <OVERRIDE_HOME_DIR>
          provide an override for the $Home directory in the event that environment variables cannot be relied on
  -h, --help
          Print help
  -V, --version
          Print version
```
# Installation from source

> [!IMPORTANT]
> This installation requires you have already installed the rust compiler.
> If you have not yet done so, install it by following the instructions [here](https://rust-lang.org/tools/install/).


```
  git clone https://github.com/4kmartin/enumex.git
  cd enumex
  cargo install --path . --root . 
```

this will give u a fully compiled binary to copy with your EDR console of choice to the isolated machine.

# Features

supported OSes:

| name | status |
| --- | --- |
| Linux Distros | partially implemented |
| Windows | fully implemented |
| macos | not Implemented |

Supported Browsers:

| name | status |
| --- | --- |
| Microsoft Edge | Fully supported |
| Google Chrome | Fully Supported |
| Mozilla Firefox | Fully Supported |
| Chromium | In development |
| Safari | not yet supported |
| Brave | Not yet supported |
| Opera | Not yet Supported |
