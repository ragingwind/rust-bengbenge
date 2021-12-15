if [[ $(uname -m) == 'arm64' ]]; then
  CONTAINER_ARCH="--container-architecture linux/amd64"
fi

act -j $1 $CONTAINER_ARCH --env-file ./scripts/act.env