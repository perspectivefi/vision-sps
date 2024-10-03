# Vision SPS (Substreams-Powered Subgraph)

This project implements a Substreams-powered subgraph for tracking ERC4626 token rates and APR calculations.

## Overview

The Vision SPS project uses Substreams to index ERC4626 tokens, also known as tokenized vaults, on the Ethereum blockchain. Specifically, it tracks:

1. Token metadata: decimals, symbol, and underlying asset address
2. Vault metrics: total assets and total supply
3. Conversion rates: the rate at which shares of the vault convert to underlying assets
4. APR calculations: based on changes in conversion rates over time

This indexing focuses on yield-bearing tokens and automated yield strategies, providing insights into the performance and efficiency of various DeFi protocols that implement the ERC4626 standard. The project processes historical data to retrieve and store these metrics, offering a comprehensive view of ERC4626 token behavior and returns over time.

## Prerequisites

- Rust (latest stable version)
- Node.js (v14 or later) and Yarn
- Substreams CLI (latest version)
- The Graph CLI (latest version)

## Project Structure

- `src/lib.rs`: Contains the main Substreams logic for processing ERC4626 token data
- `substreams.yaml`: Substreams configuration file
- `package.json`: Node.js package configuration and scripts
- `schema.graphql`: GraphQL schema for the subgraph
- `subgraph.yaml`: Subgraph manifest file

## Setup and Installation

1. Install Rust: https://www.rust-lang.org/tools/install
2. Install Node.js and Yarn: https://nodejs.org/ and https://yarnpkg.com/
3. Install Substreams CLI: Follow instructions at https://substreams.streamingfast.io/getting-started/installing-the-cli
4. Install The Graph CLI: `yarn global add @graphprotocol/graph-cli`

## Building and Running

1. Build the Substreams module:
   ```
   yarn substreams:prepare
   ```

2. Generate subgraph files:
   ```
   yarn codegen
   ```

3. Build the subgraph:
   ```
   yarn subgraph:build
   ```

4. Deploy the subgraph (replace `<SUBGRAPH_NAME>` with your subgraph name):
   ```
   yarn deploy <SUBGRAPH_NAME>
   ```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch: `git checkout -b feat/my-new-feature`
3. Commit your changes: `git commit -m 'feat: add some feature'`
4. Push to the branch: `git push origin feat/my-new-feature`
5. Submit a pull request

## License

This project is licensed under the [MIT License](LICENSE).
