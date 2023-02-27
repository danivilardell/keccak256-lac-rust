/*
Copyright © 2020 ConsenSys

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

package mimc

import (
	"math/big"

	"github.com/consensys/gnark-crypto/ecc"
	bls12377 "github.com/consensys/gnark-crypto/ecc/bls12-377/fr/mimc"
	bls12381 "github.com/consensys/gnark-crypto/ecc/bls12-381/fr/mimc"
	bls24315 "github.com/consensys/gnark-crypto/ecc/bls24-315/fr/mimc"
	bn254 "github.com/consensys/gnark-crypto/ecc/bn254/fr/mimc"
	bw6761 "github.com/consensys/gnark-crypto/ecc/bw6-761/fr/mimc"

	"github.com/consensys/gnark/frontend"
)

var encryptFuncs map[ecc.ID]func(frontend.API, MiMC, frontend.Variable, frontend.Variable) frontend.Variable
var newMimc map[ecc.ID]func(string, frontend.API) MiMC

func init() {
	encryptFuncs = make(map[ecc.ID]func(frontend.API, MiMC, frontend.Variable, frontend.Variable) frontend.Variable)
	encryptFuncs[ecc.BN254] = encryptBN254
	encryptFuncs[ecc.BLS12_381] = encryptBLS381
	encryptFuncs[ecc.BLS12_377] = encryptBLS377
	encryptFuncs[ecc.BW6_761] = encryptBW761
	encryptFuncs[ecc.BLS24_315] = encryptBLS315

	newMimc = make(map[ecc.ID]func(string, frontend.API) MiMC)
	newMimc[ecc.BN254] = newMimcBN254
	newMimc[ecc.BLS12_381] = newMimcBLS381
	newMimc[ecc.BLS12_377] = newMimcBLS377
	newMimc[ecc.BW6_761] = newMimcBW761
	newMimc[ecc.BLS24_315] = newMimcBLS315
}

// -------------------------------------------------------------------------------------------------
// constructors

func newMimcBLS377(seed string, api frontend.API) MiMC {
	res := MiMC{}
	params := bls12377.NewParams(seed)
	for _, v := range params {
		var cpy big.Int
		v.ToBigIntRegular(&cpy)
		res.params = append(res.params, cpy)
	}
	res.id = ecc.BLS12_377
	res.h = api.Constant(0)
	res.api = api
	return res
}

func newMimcBLS381(seed string, api frontend.API) MiMC {
	res := MiMC{}
	params := bls12381.NewParams(seed)
	for _, v := range params {
		var cpy big.Int
		v.ToBigIntRegular(&cpy)
		res.params = append(res.params, cpy)
	}
	res.id = ecc.BLS12_381
	res.h = api.Constant(0)
	res.api = api
	return res
}

func newMimcBN254(seed string, api frontend.API) MiMC {
	res := MiMC{}
	params := bn254.NewParams(seed)
	for _, v := range params {
		var cpy big.Int
		v.ToBigIntRegular(&cpy)
		res.params = append(res.params, cpy)
	}
	res.id = ecc.BN254
	res.h = api.Constant(0)
	res.api = api
	return res
}

func newMimcBW761(seed string, api frontend.API) MiMC {
	res := MiMC{}
	params := bw6761.NewParams(seed)
	for _, v := range params {
		var cpy big.Int
		v.ToBigIntRegular(&cpy)
		res.params = append(res.params, cpy)
	}
	res.id = ecc.BW6_761
	res.h = api.Constant(0)
	res.api = api
	return res
}

func newMimcBLS315(seed string, api frontend.API) MiMC {
	res := MiMC{}
	params := bls24315.NewParams(seed)
	for _, v := range params {
		var cpy big.Int
		v.ToBigIntRegular(&cpy)
		res.params = append(res.params, cpy)
	}
	res.id = ecc.BLS24_315
	res.h = api.Constant(0)
	res.api = api
	return res
}

// -------------------------------------------------------------------------------------------------
// encryptions functions

// encryptBn256 of a mimc run expressed as r1cs
func encryptBN254(api frontend.API, h MiMC, message, key frontend.Variable) frontend.Variable {
	res := message
	// one := big.NewInt(1)
	for i := 0; i < len(h.params); i++ {
		tmp := api.Add(res, key, h.params[i])
		// res = (res+k+c)^5
		res = api.Mul(tmp, tmp)
		res = api.Mul(res, res)
		res = api.Mul(res, tmp)
	}
	res = api.Add(res, key)
	return res

}

// execution of a mimc run expressed as r1cs
func encryptBLS381(api frontend.API, h MiMC, message frontend.Variable, key frontend.Variable) frontend.Variable {

	res := message

	for i := 0; i < len(h.params); i++ {
		tmp := api.Add(res, key, h.params[i])
		// res = (res+k+c)^5
		res = api.Mul(tmp, tmp) // square
		res = api.Mul(res, res) // square
		res = api.Mul(res, tmp) // mul
	}
	res = api.Add(res, key)
	return res
}

// execution of a mimc run expressed as r1cs
func encryptBW761(api frontend.API, h MiMC, message frontend.Variable, key frontend.Variable) frontend.Variable {

	res := message

	for i := 0; i < len(h.params); i++ {
		tmp := api.Add(res, key, h.params[i])
		// res = (res+k+c)^5
		res = api.Mul(tmp, tmp) // square
		res = api.Mul(res, res) // square
		res = api.Mul(res, tmp) // mul
	}
	res = api.Add(res, key)
	return res

}

// encryptBLS377 of a mimc run expressed as r1cs
func encryptBLS377(api frontend.API, h MiMC, message frontend.Variable, key frontend.Variable) frontend.Variable {
	res := message
	for i := 0; i < len(h.params); i++ {
		tmp := api.Add(res, h.params[i], key)
		// res = (res+key+c)**-1
		res = api.Inverse(tmp)
	}
	res = api.Add(res, key)
	return res

}

// encryptBLS315 of a mimc run expressed as r1cs
func encryptBLS315(api frontend.API, h MiMC, message frontend.Variable, key frontend.Variable) frontend.Variable {
	res := message
	for i := 0; i < len(h.params); i++ {
		tmp := api.Add(res, h.params[i], key)
		// res = (res+k+c)^5
		res = api.Mul(tmp, tmp) // square
		res = api.Mul(res, res) // square
		res = api.Mul(res, tmp) // mul
	}
	res = api.Add(res, key)
	return res

}
