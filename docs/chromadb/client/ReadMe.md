## Code setup
To build & run the docker image 
```sh
clear
# docker network create --driver bridge alpine-net
docker stop chromadb
docker run --rm --name chromadb -p 8000:8000 --network alpine-net  -v /Users/sagarmhatre/Documents/src/DevHandbook/docs/chromadb/chroma:/chroma/chroma chromadb/chroma:0.4.24
```
```sh
clear
docker build  -t chromadb-reader:1.0 .
docker stop chromadb-reader
# docker build --platform=linux/amd64 -t chromadb-reader:1.0 .
docker run --rm --name chromadb-reader --network alpine-net  chromadb-reader:1.0
```