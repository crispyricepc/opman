#!/usr/bin/env sh

VERSION=r$(git rev-list --count HEAD).$(git rev-parse --short HEAD)
sed "s/pkgver=/pkgver=${VERSION}/g" ./ci/opman-git/PKGBUILD.template > \
    ./ci/opman-git/PKGBUILD