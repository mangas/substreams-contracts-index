help:
    just -l

deploy-local:
    graph create eth-contracts-index -g http://127.0.0.1:8020
    graph deploy eth-contracts-index subgraph.yaml -g http://127.0.0.1:8020 -i http://127.0.0.1:5001 -l v0.0.1

build:
    substreams build
    graph build subgraph.yaml

protoc:
    substreams protogen

tools:
    substreams -v 
    npm install -g @graphprotocol/graph-cli
