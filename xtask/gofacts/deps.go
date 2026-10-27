package main

// The packages gofacts loads, imported so go.mod pins their version.
import (
	_ "k8s.io/apimachinery/pkg/apis/meta/v1"
	_ "sigs.k8s.io/gateway-api/apis/v1"
	_ "sigs.k8s.io/gateway-api/apis/v1alpha2"
	_ "sigs.k8s.io/gateway-api/apis/v1alpha3"
	_ "sigs.k8s.io/gateway-api/apis/v1beta1"
	_ "sigs.k8s.io/gateway-api/apisx/v1alpha1"
)
