# Demo Kind/Istio/Rocket setup
A very simple example of routing done with istio gateways, virtual services, and rust (rocket) web apps. Once set up you can control which service you talk to based on setting a cookie `beta=true`

## Setup
This should be all the steps to get things running.

1. Install [Kind](https://kind.sigs.k8s.io/)
2. Create a cluster using kind. Use the included bash script (kind.sh) to make the cluster expose a local docker container repository so you can build and push images there instead of to somewhere on the internet.
3. Build, tag, and push the docker images for both Rust services.
    1. `cd hello_world`
    2. `docker build -t rust_hello_world:1.0 .`
    3. `docker tag rust_hello_world:1.0 localhost:5000/rust_hello_world:1.0`
    4. `docker push localhost:5000/rust_hello_world:1.0`
    5. Repeat the above commands for `hello_person`
4. Install the `istio-operator` to your kind cluster. You can use this [helm](https://helm.sh/) command to do so:
    ```
    helm template manifests/charts/istio-operator/ \
        --set hub=docker.io/istio \
        --set tag=1.7.0 \
        --set operatorNamespace=istio-operator \
        --set watchedNamespaces=istio-system | kubectl apply -f -
    ```
5. Create the `istio-system` namespace, where all of our istio services will live
    `kubectl create ns istio-system`
6. Apply the istio operator definition: `kubectl apply -f istio-operator.yaml`
7. Apply the services for both rust services
    `kubectl apply -f rusthelloworld.yaml`
    `kubectl apply -f rusthelloperson.yaml`
8. Apply the istio gateway and virtual service (both defined in virtualservice.yaml)
    `kubectl apply -f virtualservice.yaml`
9. Expose the port to the istio ingress gateway:
    `kubectl port-forward -n istio-system service/demo-gateway 8800:80`
10. You should be able to hit `localhost:8800/hello/world` and see a response from the hello person service: `world`
11. If you add the cookie `beta=true` in your browser you will now get the hello world service, with a response of `Hello worldz!`

## Troubleshooting
There are a lot of moving pieces here so things can get complicated very fast. These are in no way (even close) to exhaustive ways to troubleshoot what's going on, but it's a start!

- Make sure istio is happy. If you install istioctl (`brew install istioctl`) it can help you make sure everything is good. You can run `istioctl analyze -n istio-system` to make sure the istio components are working as expected.
- Make sure you have enough RAM and CPU allocated for kind. Especially if `istio` has not started correctly.
- You can check the envoy (istio's http proxy) logs by identifying the pod for the gateway (named something like `demo-gateway-12323-23424`) and running `kubectl logs -n istio-system demo-gateway-12323-23424`.
- You can check the logs for the rust services by running `kubectl logs -n istio-system helloperson-123123-123123` just like the gateway.