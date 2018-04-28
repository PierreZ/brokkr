# brokkr [![Build Status](https://travis-ci.org/PierreZ/brokkr.svg?branch=master)](https://travis-ci.org/PierreZ/brokkr)

Experiment to generate ISO/KVM images from Docker images

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

## Requirements

* genisoimage (cdrtools package for Arch)
* Docker daemon with experimental features, see [here](https://stackoverflow.com/a/46565552)

## How-to

```bash
# Let's build our debian 9
docker build --squash -t my-debian9 -f ./Dockerfile.debian9.example .

# Export OCI image
docker save -o /tmp/debian.tar my-debian9:latest
brokkr /tmp/debian.tar
```

## Todo

* [x] Decompress, find and squash the layers
* [x] Generate image
* [ ] Make it work! Stuck on: 'no boot medium found' on virtualbox