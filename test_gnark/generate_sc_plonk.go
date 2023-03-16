package main

import (
	"os"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark/backend/plonk"
	"github.com/consensys/gnark/examples/cubic"
	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/frontend/cs/scs"
	"github.com/consensys/gnark/test"
)

func generatePlonk() error {
	var circuit cubic.Circuit

	scs, err := frontend.Compile(ecc.BN254.ScalarField(), scs.NewBuilder, &circuit)
	if err != nil {
		return err
	}

	srs, err := test.NewKZGSRS(scs)
	if err != nil {
		return err
	}
	{
		f, err := os.Create("kzg.plonk.srs")
		if err != nil {
			return err
		}
		_, err = srs.WriteTo(f)
		if err != nil {
			return err
		}
	}

	pk, vk, err := plonk.Setup(scs, srs)
	if err != nil {
		return err
	}
	{
		f, err := os.Create("cubic.plonk.vk")
		if err != nil {
			return err
		}
		_, err = vk.WriteTo(f)
		if err != nil {
			return err
		}
	}
	{
		f, err := os.Create("cubic.plonk.pk")
		if err != nil {
			return err
		}
		_, err = pk.WriteTo(f)
		if err != nil {
			return err
		}
	}

	{
		f, err := os.Create("contract_plonk.sol")
		if err != nil {
			return err
		}
		err = vk.ExportSolidity(f)
		if err != nil {
			return err
		}
	}
	return nil
}

// run this from /integration/solidity to regenerate files
// note: this is not in go generate format to avoid solc dependency in circleCI for now.
// go run contract/main.go && abigen --sol contract.sol --pkg solidity --out solidity.go
func main() {

	// var circuit cubic.Circuit
	// r1cs, _ := frontend.Compile(ecc.BN254.ScalarField(), r1cs.NewBuilder, &circuit)

	// srs, _ := test.NewKZGSRS(r1cs)

	// _, vk, _ := plonk.Setup(r1cs, srs)

	// f, _ := os.Create("contract_plonk.sol")

	// _ = vk.ExportSolidity(f)

	_ = generatePlonk()

}
