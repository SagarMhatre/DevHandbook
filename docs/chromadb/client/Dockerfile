FROM python:3.10.13

WORKDIR /code
COPY ./requirements.txt /code/requirements.txt
RUN pip install --no-cache-dir --upgrade -r /code/requirements.txt
COPY ./reader.py ./reader.py
CMD ["python", "./reader.py"]