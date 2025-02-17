<div align=right>Table of Contents↗️</div>

<h1 align=center>Fidelity Fetch

<code>fife</code>

</h1>

<div align="center">
  <a href="https://github.com/CramBL/fidelityfetch/releases" title="Latest Stable GitHub Release">
      <img src="https://img.shields.io/github/release/CramBL/fidelityfetch/all.svg?style=flat&logo=github&logoColor=white&colorB=blue&label=Latest Release" alt="GitHub release"></a>
    <img src="https://img.shields.io/badge/-Windows-6E46A2.svg?style=flat&logo=windows-11&logoColor=white" alt="Windows" title="Supported Platform: Windows">&thinsp;
    <img src="https://img.shields.io/badge/-Linux-9C2A91.svg?style=flat&logo=linux&logoColor=white" alt="Linux" title="Supported Platform: Linux">&thinsp;
    <img src="https://img.shields.io/badge/-macOS-red.svg?style=flat&logo=apple&logoColor=white" alt="macOS" title="Supported Platform: macOS">
</div>

## Purpose

Serve files efficiently on a local network.

- Browse/open/download the files through a web browser
- Easily configurable server
- Support for range requests
- Single self-contained binary
- mDNS/DNS-SD capabilities

## Quick Start

Recursively serve `<path>` and all its contents at `<hostname>.local:<port>`

```shell
fife -p <port> -m <hostname> --root <path>
```
The content can be browsed through a web browser at `http://<hostname>.local:<port>`
> Note: Many phones do not support mDNS resolution, but your PC does unless it is ancient. In that case you need to specify the IP instead of the mDNS hostname to access the contents.

The contents can also be fetched through the command-line, e.g.

```shell
curl http://<hostname>.local:<port>/some/path/to/file.txt # Get file.txt
curl -r 99-499 http://<hostname>.local:<port>/foo.bin # Starting from byte 99, get the next 400 bytes of foo.bin
```

If no port is specified, any available port is used (`fife` requests a free port from the OS).

## Demo

### Installing and setting up on a Raspberry Pi Zero W

![demo-fife](https://github.com/user-attachments/assets/368f9af1-9a1a-4d52-98a9-cc017ebd40af)

## Installation

### Prebuilt binaries

Check out the [the release page](https://github.com/CramBL/fidelityfetch/releases/latest).

### From source

```shell
cargo install fidelityfetch
```

### Systemd

There's an example service unit file at [./package/fife.service](./package/fife.service)

### TODO: Yocto recipe
