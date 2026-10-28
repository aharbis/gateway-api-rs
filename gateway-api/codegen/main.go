// Command codegen writes the IR the gateway-api crate is generated from, with
// the config in config.yaml. `cargo xtask generate` runs it.
package main

import "github.com/kube-rs/gateway-api-rs/codegen/gofacts"

func main() {
	gofacts.Main()
}
