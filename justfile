# TODO: Log the most frequently used commands and deprecate the unused ones.

set positional-arguments

# choose a recipe interactively
default:
  just --choose

#
# VARIABLES
#

docker_image := "infinityper"
docker_container_name := 'infinityper1'
username := 'lloydlobo'

#
# ALIASES
#

alias do-b := docker-build
alias do-r := docker-run
alias do-l := docker-logs
alias do-s := docker-stop
alias do-dev := docker-dev
alias do-br := docker-build-run
alias do-brdev := docker-build-dev
alias do-brentry := docker-build-run-entrypoint

#
# BUILD
#

docker-build:
  docker build -t {{docker_image}} .

#
# DEV
#

# run cmd container in detatched mode at 8080
docker-run:
  docker run -dp 8080:3030 --rm --name {{docker_container_name}} {{docker_image}}

# run entrypoint with args, `*` accepts 0 or more args
# docker-run-entrypoint-args FLAGS *OPTIONS:
#   docker run -it --entrypoint bin/bash {{docker_image}} && pwd && ./{{docker_image}} {{FLAGS}} {{OPTIONS}} 

# Run the binary interactively in the terminal `$ ./infinityper`
docker-run-entrypoint:
  docker run -it --rm --name {{docker_container_name}} --entrypoint bin/bash {{docker_image}}

# fetches container logs & follows log output
docker-logs:
  docker logs -f {{docker_container_name}}

# stop container `sample1`
docker-stop:
  docker stop {{docker_container_name}}


#
# API LOGS
#

# test GET request to `localhost:8080`
curl-port:
  curl --request GET 'localhost:8080/'

#
# PUBLISHING
#

# tags the image with a docker tag
docker-tag:
  docker tag {{docker_image}} {{username}}/{{docker_image}}

# pushes to hub with default tag `latest` if not specified
docker-push:
  docker push {{username}}/{{docker_image}}

#
# CLEANUP
#

# stop all running containers
docker-stop-all:
  docker stop $(docker ps -q)

# remove all docker containers
docker-image-rm-all:
  docker image rm -f $(docker image ls -q)

#
# DEV RECIPES
#
 
# build image and run container
docker-build-run:
  just docker-build && just docker-run

# build image, run container interactively with entrypoint /bin/bash
docker-build-run-entrypoint:
  just docker-build && just docker-run-entrypoint

# build image, run container, and watch logs.
docker-build-dev:
  just docker-build && just docker-run && just docker-logs


# run and watch container logs
docker-dev:
  just docker-run && just docker-logs


