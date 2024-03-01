clear

HOME_PATH=$PWD

## Platform
# M1 MacBook Pro
PLATFORM="--platform linux/arm64"
# Intel Windows
# PLATFORM="--platform linux/arm64"

## Cleanup previous

docker container rm -f kibana 

# docker logs elasticsearch | grep 'publish_address' > elasticsearch_publish_address.txt
# cat elasticsearch_publish_address.txt

# ENROLLMENT_TOKEN='eyJ2ZXIiOiI4LjcuMCIsImFkciI6WyIxNzIuMTcuMC4yOjkyMDAiXSwiZmdyIjoiZjFiNGM5MWM4NTU0YzBlZTYzZmZiYzkyNGJmZmQ1YzM4OWY4ZDBkMjY0YjRlMWYwZDI2OTRiMDE5OTA0YThlZSIsImtleSI6Im1wMmEtSTBCOHFYVkw2UFFGNFd6OmYwNGFJNjcyU0JlekFxRWctSTdSakEifQ=='
# echo ENROLLMENT_TOKEN = $ENROLLMENT_TOKEN

docker run \
--name "kibana" \
--publish "5601:5601" \
--interactive \
--tty \
--env "ENTERPRISESEARCH_HOST=http://localhost:3002" \
"docker.elastic.co/kibana/kibana:8.12.2"
# -e "ENROLLMENT_TOKEN=$ENROLLMENT_TOKEN"   \
# --network $DOCKER_NETWORK \
#--rm \