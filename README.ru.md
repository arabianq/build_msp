[English](README.md) | [Русский](README.ru.md)

# build_msp
_Утилита для создания модов в формате .msp_

---

### Как использовать
```
Usage: build_msp [OPTIONS]

Options:
  -i, --input <INPUT>        [default: .]
  -o, --output <OUTPUT>      [default: mod.msp]
  -m, --manifest <MANIFEST>  [default: ./manifest]
  -h, --help                 Print help
  -V, --version              Print version

```
- **--input** - исходная директория с файлами мода
- **--output** - путь к конечному .msp файлу
- **--manifest** - путь к манифесту мода

### Пример манифеста
```
titleid=0100D3F008746000
version=65536
patchset=kpnp_russ
```
- titleid - title_id игры (обязательно)
- version - рекомендуемая версия игры (опционально)
- patchset- название для директории с .ips патчами (опционально)

### Установка
Вы можете скачать готовые исполняемые файлы со [страницы релизов](https://github.com/arabianq/build_msp/releases)

### Установка с помощью cargo
Вы можете установить build_msp из crates.io
```cargo install build_msp```

### Сборка
1. Установите [Rust Lang](https://www.rust-lang.org/tools/install)
2. Клонируйте этот репозиторий (включая субмодуль switch-tools)

   ```git clone --recursive https://github.com/arabianq/build_msp```
3. Запустите сборку с помощью cargo

   ```cd build_msp; cargo build --release```
4. Готово. Теперь в директории _target/release_ находится исполняемый файл

### Особенности
- **Устанавливать .msp файлы можно только с помощью DBI (Duckbill Installer) 772+. [Последняя версия DBI](https://dbi.ultranx.ru/assets/dbi_ru.zip)**
- **build_romfs** и **build_pfs0** взяты из [switch-tools](https://github.com/switchbrew/switch-tools)
- Поддерживаются romfs, exefs и ips моды
- Поддерживаются confg.ini и icon.jpg