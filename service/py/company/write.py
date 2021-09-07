import pymongo
import random

uri = "mongodb://root:8DNsidknweoRGwSbWgDN@localhost:27019"
database = "sms"
shop = "shop"

myclient = pymongo.MongoClient(uri)
shop_table = myclient[database][shop]
for item in shop_table.find():
    shop_table.update_one({"shop_id": item["shop_id"]}, {
        "$set": {"cfo": random.randint(1, 100)}})
