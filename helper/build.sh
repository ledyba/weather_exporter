#! /bin/bash -eu

PROJ_PATH=$(readlink -f $(cd $(dirname $(readlink -f $0)) && pwd))
cd ${PROJ_PATH}/..

mkdir -p artifacts

docker-compose build
docker run -i -v ${PWD}/artifacts:/artifacts weather_exporter cp /weather_exporter /artifacts