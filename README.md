# Fidelity Fetch (fife)

<div align="center">
  <a href="https://github.com/CramBL/fidelityfetch/releases" title="Latest Stable GitHub Release">
      <img src="https://img.shields.io/github/release/CramBL/fidelityfetch/all.svg?style=flat&logo=github&logoColor=white&colorB=blue&label=" alt="GitHub release">
  </a>    
    <img src="https://img.shields.io/badge/-Windows-6E46A2.svg?style=flat&logo=windows-11&logoColor=white" alt="Windows" title="Supported Platform: Windows">&thinsp;
    <img src="https://img.shields.io/badge/-Linux-9C2A91.svg?style=flat&logo=linux&logoColor=white" alt="Linux" title="Supported Platform: Linux">&thinsp;
    <img src="https://img.shields.io/badge/-macOS-red.svg?style=flat&logo=apple&logoColor=white" alt="macOS" title="Supported Platform: macOS">
</div>

## Purpose

Single self-contained binary that provides an easily configurable server that supports range requests and allows for serving and browsing remote files through a web browser.


## Demo

### Installing and setting up on a Raspberry Pi Zero W

![demo-fife](https://github.com/user-attachments/assets/368f9af1-9a1a-4d52-98a9-cc017ebd40af)

## Installation

### Prebuilt binaries

```shell
curl -L -H "Accept: application/vnd.github.v3.raw" \
    https://api.github.com/repos/CramBL/fidelityfetch/contents/install.sh \
    | sh -s -- --to <DEST>
```

### From source

```shell
cargo install fidelityfetch
```
