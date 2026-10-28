// Package inference pins the Inference Extension for gofacts' test: its
// imports keep go mod tidy from dropping the module.
package inference

import (
	_ "sigs.k8s.io/gateway-api-inference-extension/api/v1"
	_ "sigs.k8s.io/gateway-api-inference-extension/apix/v1alpha1"
)
