import pymongo


uri = "mongodb://localhost:27017/"
database = "price"
code = "new_code"
day = "day"

myclient = pymongo.MongoClient(uri)
code_table = myclient[database][code]
day_table = myclient[database][day]

res = code_table.find()


def code_insert_one(item):
    code_table.insert_one(item)


def code_insert_many(items):
    code_table.insert_many(items)


def day_insert_many(items):
    day_table.insert_many(items)


def code_drop():
    code_table.drop()


def get_code():
    item = res.next()
    try:
        x = item["code"]
    except:
        x = None
    return x
