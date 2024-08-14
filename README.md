# Substreams-powered subgraph: tracking contract creation

A basic Substreams-powered subgraph, including the Substreams definition. This example detects new
contract deployments on Ethereum, tracking the creation block and timestamp. There is a
demonstration of the Graph Node integration, using `substreams_entity_change` types and helpers.

## Prerequisites

This
[requires the dependencies necessary for local Substreams development](https://substreams.streamingfast.io/developers-guide/installation-requirements).

## Quickstart

```
yarn install # install graph-cli
yarn substreams:prepare # build and package the substreams module
yarn subgraph:build # build the subgraph
yarn deploy # deploy the subgraph
```

## Authentication

The Substreams-powered subgraph requires authentication to access the Substreams API. You can
authenticate by setting the `SUBSTREAMS_API_KEY` environment variable:

#### https://substreams.streamingfast.io/documentation/consume/authentication#authentication-with-the-graph-market
