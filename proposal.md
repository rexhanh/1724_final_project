Team Members:
Yuanrong Han 1010787409
Lin Sheng 1010798585
Ziyue Gong 1005710740

<!-- API Key: 08GJX8AILBFV6R98
https://www.alphavantage.co/documentation/ -->

# Motivation

Our project is inspired by one of the ideas, "Personal Finance Tracker." One of our teammates recently got into managing his stock, and after seeing the provided ideas, he decided to build an app to track his stocks.

Our team gathered together because we are all interested in creating an app with Rust, and building such an app will be perfect because a stock tracking app will require a lot of design and front-end work, and we would like to explore frontend built in Rust with text user interface. Since creating this project requires calling real-time stock API, we have already found a free API to ensure this project can finish on time.

​​Lastly, we think Rust is a good programming language choice for our tool. With its support for concurrency and asynchronous requests, we can make large amounts of requests and gather stock information efficiently.

# Objective and key features

Our stock tracker allows users to search for stock, providing both list and detailed views, with relevant news and basic data insights for users to better interpret the stock price in context. The list view shows the latest stock prices, open, high, low, previous close, change, and change percent; the detailed view shows a time series change of stock price across time. We did not find an API that was free without delay, but we found one that was free but with some delay.
Users will be able to use this as a GUI version of it.
We will be using stock market data API from [alpha vantage](https://www.alphavantage.co/documentation/)
We have obtained an API key.
This app will be mainly built on **ratatui** and **request**

### GUI usage

We will also create a GUI for this tool using [ratatui](https://ratatui.rs/). The GUI will mainly consist of two parts: a list view on the left and a chart view on the right. At the top, there will be a search bar that users can add and search for the stock/ETF/index, which will show up in the list view on the left side.

1. Starting GUI
   `$finance --gui`

2. A list view of stocks/ETF/index
   On the left, a list view contains user-added stock/ETF/index, showing their names, current prices, and changes.
   **API will be used**: https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol=IBM&apikey=demo
   Example API return

   ```
   {
    "Global Quote": {
        "01. symbol": "IBM",
        "02. open": "209.5300",
        "03. high": "211.1200",
        "04. low": "204.2600",
        "05. price": "205.0400",
        "06. volume": "6953184",
        "07. latest trading day": "2024-10-30",
        "08. previous close": "210.4300",
        "09. change": "-5.3900",
        "10. change percent": "-2.5614%"
    }
   }
   ```

   So we will use the price, change, change percent from the returned object.
   **Will use List widget from ratatui**

3. A more detailed view on the right
   On the right, a chart showing daily movement will appear, or the user can choose to show a chart for a month, six months, or a year.
   **API will be used**: https://www.alphavantage.co/query?function=TIME_SERIES_INTRADAY&symbol=IBM&interval=5min&apikey=demo
   Example API return

   ```
   {
    "Meta Data": {
        "1. Information": "Intraday (5min) open, high, low, close prices and volume",
        "2. Symbol": "IBM",
        "3. Last Refreshed": "2024-10-29 19:55:00",
        "4. Interval": "5min",
        "5. Output Size": "Compact",
        "6. Time Zone": "US/Eastern"
    },
    "Time Series (5min)": {
        "2024-10-29 19:55:00": {
            "1. open": "210.4600",
            "2. high": "210.4600",
            "3. low": "210.3600",
            "4. close": "210.3600",
            "5. volume": "633"
        },
        ...
    }
   }
   ```

   We will be utilizing the dates, and the close price to render a chart.
   **Will use Chart widget from ratatui**

4. A more detailed view on the right: show more detail about that stock/ETF/index
   The bottom of the chart will show the open, high, low, previous close, change, and change percent.
   **API will be used**: https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol=IBM&apikey=demo
   This is the same API as in part 2, but here will use the high, low, open, and close.
5. Search bar & add
   You can search stock symbols and add them to the list view.
   **API will be used**:
   https://www.alphavantage.co/query?function=SYMBOL_SEARCH&keywords=tesco&apikey=demo
   Example API return:

   ```
   {
    "bestMatches": [
        {
            "1. symbol": "TSCO.LON",
            "2. name": "Tesco PLC",
            "3. type": "Equity",
            "4. region": "United Kingdom",
            "5. marketOpen": "08:00",
            "6. marketClose": "16:30",
            "7. timezone": "UTC+01",
            "8. currency": "GBX",
            "9. matchScore": "0.7273"
        },
        ...
    ]
   }
   ```

   We will be able to utilize the bestMatches to show a list of best matched stock/ETF/index.
   **Will use tui-textarea/tui-input and popup from ratatui**

6. News & Sentiment
   Show a piece of news about the stock
   **API will be used**:
   https://www.alphavantage.co/query?function=NEWS_SENTIMENT&tickers=AAPL&apikey=demo
   Example API return(shorter):
   ```
   {
      ...
      "feed": [
         {
            ...
            "title":"News Title",
            "url":"https://examplenews.com",
            "summary":"example summary",
            "banner_image":"https://exampleimage.png",
            "category_within_source":"category",
            "topics":[
               {
                  "topic":"topic 1",
                  ...
               },
               ...
            ]
         },
         ...
      ]
   }
   ```
   We will utilize to show the user relevant news, and the source, a short summary and an image if there is one.
   **Will use ratatui-image for image rendering**
7. Analytics insights
   Show top gainers and losers at the bottom of the app
   **API will be used**:
   https://www.alphavantage.co/query?function=ANALYTICS_FIXED_WINDOW&SYMBOLS=AAPL,MSFT,IBM&RANGE=2023-07-01&RANGE=2023-08-31&INTERVAL=DAILY&OHLC=close&CALCULATIONS=MEAN,STDDEV,CORRELATION&apikey=demo

### Optionally

We will try to add user authentication so users can log in to track their favorite stocks.

# Tentative plan

Yuanrong: will do part 2 list view and part 5 search bar
Lin: will do part 4 details and part 6 news
Ziyue: will part 7 data analytics: might use a different API and Rust crates

**Early November to mid-November**
Complete the basic GUI component: search bar, chart, and stock details.

**Mid-November to early December**
Complete the advanced GUI part of the project: data analytics, news, list view

**Early December to mid-December**
Test the entire project and finish the report.
