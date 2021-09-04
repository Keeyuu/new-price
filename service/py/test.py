import baostock as bs
import pandas as pd

lg = bs.login()
# print('login respond error_code:'+lg.error_code)
print('login respond  error_msg:'+lg.error_msg)

rs = bs.query_history_k_data_plus("sh.600000",
                                  "date,code,open,high,low,close,preclose,volume,amount,adjustflag,turn,tradestatus,pctChg,peTTM,pbMRQ,psTTM,pcfNcfTTM,isST",
                                  start_date='2021-09-03', end_date='2021-09-03',
                                  frequency="d", adjustflag="3")
print('query_history_k_data_plus respond  error_msg:'+rs.error_msg)

data_list = []
while (rs.error_code == '0') & rs.next():
    data_list.append(rs.get_row_data())
result = pd.DataFrame(data_list, columns=rs.fields)
# result.to_json("history_k_data.json",)
print("--------------------------")
print(data_list)
print("--------------------------")
print(result)
print("--------------------------")

bs.logout()
