package config

import (
	"fmt"
	"io/ioutil"
	"os"
	"sync"

	"gopkg.in/yaml.v2"
)

type Config struct {
	// CommonHead []Data `yaml:"commonHead"`
	// Cookies    []Data `yaml:"cookies"`
	Mongo Mongo `yaml:"mongo"`
	// Size       int    `yaml:"size"`
}

// type Data struct {
// 	Key   string `yaml:"key"`
// 	Value string `yaml:"value"`
// }

type Mongo struct {
	Database      string `yaml:"database"`
	AllCode       string `yaml:"all_code"`
	CollectionDay string `yaml:"collection_day"`
	DaySize       int    `yaml:"day_size"`
	Uri           string `yaml:"uri"`
}

var config Config

var confMutex sync.Mutex

func Get() Config {
	return config
}

func init() {
	confMutex.Lock()
	defer confMutex.Unlock()
	fd, err := os.Open("./config/config.yaml")
	if err != nil {
		fmt.Println("1")
		panic(err)
	}
	filebytes, err := ioutil.ReadAll(fd)
	if err != nil {
		fmt.Println("2")
		panic(err)
	}
	err = yaml.Unmarshal(filebytes, &config)
	if err != nil {
		fmt.Println("3")
		panic(err)
	}
}
