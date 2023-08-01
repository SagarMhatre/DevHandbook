```
helm create <projectname> 
```


values.yaml
```
image:
  repository:   
  tag:

service:
  type: 
  port:

```

# Reading configurations from configmap & secrets
```
      containers:
        - name: {{ .Chart.Name }}
          securityContext:
            {{- toYaml .Values.securityContext | nindent 12 }}
          image: "{{ .Values.image.repository }}:{{ .Values.image.tag | default .Chart.AppVersion }}"
          env:
            - name: ENV_VAR1_NAME
              valueFrom:
                configMapKeyRef:
                  name: CONFIG_MAP_NAME
                  key: KEY1_NAME
            - name: ENV_VAR2_NAME
              valueFrom:
                configMapKeyRef:
                  name: SECRETS_NAME
                  key: SECRET_KEY1_NAME
```

Above configurations are defined in configmap & secrets


``` 
# configmap.yaml

apiVersion: v1
kind: ConfigMap
metadata:
  name: CONFIG_MAP_NAME
  namespace: NAME_SPACE_NAME
data:
  KEY1_NAME:VALUE1      
  KEY2_NAME:VALUE2
```

Note: Values in the secrets a re base 64 encoded

```
# secrets.yaml

apiVersion: v1
kind: Secret
metadata:
  name: SECRETS_NAME
  namespace: NAME_SPACE_NAME
type: Opaque
data:
  SECRET_KEY1_NAME: YnJpeA==
  SECRET_KEY2_NAME: dGVzdGluZw==

```

# Liveness probe without a service exposed on a port
```
          livenessProbe:
            exec:
              command: 
                - ls 
                - -lt
            initialDelaySeconds: 5
            periodSeconds: 10
          readinessProbe:
            exec:
              command: 
                - ls 
                - -lt
            initialDelaySeconds: 60
            periodSeconds: 20
```

# custome values.yaml
```
helm install -f ./custom-values.yaml stable/nginx-ingress
```