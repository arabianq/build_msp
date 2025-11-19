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

### Сборка
1. Установите CMake (если еще не установлен)
2. Клонируйте этот репозиторий (включая субмодуль switch-tools)

    ```git clone --recursive https://github.com/arabianq/build_msp```
3. Сконфигурируйте проект с помощью CMake
    ```cmake ..```
4. Соберите проект
    ```make```
5. Готово. Исполняемый файл будет находиться текущей директории.

### Особенности
- **Устанавливать .msp файлы можно только с помощью DBI (Duckbill Installer) 772+. [Последняя версия DBI](https://dbi.ultranx.ru/assets/dbi_ru.zip)**
- **build_romfs** и **build_pfs0** взяты из [switch-tools](https://github.com/switchbrew/switch-tools)
- Поддерживаются romfs, exefs и ips моды
- Поддерживаются confg.ini и icon.jpg