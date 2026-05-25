# AviUtl2 Community Translation

[![AviUtl2 Catalog](https://aviutl2-catalog-badge.sevenc7c.workers.dev/badge/v/aviutl2-community.aviutl2_community_translation_companion)](https://aviutl2-catalog-badge.sevenc7c.workers.dev/package/aviutl2-community.aviutl2_community_translation_companion)

Maintainer: [@sevenc-nanashi](https://github.com/sevenc-nanashi)

Unofficial multilingual translation project for AviUtl2.
This repository manages:

- Translation files for AviUtl2 main program and included effects, and
- A plugin that automatically updates the translated files.

> [!NOTE]
> If your language is not supported and you want to add your language, please let me know by creating Issue!

## Usage

### Automatic

Please install [AviUtl2 Community Translation Companion](https://aviutl2-catalog-badge.sevenc7c.workers.dev/package/aviutl2-community.aviutl2_community_translation_companion) from AviUtl2 Catalog.
Alternatively, you can download the latest release from [Releases](https://github.com/aviutl2/aviutl2-community-translation/releases) and drop `aviutl2-community.aviutl2_community_translation_companion-v*.au2pkg.zip` to preview window of AviUtl2 to install.

### Manual

Open the `.aul2` file in the `./locales/` folder and download the translation files from the download button at the top right.

## About the Plugin

AviUtl2 Community Translation Companion is a plugin that automatically downloads and applies the translation files to AviUtl2.
It checks for updates of the translation files at startup and downloads and applies them if necessary.

It also automatically copies `English.<namespace>.aul2` to `community_en.<namespace>.copied.aul2`,
allowing the translation files to be applied even if the existing plugins/scripts only provide translation files for `English`.

> [!TIP]
> **To plugin/script developers:**
> With this behavior, there is no need to provide translation files for `community_en`.
> It is recommended to provide them in the more general `English`.

## How to Translate

Please use <https://crowdin.com/project/aviutl2-community-translation>.
The repository is synchronized with Crowdin periodically.

> [!TIP]
> When translating, you might want to use "Multilingual" layout to use English as reference.
> You can switch to "Multilingual" layout from the layout selector at the top right of the translation editor.

## License

The translation files in this repository are provided under the [MIT License](LICENSE).
The copyrights for the AviUtl2 main program and included effects belong to their respective authors.
