
https://learn.acloud.guru/course/33a5a87a-2b2c-4e51-b00f-8f2e2a15ab30/learn/501f3829-6645-4ddc-8634-a004142bef7d/5903da33-bd82-482e-bbd7-972c931ffa9b/watch


## Pre-req

https://marketplace.visualstudio.com/items?itemName=ms-python.python

### print json

```py
jsonBody = {
            "documents":[
                {"id": 1,
                 "text": text}
            ]
        }

        # Let's take a look at the JSON we'll send to the service
        print(json.dumps(jsonBody, indent=2))
```