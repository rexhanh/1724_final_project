# Motivation

# Objective and key features

Our objecttive is to make an app that shows a view of stocks' latest prices, open, high, low, previous close, change, and change percent. We did not find an API that is free without delay, but we find an API that is free but with some delay.
User will be able to use this as a CLI tool, and also a GUI version of it.
CLI usage.
We will be using stock market data API from [alpha vantage](https://www.alphavantage.co/documentation/)
We have obtained an API key.

### Basic CLI usage

Getting latest price of a stock/ETF/index

```
$finance --check AAPL
{
    "open": "229.7400",
    "high": "233.2200",
    "low": "229.5700",
    "price": "231.4100",
    "latest trading day": "2024-10-25",
    "previous close": "230.5700",
    "change": "0.8400",
    "change percent": "0.3643%"
}
```

If the symbol does not exist

```
$finance --check AAPL1
Symbol AAPL1 does not exit
```

If finds multiple similar ones, it shows the top 5 most similar ones.

```
$finance --check apple
Do you mean APLE, AAPL, AAPL34.SAO, APC.DEX, APC.FRK?
```

### GUI usage

We will also make a GUI of this tool using [ratatui](https://ratatui.rs/), starting GUI by using
`$finance --gui`

The GUI will mainly contain two part, on the left will be a list view, on the right will be a chart view. At the top, there will be a search bar that user can add and search for the stock/ETF/index, and it will show up in list view on the left side.

**Will use tui-textarea/tui-input**

The list view contains multiple stock/ETF/index, showing its name, current price, and change.

**Will use List widget from ratatui and popup from ratatui**

On the right will be a chart showing daily moving, or the user can choose to show chart of a month, six month or a year.

**Will use Chart widget from ratatui**

On the bottom of the chart will show the open, high, low, previous close, change, and change percent.

# Tentative plan

Early November to mid November
Complete the command line interface part of the project. If CLI part finishes early, we will move on to GUI earlier.

Mid November to early December
Complete the GUI part of the project

Early December to mid December
Complete the entire project, finishing the report.
