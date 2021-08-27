package data

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"os"
	"service/model"
)

func ReadAll() ([]model.Code, error) {
	datas := make([]model.Code, 0, 10000)
	fd, err := os.Open("./data/code.json")
	if err != nil {
		panic(err)
	}
	by, err := ioutil.ReadAll(fd)
	if err != nil {
		panic(err)
	}
	if err = json.Unmarshal(by, &datas); err != nil {
		fmt.Println(err)
		panic(err)
	}
	return datas, nil
}
