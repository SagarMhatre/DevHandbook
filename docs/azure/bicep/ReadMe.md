
### Pre-req
https://learn.microsoft.com/en-us/cli/azure/install-azure-cli-macos#install-with-homebrew
https://learn.microsoft.com/en-us/powershell/azure/install-azps-macos?view=azps-10.4.1#installation
https://marketplace.visualstudio.com/items?itemName=ms-azuretools.vscode-bicep

Ref : https://learn.microsoft.com/en-in/training/paths/fundamentals-bicep/


### Choose the subscription & resource group
Use pwsh in the vscode terminal 
```sh
Connect-AzAccount
# .. A browser opens so that you can sign in to your Azure account.

# list your subscriptions and their IDs ...
Get-AzSubscription

# Set the default subscription ...
$context = Get-AzSubscription -SubscriptionId {Your subscription ID}
$context = Get-AzSubscription -SubscriptionId  e7b734c1-f0b1-4be0-8076-6f87c682815b
Set-AzContext $context

# [Optional] set the default resource group (to omit the parameter from the rest of the Azure PowerShell commands)
Set-AzDefault -ResourceGroupName [ resource group name]

```

### Deploy
```sh
New-AzResourceGroupDeployment `
  -TemplateFile main.bicep `
  -environmentType nonprod 

# Resource Manager looks at what is already deployed in Azure. 
# It then looks at what you're trying to deploy, and it sets up a sequence of steps to achieve this state. 
# All these activities involve invoking the Resource Manager API.

# View the JSON template that's submitted to Resource Manager by using ...
bicep build main.bicep


```

### Verify the deployment
```sh
Get-AzResourceGroupDeployment -ResourceGroupName [ resource group name] | Format-Table
Get-AzResourceGroupDeployment -ResourceGroupName ai-learn| Format-Table
```