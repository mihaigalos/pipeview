@_default:
  just --list --unsorted


tool := "pipeview"
docker_image_version := "0.0.3"
docker_user_repo := "mihaigalos"
docker_image_dockerhub := docker_user_repo + "/" + tool + ":" + docker_image_version
docker_image_dockerhub_latest := docker_user_repo + "/" + tool + ":latest"

build:
   sudo docker build --network=host -t {{ docker_image_dockerhub_latest }} .

