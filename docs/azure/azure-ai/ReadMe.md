# Original Source :
https://github.com/Azure-Samples/azure-search-openai-demo

Ref:
https://learn.microsoft.com/en-us/azure/developer/azure-developer-cli/azd-templates?tabs=csharp

# Pre-reqs

- Install the Extension for Azure Developer CLI in VS code 
https://learn.microsoft.com/en-in/azure/developer/azure-developer-cli/debug?pivots=ide-vs-code
https://marketplace.visualstudio.com/items?itemName=ms-azuretools.azure-dev

- Install / Update the Azure CLI
```sh
curl -fsSL https://aka.ms/install-azd.sh | bash
```

azd version  
azd version 1.3.1 (commit b5030da0f29ffe98664c40450187fd8a4cb0d157)

Change to the Tenant where u have signed up for Azure AI & verify if it has changed 

```sh
# To get a list ..
az account list --output table
# To change ... 
az account set --subscription $SUBSCRIPTION_ID
# To verify ...
az account show --output table 
```



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
     - Azure Cognitive Search

```sh
        azd env set AZURE_SEARCH_SERVICE_SKU free
        azd config unset AZURE_SEARCH_SERVICE_SKU
        azd env get-values
```

([See possible sku values](https://learn.microsoft.com/azure/templates/microsoft.search/searchservices?pivots=deployment-language-bicep#sku))

     - Form Recognizer 
        infra/main.parameters.json
```json
    "formRecognizerSkuName": {      
        "value":"free" 
    },
```

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

If you already have a Resource Group 
```sh
azd env set AZURE_RESOURCE_GROUP $RESOURCE_GROUP_NAME
azd env set AZURE_LOCATION "East US"
azd env get-values   
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
 - When asked if you want to permanently delete the resources, enter yaz account tenant list

 ### Troubleshooting

If you need to find the name of your deleted resources, you can get a list of deleted resource names

 ```sh
 Get-AzResource -ResourceId /subscriptions/83307c02-8dab-47fa-bf09-f49e7989eec1/providers/Microsoft.CognitiveServices/deletedAccounts -ApiVersion 2021-04-30
 ```

Once you delete a resource, you won't be able to create another one with the same name for 48 hours. To create a resource with the same name, you will need to purge the deleted resource.

To purge a deleted Azure AI services resource, 
```sh 
Remove-AzResource -ResourceId /subscriptions/{subscriptionID}/providers/Microsoft.CognitiveServices/locations/{location}/resourceGroups/{resourceGroup}/deletedAccounts/{resourceName}  -ApiVersion 2021-04-30

# e.g.
Remove-AzResource -ResourceId /subscriptions/83307c02-8dab-47fa-bf09-f49e7989eec1/providers/Microsoft.CognitiveServices/locations/eastus2/resourceGroups/rg-azure-search-openai-demo/deletedAccounts/cog-fr-hkjtpkog5vv5a   -ApiVersion 2021-04-30 -verbose

Remove-AzResource -ResourceId /subscriptions/83307c02-8dab-47fa-bf09-f49e7989eec1/providers/Microsoft.CognitiveServices/locations/eastus/resourceGroups/rg-azure-search-openai-demo/deletedAccounts/cog-hkjtpkog5vv5a -ApiVersion 2021-04-30 -verbose

Remove-AzResource -ResourceId   /subscriptions/83307c02-8dab-47fa-bf09-f49e7989eec1/providers/Microsoft.CognitiveServices/locations/eastus/resourceGroups/kms/deletedAccounts/classify-text -ApiVersion 2021-04-30 -verbose

```