# 🗼 radio-tower

Experimental remote frontend for [Transmission](https://github.com/transmission/transmission) using Dioxus liveview. The LiveView app runs the application logic in the Rust binary on the server and communicates layout updates to the FE via websockets. The server communicates with Transmission over its JSON-based RPC protocol. 

The project also demonstrates building tailwind css and embedding it in the compiled Rust binary.

## Getting started

```
nvm install 16 # first time
nvm use 16
make dev
```
