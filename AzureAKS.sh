#!/bin/bashās

az group create --name myResourceGroup --location eastus
az aks create --resource-group myResourceGroup --name myAKSCluster --node-count 3 --enable-addons monitoring --generate-ssh-keys
