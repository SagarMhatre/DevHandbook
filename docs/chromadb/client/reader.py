import chromadb
chroma_client = chromadb.HttpClient(host='chromadb', port=8000)
collection = chroma_client.get_or_create_collection(name="my_collection2")

results = collection.query(
    query_texts=["This is a query document for learning about saturn "],
    n_results=1
)
print(results)
results = collection.query(
    query_texts=["This is a query document for learning about dog "],
    n_results=1
)
print(results)