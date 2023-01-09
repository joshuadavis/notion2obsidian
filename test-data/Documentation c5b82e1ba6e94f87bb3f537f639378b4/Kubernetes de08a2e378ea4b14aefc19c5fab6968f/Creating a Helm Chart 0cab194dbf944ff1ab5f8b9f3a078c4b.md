# Creating a Helm Chart

# Before Creating a Chart

1. Create your application
2. Dockerize the application - You should have a docker image that is either local, or pushed to a repository.

# Create a Chart

To create a chart

```bash
helm create <chartname>
```

Chart file structure:

```bash
Chart.yaml
values.yaml
templates/
	ingress.yaml
  deployment.yaml
	service.yaml
charts/
```

- `values.yaml` - The default template parameters.    i.e. if you `helm install` the chart without any parameters, these will be used
- 

# Template Files

The template files allow you to parameterize K8s objects for the deployment.

Template directives inside these files allow you to substitute values into the templates.

## Service Template

in `templates/service.yaml`we define the service, including:

- Name
- Port
- Selectors

## Deployment Template

Allows you to specify the deployment of the service:

- replica count (number of containers)
- Pod template (`spec: template` )
    - Containers:
        - The image to use, image pull policy
- Ports, liveness probe, etc.

## Ingress Template

- Define the ingress for the service