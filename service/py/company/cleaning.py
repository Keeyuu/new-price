import pymongo


uri = ""
database = "sms"
supplier = "supplier"
shop = "shop"

myclient = pymongo.MongoClient(uri)
supplier_table = myclient[database][supplier]
shop_table = myclient[database][shop]

supplier_list = []
for item in supplier_table.find():
    supplier_list.append(item["supplier_id"])
index = 0
for item in shop_table.find():
    try:
        id_ = item["supplier_id"]
    except:
        id_ = "10001"
    else:
        if id_ not in supplier_list:
            index += 1
            print(item["shop_id"], "  ", index)
            if item["country"] == "VN":
                shop_table.update_one({"shop_id": item["shop_id"]}, {
                    "$set": {"supplier_id": "10001"}})
            elif item["country"] == "TR":
                shop_table.update_one({"shop_id": item["shop_id"]}, {
                    "$set": {"supplier_id": "10000"}})
