package main

import (
	"os"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark/backend/groth16"
	"github.com/consensys/gnark/examples/cubic"
	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/frontend/cs/r1cs"
)

// run this from /integration/solidity to regenerate files
// note: this is not in go generate format to avoid solc dependency in circleCI for now.
// go run contract/main.go && abigen --sol contract.sol --pkg solidity --out solidity.go
func main() {
	// var witness cubic.Circuit
	// witness.X.Assign(3)
	// witness.Y.Assign(35)
	var circuit cubic.Circuit
	r1cs, _ := frontend.Compile(ecc.BN254.ScalarField(), r1cs.NewBuilder, &circuit)

	_, vk, _ := groth16.Setup(r1cs)

	f, _ := os.Create("contract_g16.sol")

	_ = vk.ExportSolidity(f)

}
