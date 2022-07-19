# helmtokustomize
Converts helm generated chart.yaml to separate files and adds kustomize.yaml to reference all the files.

## Why
Sometimes, Helm hides too much details from being explicitly clear.

## How to use
Usually with Helm, you create a values.yaml and then you apply the chart with the defined values.  There are many projects out there that only document how to use Helm and don't make their manifests clear.  

This way you can use Helm to generate the manifests and `helmtokustomize` to split the files.

If you are using Kustomization to track the kubernetes yamls, then you can run a command like the following:

```
helm template <release_name> <chart/name> --namespace <your_app_namespace> -f values.yaml > chart.yaml
```

This will create a single file that is called `chart.yaml`.

You can then run:
```
h2k chart.yaml
```

and it will create individual files for each of the sections in the `chart.yaml`.
