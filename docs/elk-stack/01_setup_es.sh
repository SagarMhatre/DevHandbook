# sudo chmod 755 ./setup_es.sh
clear

HOME_PATH=$PWD

## Platform
# M1 MacBook Pro
PLATFORM="--platform linux/arm64"
# Intel Windows
# PLATFORM="--platform linux/arm64"

## Network
DOCKER_NETWORK=
# DOCKER_NETWORK= --network  es_stack_network

ELASTIC_PASSWORD="A=Very=Strong=Passw0rd"

## Cleanup previous

docker container rm -f elasticsearch 
docker container rm -f kibana
docker container rm -f enterprise-search
docker volume rm es-config
docker volume rm es-data

# It will be on https
echo "https://localhost:9200 elastic $ELASTIC_PASSWORD"

docker run \
--name "elasticsearch" --publish "9200:9200" \
--volume "es-config:/usr/share/elasticsearch/config:rw" \
--volume "es-data:/usr/share/elasticsearch/data" \
--env "ELASTIC_PASSWORD=$ELASTIC_PASSWORD"  \
--interactive \
--tty \
"docker.elastic.co/elasticsearch/elasticsearch:8.12.2" 
# --interactive \
# --tty \
# "$DOCKER_NETWORK" \ 
# --rm \
# > setup_es.log

# read -p "Press any key to continue ..."
# docker logs elasticsearch | grep 'start new Elasticsearch nodes with `bin/elasticsearch --enrollment-token'
# docker logs elasticsearch | grep 'publish_address'
# docker ps --format "{{.Names}}" 

read -p "Press any key to exit ..."
exit