set dotenv-load

docker-run: docker
    docker run --rm -p 3000:3000 $DOCKER_IMAGE

docker:
    npm run build
    docker build -t $DOCKER_IMAGE .
