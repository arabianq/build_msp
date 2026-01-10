[English](README.md) | [Русский](README.ru.md)

# build_msp
_A tool that helps to build .msp file mod_

---

### How to use
```
Usage: build_msp [OPTIONS]

Options:
  -i, --input <INPUT>        [default: .]
  -o, --output <OUTPUT>      [default: mod.msp]
  -m, --manifest <MANIFEST>  [default: ./manifest]
  -h, --help                 Print help
  -V, --version              Print version

```
- **--input** - the source directory with the mod files
- **--output** - path to the .msp file
- **--manifest** - path to the manifest

### Manifest example
```
titleid=0100D3F008746000
version=65536
patchset=kpnp_russ
```
- titleid - title_id of a game (required)
- version - recommended game version (optional)
- patchset- name for the .ips patches directory (optional)

### Installing
You can download pre-built binaries from [releases page](https://github.com/arabianq/build_msp/releases)

### Building
1. Install CMake (if not already installed)
2. Clone this repository (including switch-tools submodule)

    ```git clone --recursive https://github.com/arabianq/build_msp```
3. Configure the project with CMake
    ```cmake .```
4. Build the project
    ```make```
5. Done. The executable will be in the current directory.

### Notes
- **You can install .msp files only with DBI (Duckbill Installer) 772+. [Latest DBI](https://cp.arabianq.ru/ultranx/dbi/dbi_ru.zip)**
- Supports romfs, exefs and ips mods
- Supports confg.ini and icon.jpg