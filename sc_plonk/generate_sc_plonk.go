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

	//scs : circuit in sparse r1cs format
	scs, err := frontend.Compile(ecc.BN254.ScalarField(), scs.NewBuilder, &circuit)
	if err != nil {
		return err
	}

	// Generate kzg global parameters trusted setup
	srs, err := test.NewKZGSRS(scs)
	if err != nil {
		return err
	}
	{
		f, err := os.Create("files/kzg.plonk.srs")
		if err != nil {
			return err
		}
		_, err = srs.WriteTo(f)
		if err != nil {
			return err
		}
	}

	//generate plonk vk and pk
	pk, vk, err := plonk.Setup(scs, srs)
	if err != nil {
		return err
	}
	{
		f, err := os.Create("files/cubic.plonk.vk")
		if err != nil {
			return err
		}
		_, err = vk.WriteTo(f)
		if err != nil {
			return err
		}
	}
	{
		f, err := os.Create("files/cubic.plonk.pk")
		if err != nil {
			return err
		}
		_, err = pk.WriteTo(f)
		if err != nil {
			return err
		}
	}
	// export solidity verifier
	{
		f, err := os.Create("files/contract_plonk.sol")
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

func main() {

	_ = generatePlonk()

}
