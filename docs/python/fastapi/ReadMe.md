Ref : https://fastapi.tiangolo.com/tutorial/

requirements.txt
```txt
fastapi==0.45.0
uvicorn[standard]==0.28.0
```

https://fastapi.tiangolo.com/deployment/versions/#pin-your-fastapi-version
https://github.com/encode/uvicorn/releases

```sh
pip install -r requirements.txt
uvicorn main:app --reload
```