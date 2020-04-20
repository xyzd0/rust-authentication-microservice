# rust-authentication-microservice

An authentication microservice built in Rust. This is one of my first Rust projects so there will be things to improve, and I will go back and do these as and when I get time!

Implements the ability to register new users, and authenticate existing users. No authoristion (e.g. RBAC) has been implemented, this would be done in a separate microservice.

This is intended to be a demo of how Rust could be used to build a gRPC microservice. It is not ready for production use in its current state, particularly security-wise.

## Technologies Used

*  [Rust](https://rust-lang.org)
*  [gRPC](https://grpc.io/) with [tonic](https://github.com/hyperium/tonic)
*  [Postgres](https://www.postgresql.org/) with [SQLx](https://github.com/launchbadge/sqlx)
*  Containerisation with [Docker](https://www.docker.com/), [Kubernetes](https://kubernetes.io/), and [Skaffold](https://github.com/GoogleContainerTools/skaffold)

## TODOs

Rough list of things that could be improved (time permitted)!

*  [ ] Implement Certificate Authority with [JWK](https://tools.ietf.org/html/rfc7517) support
*  [ ] Remove `JWT_SECRET` from `.env`. Sign and validate with a key issued from ^
*  [ ] More unit tests
*  [ ] More system tests (`authentication_test` crate)
*  [ ] Logging
*  [ ] Refactoring to reduce the size of files such as `database.rs` and `repository.rs`. Look to separate these out into modules for different types of data to reduce coupling.
    *  Refer to [1](https://github.com/clifinger/canduma) and [2](https://github.com/launchbadge/sqlx/tree/master/examples/realworld) for better structuring examples.

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
