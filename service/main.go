package main

import (
	"context"
	"service/config"
	"service/dao/db"
	"service/data"
	"service/util"
)

func main() {
	list, err := data.ReadAll()
	if err != nil {
		return
	}
	col := db.GetCollection(config.Get().Mongo.AllCode)
	for _, c := range list {
		c.UpdateAt = util.GetDay()
		if _, err := col.InsertOne(context.TODO(), c); err != nil {
			println(err)
			continue
		}
	}

}
