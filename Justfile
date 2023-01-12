@_default:
  just --list --unsorted


tool := "pipeview"
docker_image_version := `cat Cargo.toml | grep ^version | cut -d'=' -f 2 | sed -e 's/"//g' -e 's/ //g'`
docker_user_repo := "mihaigalos"
docker_image_dockerhub := docker_user_repo + "/" + tool + ":" + docker_image_version
docker_image_dockerhub_latest := docker_user_repo + "/" + tool + ":latest"

build:
    sudo docker build \
        --network=host \
        --tag {{ docker_image_dockerhub }} \
        --tag {{ docker_image_dockerhub_latest }} \
        .

run *args:
    sudo docker run --rm -it \
      -v $(pwd):/src \
      {{ docker_image_dockerhub }} {{ args }}

