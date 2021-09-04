from re import split
import time


class CalcDate():
    def __init__(self):
        self.date = {}
        self.date["code"] = {"year": 2021, "month": 9, "day": 1}
        self.date["start"] = {"year": 2019, "month": 8, "day": 25}
        self.date["end"] = {"year": 2021, "month": 9, "day": 1}

    def update(self, index="code"):
        self.date[index] = {"year": int(time.strftime(
            "%Y", time.localtime())), "month": int(time.strftime("%m", time.localtime())), "day": int(time.strftime("%d", time.localtime()))}

    def get_sdate(self, index="code"):
        date = self.date[index]
        return '{}-{}-{}'.format(date["year"], date["month"], date["day"])

    def get_ddate(self, index="code"):
        date = self.date[index]
        return date["year"]*10000+date["month"]*100+date["day"]

    def parse_date(self, string):
        try:
            tmp = str.split(string, "-")
            return int(tmp[0])*10000+int(tmp[1])*100+int(tmp[2])
        except:
            return 99999
