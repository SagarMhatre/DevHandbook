
## Pricing
https://www.elastic.co/pricing

Free ?
https://www.elastic.co/downloads 
Download the Elastic Stack to get started with Elastic Enterprise Search, Observability, and Security for free. 

## Elastic Enterprise Search
is an additional Elastic service that adds APIs and UIs to those already provided by Elasticsearch and Kibana.

## TODO 
[] Download & Install https://www.elastic.co/downloads/enterprise-search

## Installation

```sh
sh 01_setup_es.sh
```

```log
ℹ️  Configure Kibana to use this cluster:
• Run Kibana and click the configuration link in the terminal when Kibana starts.
• Copy the following enrollment token and paste it into Kibana in your browser (valid for the next 30 minutes):
  eyJ2ZXIiOiI4Ljc******xRWctSTdSakEifQ==
```

```sh
sh 02_setup_kb.sh
```

```log
Go to http://0.0.0.0:5601/?code=949287 to get started.
```



### Create an index
http://0.0.0.0:5601/app/enterprise_search/content/search_indices/new_index/api

name : mydocs

### Generate an API key at 
http://0.0.0.0:5601/app/enterprise_search/content/search_indices/search-mydocs/overview

mydocs-dev
WUdIay1JMEJJWVU4Q2hDME5BYlA6Q2oyUWxFSmRRUmF2MUJIYWxJNldkdw==

### Test Connection
```rest
@API_KEY='WUdIay1JMEJJWVU4Q2hDME5BYlA6Q2oyUWxFSmRRUmF2MUJIYWxJNldkdw=='
curl "https://localhost:9200/search-mydocs" \
  -H "Authorization: ApiKey "{{API_KEY}}"" \
  -H "Content-Type: application/json"
```

### Ingest data
```rest
@API_KEY='WUdIay1JMEJJWVU4Q2hDME5BYlA6Q2oyUWxFSmRRUmF2MUJIYWxJNldkdw=='
curl "https://localhost:9200/_bulk?pretty&pipeline=ent-search-generic-ingestion" \
  -H "Authorization: ApiKey "{{API_KEY}}"" \
  -H "Content-Type: application/json" \
  -d'
{ "index" : { "_index" : "search-mydocs" } }
{"name": "Snow Crash", "author": "Neal Stephenson", "release_date": "1992-06-01", "page_count": 470, "_extract_binary_content": true, "_reduce_whitespace": true, "_run_ml_inference": true}
{ "index" : { "_index" : "search-mydocs" } }
{"name": "Revelation Space", "author": "Alastair Reynolds", "release_date": "2000-03-15", "page_count": 585, "_extract_binary_content": true, "_reduce_whitespace": true, "_run_ml_inference": true}
{ "index" : { "_index" : "search-mydocs" } }
{"name": "1984", "author": "George Orwell", "release_date": "1985-06-01", "page_count": 328, "_extract_binary_content": true, "_reduce_whitespace": true, "_run_ml_inference": true}
{ "index" : { "_index" : "search-mydocs" } }
{"name": "Fahrenheit 451", "author": "Ray Bradbury", "release_date": "1953-10-15", "page_count": 227, "_extract_binary_content": true, "_reduce_whitespace": true, "_run_ml_inference": true}
{ "index" : { "_index" : "search-mydocs" } }
{"name": "Brave New World", "author": "Aldous Huxley", "release_date": "1932-06-01", "page_count": 268, "_extract_binary_content": true, "_reduce_whitespace": true, "_run_ml_inference": true}
{ "index" : { "_index" : "search-mydocs" } }
{"name": "The Handmaid'"'"'s Tale", "author": "Margaret Atwood", "release_date": "1985-06-01", "page_count": 311, "_extract_binary_content": true, "_reduce_whitespace": true, "_run_ml_inference": true}
'
```

### Search

```rest
@API_KEY='WUdIay1JMEJJWVU4Q2hDME5BYlA6Q2oyUWxFSmRRUmF2MUJIYWxJNldkdw=='
curl "https://localhost:9200/search-mydocs/_search?pretty" \
  -H "Authorization: ApiKey "{{API_KEY}}"" \
  -H "Content-Type: application/json" \
  -d'
{
  "query": {
    "query_string": {
      "query": "snow"
    }
  }
}'
```

```rest
@API_KEY='WUdIay1JMEJJWVU4Q2hDME5BYlA6Q2oyUWxFSmRRUmF2MUJIYWxJNldkdw=='
curl "https://localhost:9200/search-mydocs/_search?pretty" \
  -H "Authorization: ApiKey "{{API_KEY}}"" \
  -H "Content-Type: application/json" \
  -d'
{
  "query": {
    "query_string": {
      "query": "a"
    }
  }
}'
```

## Getting the API Key

```rest
@elastic_password="A=Very=Strong=Passw0rd"
curl -u  elastic:{{elastic_password}} -XPOST 'https://localhost:9200/_security/api_key' -H 'Content-Type: application/json' -d '{
  "name": "my-api-key",
  "role_descriptors": {
    "applications": [
      {
        "application": "my_app",
        "privileges": ["read"],
        "resources": ["*"]
      }
    ]
  }
}'
```