

```sh 

mkdir $PROJECT_NAME
cd $PROJECT_NAME

azd template list
azd template list | grep func

# Select the one suitable for you
azd init --template Azure-Samples/todo-csharp-sql-swa-func  
```az-

