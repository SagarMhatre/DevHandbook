# Pre-reqs

- Install the Extension for Azure Developer CLI in VS code 
https://learn.microsoft.com/en-in/azure/developer/azure-developer-cli/debug?pivots=ide-vs-code
https://marketplace.visualstudio.com/items?itemName=ms-azuretools.azure-dev

- Install the Azure CLI
```sh
curl -fsSL https://aka.ms/install-azd.sh | bash
```

azd version  
azd version 1.3.0 (commit 6605c5768cb3ab6d9e0e6c2d892c9e8b49e1f720)

https://nodejs.org/en
node -v
v20.6.1

npm -v
9.8.1



python --version                               
Python 3.11.4

- VSCode > View menu > Command Palette > Type and pick "Azure Developer: init"

# Steps

Use  region US East (some services may not be available in other regions )
https://learn.microsoft.com/en-us/azure/ai-services/openai/quotas-limits#quotas-and-limits-reference

azd auth login

    Browser will open 
    (if already logged in ...)
        Authentication complete. You can return to the application. Feel free to close this browser tab.


cd az samples
mkdir azure-search-openai-demo
cd azure-search-openai-demo
azd init -t azure-search-openai-demo

    Initializing an app to run on Azure (azd init)

    (✓) Done: Downloading template code to: /Users/sagarmhatre/Documents/src/az samples/azure-search-openai-demo

    ? Enter a new environment name: azure-search-openai-demo

    SUCCESS: New project initialized!
    You can view the template code in your directory: /Users/sagarmhatre/Documents/src/az samples/azure-search-openai-demo
    Learn more about running 3rd party code on our DevHub: https://aka.ms/azd-third-party-code-notice


## Cost Reduction

    To reduce costs, you can switch to free SKUs for 
     - Azure App Service
     - Azure Cognitive Search, and 
     - Form Recognizer 

    by changing the parameters file under the infra folder. 
    There are some limits to consider; for example, you can have up to 1 free Cognitive Search resource per subscription, and the free Form Recognizer resource only analyzes the first 2 pages of each document. 

    You can also reduce costs associated with the Form Recognizer by reducing the number of documents in the data folder, or by removing the postprovision hook in azure.yaml that runs the prepdocs.py script.

# Enabling Application Insights
To enable Application Insights and the tracing of each request, along with the logging of errors, set the AZURE_USE_APPLICATION_INSIGHTS variable to true before running azd up

## Install the Fluent UI React package
Ref :https://www.npmjs.com/package/@fluentui/react
```sh
npm i @fluentui/react
```

```sh
azd env set AZURE_USE_APPLICATION_INSIGHTS true
```
Deploy ...

```sh
azd up
```
    After the application has been successfully deployed you will see a URL printed to the console. Click that URL to interact with the application in your browser.



# Clean up
To clean up all the resources created by this sample:

 - Run azd down
 - When asked if you are sure you want to continue, enter y
 - When asked if you want to permanently delete the resources, enter y
