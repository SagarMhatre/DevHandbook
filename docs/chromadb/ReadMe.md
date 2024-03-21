

## On Docker

```sh
docker run --rm --name chromadb -p 8000:8000 chromadb/chroma:0.4.24
```

## Persistent Storage
Indexed data are located in "/chroma/chroma/" - https://github.com/chroma-core/chroma/blob/0.4.24/docker-compose.yml#L14
```sh
clear
docker stop chromadb
docker run --rm --name chromadb -p 8000:8000 -v /Users/sagarmhatre/Documents/src/DevHandbook/docs/chromadb/chroma:/chroma/chroma chromadb/chroma:0.4.24
```
GA Tags : https://github.com/chroma-core/chroma/tags
Dev releases : https://github.com/chroma-core/chroma/releases

