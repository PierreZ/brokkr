# brokkr [![Build Status](https://travis-ci.org/PierreZ/brokkr.svg?branch=master)](https://travis-ci.org/PierreZ/brokkr)

Generate KVM images from Docker images

## Why

Dockerfiles are a great way to represent and build docker images. But many users are using doing something like this:

```docker
FROM debian:old_release
RUN apt update
RUN apt install ...
# ...
# Doing some weird stuff with bash
# ...
```

The truth here is that they are not creating a process, but a custom OS image.

On the other hand, creating customs images for custom needs or to do some immutable infrastructure is complicated and mostly distribution-based.

`Brokkr` tends to solve this problem by reusing all Docker and OCI ecosystem to help you to create custom OS images.

## Why Brokkr

From [Wikipedia](https://en.wikipedia.org/wiki/Brokkr):

> In Norse mythology, Brokkr (Old Norse "the one who works with metal fragments; blacksmith", anglicized Brokk) is a dwarf, and the brother of Eitri or Sindri.

## How-to

```bash
docker pull debian:9
docker save -o /tmp/debian.tar debian:9
brokkr /tmp/debian.tar
```

## Todo

- [x] Decompress, find and squash the layers
- [ ] Handle multiple layers ([tar-rs](https://github.com/alexcrichton/tar-rs) seems to panic when files already exists...)
- [ ] Install what is missing for a bootable image:
  - [ ] initramfs
  - [ ] Kernel
  - [ ] SystemD
  - [ ] ?
- [ ] Generate image ([hint?](https://askubuntu.com/a/165630))