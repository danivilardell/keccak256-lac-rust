package poseidon

import (
	"github.com/consensys/gnark/frontend"
)

// NROUNDSF constant from Poseidon paper
const NROUNDSF = 8

// NROUNDSP constant from Poseidon paper
var NROUNDSP = []int{56, 57, 56, 60, 60, 63, 64, 63, 60, 66, 60, 65, 70, 60, 64, 68}

const spongeChunkSize = 31
const spongeInputs = 16

type poseidonCircuit struct {
	In  [1]frontend.Variable
	Out frontend.Variable `gnark:",public"`
}

func (p *poseidonCircuit) Define(api frontend.API) error {
	t := len(p.In) + 1

	nRoundsF := NROUNDSF
	nRoundsP := NROUNDSP[t-2]

	C := c.c[t-2]
	S := c.s[t-2]
	M := c.m[t-2]
	P := c.p[t-2]

	state := make([]frontend.Variable, t)
	state[0] = frontend.Variable(0)

	for i := 1; i <= len(p.In); i++ {
		state[i] = p.In[i-1]
	}

	state = ark(api, state, C, 0)

	for i := 1; i < nRoundsF/2; i++ {
		state = exp5state(api, state)
		state = ark(api, state, C, i*t)
		state = mix(api, state, t, M)
	}

	state = exp5state(api, state)
	state = ark(api, state, C, (nRoundsF/2)*t)
	state = mix(api, state, t, P)

	for i := 0; i < nRoundsP; i++ {
		state[0] = exp5(api, state[0])
		state[0] = api.Add(state[0], C[(nRoundsF/2+1)*t+i])

		newState0 := frontend.Variable(0)
		for j := 0; j < len(state); j++ {
			newState0 = api.Add(newState0, api.Mul(S[(t*2-1)*i+j], state[j]))
		}

		for k := 1; k < t; k++ {
			state[k] = api.Add(state[k], api.Mul(state[0], S[(t*2-1)*i+t+k-1]))
		}
		state[0] = newState0
	}

	for i := 0; i < nRoundsF/2-1; i++ {
		state = exp5state(api, state)
		state = ark(api, state, C, (nRoundsF/2+1)*t+nRoundsP+i*t)
		state = mix(api, state, t, M)
	}

	state = exp5state(api, state)
	state = mix(api, state, t, M)

	api.AssertIsEqual(p.Out, state[0])
	return nil
}

func mix(api frontend.API, state []frontend.Variable, t int, m [][]frontend.Variable) []frontend.Variable {
	res := make([]frontend.Variable, t)
	for i := 0; i < t; i++ {
		res[i] = frontend.Variable(0)
	}
	for i := 0; i < len(state); i++ {
		for j := 0; j < len(state); j++ {
			res[i] = api.Add(res[i], api.Mul(m[i][j], state[j]))
		}
	}
	return res
}

func ark(api frontend.API, state []frontend.Variable, c []frontend.Variable, it int) []frontend.Variable {
	res := make([]frontend.Variable, len(state))
	for i := 0; i < len(state); i++ {
		res[i] = api.Add(state[i], c[it+i])
	}
	return res
}

func exp5state(api frontend.API, state []frontend.Variable) []frontend.Variable {
	res := make([]frontend.Variable, len(state))
	for i := 0; i < len(state); i++ {
		res[i] = exp5(api, state[i])
	}
	return res
}

func exp5(api frontend.API, a frontend.Variable) frontend.Variable {
	return api.Mul(a, api.Mul(a, api.Mul(a, api.Mul(a, a))))
}
