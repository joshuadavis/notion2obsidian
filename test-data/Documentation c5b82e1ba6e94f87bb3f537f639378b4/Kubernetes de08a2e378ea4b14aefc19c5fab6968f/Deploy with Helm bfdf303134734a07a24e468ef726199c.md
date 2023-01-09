# Deploy with Helm

What is Helm?  Why should we use it?

- Deploying can also be done with `kubectl`, but this is *very tedious*....
    - Each k8s resource has a yaml file, and you need to use `kubectl` for each one.
        - Deployment
        - The service itself
        - Maybe an 'ingress'
        - Maybe other services, each with a deployment and maybe an ingress.
    - Yeah, bummer.
- With Helm
    - You can treat all the k8s resources as one thing, and you can deploy them all at once.
    - All the k8s resource YAML files are in a directory.   This is referred to as a 'chart'
    - It is also a templating engine - you can abstract away repeated information into a template.
    - Sounds a bit like Cloud Formation with ECS, doesn't it?
    - Helm also keeps track of what versions of the resources are installed, so things can be rolled back if a deployment fails.

What you need to deploy with Helm

1. A docker image, deployed in a repo
2. A Kubernetes cluster
3. Helm and Tiller installed on the K8s cluster

To create a chart

```bash
helm create <chartname>
```

To deploy for the first time:

```bash
helm install <appname> .
```

where `.` is the directory containing the helm chart

To update the app:

```bash
helm update <appname> .
```

What about environments, etc?

Make a Helm Chart

[How to Create Your First Helm Chart](https://docs.bitnami.com/kubernetes/how-to/create-your-first-helm-chart/#step-3-modify-chart-to-deploy-a-custom-service)

Example

[Deploy a Java Application on Kubernetes with Helm](https://docs.bitnami.com/kubernetes/how-to/deploy-java-application-kubernetes-helm/)

# Deploying with Helm, to Docker for Mac Kubernetes

[Kubernetes Application Deployment Made Easy using Helm on Docker for Mac 18.05.0 CE](http://collabnix.com/kubernetes-application-deployment-made-easy-using-helm-on-docker-for-mac-18-05/)

Install helm:

```jsx
brew install kubernetes-helm
```

To upgrade

```jsx
brew upgrade kubernetes-helm
```

Might also need to upgrade `kubectl`

```jsx
brew upgrade kubernetes-cli
```

To initialize `helm` with the current Kubernetes context:

```jsx
helm init --upgrade
```

- This will initialize the client and server.   Probably shouldn't need to do this on an established K8s cluster.

# Managing an application

First time installation, from the directory where the helm chart is stored.

```jsx
helm install --name <releasename> ./
```

- This will use all the values in `values.yaml` and deploy / update everything as appropriate.

Upgrade

```jsx
helm upgrade --name <releasename> ./
```

Get the status:

```bash
helm status <releasename>
```

Delete (uninstall)

```jsx
helm delete <releasename>
```

What if the application doesn't come up?

Do `helm status <releasename>` and look at the pods.

If pod status is `ImagePullBackOff` it might be having trouble downloading the image.   Make sure the image reference exists.