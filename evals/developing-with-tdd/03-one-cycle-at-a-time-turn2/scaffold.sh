#!/bin/sh
here="$(dirname "$0")"
exec "$here/../fixture/setup.sh" feature "$here/crate"
