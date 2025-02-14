#!/bin/bash

# Build and deploy Solana program
anchor build
anchor deploy --provider.cluster mainnet

# Verify deployment
solana program show --program-id target/deploy/space_gpt_program-keypair.json
