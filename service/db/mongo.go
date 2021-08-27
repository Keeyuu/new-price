package db

import (
	"context"
	"fmt"
	"service/config"

	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
	"go.mongodb.org/mongo-driver/mongo/readpref"
)

var mg *mongo.Database

func GetMg() *mongo.Database {
	return mg
}

func init() {
	initMongo()
	createIndex(mg)
}

func createIndex(m *mongo.Database) {
	c_main := mg.Collection(config.Get().Mongo.AllCode)
	c_main.Indexes().CreateMany(context.TODO(), []mongo.IndexModel{
		{
			Keys:    bson.D{{"code", 1}},
			Options: options.Index().SetUnique(true).SetName("code"),
		},
		{
			Keys:    bson.D{{"roe", 1}},
			Options: options.Index().SetName("roe"),
		},
	})
	// for i := 1; i <= config.Get().Mongo.DaySize; i++ {
	// 	c_day := mg.Collection(fmt.Sprintf("%s:%d", config.Get().Mongo.AllCode, i))
	// 	c_day.Indexes().CreateMany(context.TODO(), []mongo.IndexModel{
	// 		{
	// 			Keys:    bson.D{{"code", 1}},
	// 			Options: options.Index().SetName("code"),
	// 		},
	// 		{
	// 			Keys:    bson.D{{"day", 1}},
	// 			Options: options.Index().SetName("day"),
	// 		},
	// 	})
	// }
}

func initMongo() {
	// uri := "mongodb+srv://<username>:<password>@<cluster-address>/test?w=majority"
	client, err := mongo.Connect(context.Background(), options.Client().ApplyURI(config.Get().Mongo.Uri))
	if err != nil {
		panic(err)
	}
	// Ping the primary
	if err := client.Ping(context.TODO(), readpref.Primary()); err != nil {
		panic(err)
	}
	mg = client.Database(config.Get().Mongo.Database)
	createIndex(mg)
	fmt.Println("Successfully connected and pinged.")
}
