package poseidon

import (
	"math/big"
	"testing"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark/backend"
	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/test"
)

func TestPoseidon(t *testing.T) {

	witness := poseidonCircuit{}
	witness.In = [1]frontend.Variable{1}
	//Out is 2311686534562335445660052947558801906779494465744002761606135193671983395497
	out := new(big.Int)
	out, ok := out.SetString("10224888382415584039696622724312659911103876386378895789246809299302747517746", 10)
	if !ok {
		t.Fatal("could not parse big int")
	}
	witness.Out = frontend.Variable(out)

	assert := test.NewAssert(t)
	assert.ProverSucceeded(&poseidonCircuit{}, &witness, test.WithCurves(ecc.BN254), test.WithBackends(backend.GROTH16, backend.PLONK))
}
