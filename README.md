# rust-authentication-microservice

An authentication microservice built in Rust.

Implements the ability to register new users, and authenticate existing users.

This is intended to be a demo of how Rust could be used to build a microservice. It is not ready for production use in its current state, particularly security-wise.

## Technologies Used

*  [Rust](https://rust-lang.org)
*  [gRPC](https://grpc.io/) with [tonic](https://github.com/hyperium/tonic)
*  [Postgres](https://www.postgresql.org/) with [SQLx](https://github.com/launchbadge/sqlx)
*  Containerisation with [Docker](https://www.docker.com/), [Kubernetes](https://kubernetes.io/), and [Skaffold](https://github.com/GoogleContainerTools/skaffold)

## Setup

*  Refer to the [Skaffold Quickstart](https://skaffold.dev/docs/quickstart/) to details on how to set up your environment.
*  Start a local Docker registry instance.

```sh
docker run -d -p 5000:5000 --restart=always --name registry registry:2
```

*  Run skaffold

```sh
skaffold dev
```

*  Wait for it to build, the first one will be quite slow.

## Notes

### Postgres Port Forwarding

Run the following command to port forward the Postgres instance to your host.

```sh
kubectl port-forward $(kubectl get pods|awk '/^authentication-postgres.*Running/{print$1}'|head) 5432:5432
```
