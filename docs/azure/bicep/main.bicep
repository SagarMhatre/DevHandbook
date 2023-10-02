/*
Source : 
https://learn.microsoft.com/en-in/training/modules/build-first-bicep-template/6-exercise-add-parameters-variables-bicep-template?pivots=cli
https://learn.microsoft.com/en-in/training/modules/build-first-bicep-template/5-add-flexibility-parameters-variables
*/

param location string = 'westus3'
param storageAccountName string = 'toylaunch${uniqueString(resourceGroup().id)}'
/*

uniqueString

  Seed value should be different across different deployments but consistent across all deployments of the same resources.
  Get the same name every time you deploy the same set of resources. 
  But you'll get a different name whenever you deploy a different set of resources by using the same template. 

  Note :
    uniqueString() function will create strings that start with a number. 
    Some Azure resources, like storage accounts, don't allow their names to start with numbers. 
    This means it's a good idea to use string interpolation to create resource names, like in the preceding example.

*/

param appServiceAppName string = 'toylaunch${uniqueString(resourceGroup().id)}'

@allowed([
  'nonprod'
  'prod'
])
param environmentType string

var storageAccountSkuName = (environmentType == 'prod') ? 'Standard_GRS' : 'Standard_LRS'

resource storageAccount 'Microsoft.Storage/storageAccounts@2022-09-01' = {
  name: storageAccountName
  location: location
  sku: {
    name: storageAccountSkuName
  }
  kind: 'StorageV2'
  properties: {
    accessTier: 'Hot'
  }
}

/*
    var appServicePlanName = 'toy-product-launch-plan'
    var appServicePlanSkuName = (environmentType == 'prod') ? 'P2v3' : 'F1'

    resource appServicePlan 'Microsoft.Web/serverFarms@2022-03-01' = {
      name: appServicePlanName
      location: location
      sku: {
        name: appServicePlanSkuName
      }
    }

    resource appServiceApp 'Microsoft.Web/sites@2022-03-01' = {
      name: appServiceAppName
      location: location
      properties: {
        serverFarmId: appServicePlan.id
        httpsOnly: true
      }
    }

*/

module appService 'modules/appService.bicep' = {
  name: 'appService'
  params: {
    location: location
    appServiceAppName: appServiceAppName
    environmentType: environmentType
  }
}

output appServiceAppHostName string = appService.outputs.appServiceAppHostName
// We are referencing the module's output instead of a resource property.

/*
To run the above

New-AzResourceGroupDeployment `
  -TemplateFile main.bicep `
  -environmentType nonprod 
  
*/

