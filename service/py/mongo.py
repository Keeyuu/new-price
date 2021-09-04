import pymongo


uri = "mongodb://localhost:27017/"
database = "price"
table = "code"


myclient = pymongo.MongoClient(uri)
mytable = myclient[database][table]


res = mytable.find()


def get_code():
    item = res.next()
    try:
        x = item["data"]["code"]
    except:
        x = None
    return x


a = get_code()


print(a)
