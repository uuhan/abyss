from tqsdk import TqApi

api = TqApi(web_gui=True)

quote = api.get_quote("SHFE.bu2012")

while True:
    print(quote.datetime, quote.last_price, quote.volume)
    api.wait_update()
