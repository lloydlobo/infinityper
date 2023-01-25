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
alias do-br := docker-build-run
alias do-s := docker-stop
alias do-l := docker-logs

#
# BUILD STEPS
#

docker-build:
  docker build -t {{docker_image}} .

#
# DEV STEPS
#

# run container in detatched mode at 8080
docker-run:
  docker run -dp 8080:3030 --rm --name {{docker_container_name}} {{docker_image}}

# build image and run container
docker-build-run:
  just docker-build && just docker-run

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
# PUBLISHING STEPS
#

# tags the image with a docker tag
docker-tag:
  docker tag {{docker_image}} {{username}}/{{docker_image}}

# pushes to hub with default tag `latest` if not specified
docker-push:
  docker push {{username}}/{{docker_image}}

#
# MISC
#

# stop all running containers
docker-stop-all:
  docker stop $(docker ps -q)

# remove all docker containers
docker-image-rm-all:
  docker image rm -f $(docker image ls -q)

