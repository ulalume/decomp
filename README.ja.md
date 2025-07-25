# Archr - macOS-like Archive Extractor

macOS likeな解凍ツールです。クロスプラットフォームですが主にWindowsをターゲットにしています。

## インストール方法

```bash
cargo install --git https://github.com/ulalume/archr
```

## 使い方

### 1. ファイル関連付け（推奨）
Windowsのファイル関連付けで圧縮ファイルのデフォルトアプリに設定すると、圧縮ファイルをダブルクリックするだけでその場に解凍できます。
アプリの場所: `C:\Users\[username]\.cargo\bin\archr.exe`

### 2. GUIモード
```bash
archr.exe
```
ファイルダイアログが開くので、解凍したい圧縮ファイルを選択してください。

### 3. コマンドラインモード
```bash
archr.exe archive1.zip archive2.7z archive3.tar.gz
```
指定したファイルを一括で解凍します。

## 対応形式

- **ZIP** (.zip)
- **7-Zip** (.7z)
- **RAR** (.rar)
- **TAR** (.tar)
- **GZIP** (.gz, .tar.gz, .tgz)
- **XZ** (.xz, .tar.xz)
- **BZIP2** (.bz2, .tar.bz2)
- **LHA/LZH** (.lha, .lzh)
- 日本語ファイル名に対応（Shift_JIS/CP932エンコーディング）

## 注意事項

- パスワード付きアーカイブは現在サポートしていません
