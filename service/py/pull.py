import pandas as pd
import baostock as bs
import mongo
import CalcDate


class pull:
    def __init__(self):
        self.has_save = {}
        date = CalcDate.CalcDate()
        date.update()
        self.date = date

    def get(self, code):
        data_list = []
        rs = bs.query_history_k_data_plus(code, "date,code,open,high,low,close,preclose,volume,amount,adjustflag,turn,tradestatus,pctChg,peTTM,pbMRQ,isST",
                                          start_date=self.date.get_sdate("start"), end_date=self.date.get_sdate("end"), frequency="d", adjustflag="3")
        while (rs.error_code == '0') & rs.next():
            data_list.append(rs.get_row_data())
        else:
            return data_list, rs

    def save_day(self, code):
        items = []
        list, rs = self.get(code)
        for item in list:
            data = {}
            i = 0
            for j in item:
                if rs.fields[i] == "date":
                    data["date"] = j
                    data["ddate"] = self.date.parse_date(j)
                elif rs.fields[i] == "code":
                    data["code"] = j
                elif rs.fields[i] in ["adjustflag", "tradestatus", "isST"]:
                    data[rs.fields[i]] = int(j)
                else:
                    try:
                        data[rs.fields[i]] = float(j)
                    except:
                        data[rs.fields[i]] = j
                        print('转换失败!   field:{}:value{}  '.format(
                            rs.fields[i], j))
                i += 1
            items.append(data)
        mongo.day_insert_many(items)

    def update_code(self):
        sdate = self.date.get_sdate()
        ddate = self.date.get_ddate()
        if self.date.get_ddate("end") < ddate+21:  # 21天才会更新一次全部代码
            print("没到21天,不会更新code表")
            return
        rs = bs.query_all_stock(day=sdate)
        items = []
        while (rs.error_code == '0') & rs.next():
            res = rs.get_row_data()
            items.append({"code": res[0], "update_at": ddate})
        else:
            if len(items) > 1000:
                print("down")
                mongo.code_drop()
                mongo.code_insert_many(items)
            else:
                print("今天不是开盘日,不会更新code表")


bs.login()
a = pull()
open = False
for i in range(5000):
    code = mongo.get_code()
    if not open:
        print("not open:", code)
    if code == None:
        break
    if code == "sh.600007":
        open = True
        continue
    if open:
        a.save_day(code)
        print("has down: ", code)
bs.logout()
