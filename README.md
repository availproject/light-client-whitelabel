# White label light client for the Avail network

## Installation

Download the latest Light Client build from the [releases](https://github.com/availproject/light-client-whitelabel/releases) page.

## Running

The easiest way to connect the light client to Avail mainnet is using the following command.

```bash
./light-client-whitelabel -n mainnet
```

The rest of the CLI options can be found [here](https://github.com/availproject/avail-light/blob/main/client/README.md#options).

For logging and metrics project name can be set in two ways:

1. Using the `project-name` CLI option:

```bash
./light-client-whitelabel -n mainnet --project-name example_project
```

2. Using the configuration file with the `project_name` parameter.

```toml
project_name = "example_project"
```

## Running the client using Docker 

To build the docker image you can run the following command:

```bash 
 docker build -t light-client-whitelabel:latest -f Dockerfile.release .
```

Once the image is built, you can run the image by supplying params:

```bash
docker run light-client-whitelabel:latest --network mainnet --project-name TestName
```

The `network` parameter is required to connect seamlessly; without it, ensure your config file includes all necessary p2p network components.
The rest of the configuration reference can be found [here](https://github.com/availproject/avail-light/blob/main/client/README.md#configuration-reference).

## Additional information

- Light client [overview](https://docs.availproject.org/docs/operate-a-node/run-a-light-client/Overview).
- Detailed [readme](https://github.com/availproject/avail-light/tree/main/client).
