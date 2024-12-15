# ECE 1724 Final Project

## Members

**Team Members**:  
Yuanrong Han 1010787409 rex.han@mail.utoronto.ca rexhanh  
Lin Sheng 1010798585 jeff.sheng@mail.utoronto.ca OkayJeff5  
Ziyue Gong 1005710740 joy.gong@mail.utoronto.ca ZyeG 

## Motivation
We picked stocks as a project topic because the data is easily available by third party api and we wanted to focus on exploring usage of TUI and programming using rust in general.

We used text user interface (TUI) with a web-served endpoint for a detailed plot, because we wanted to combine the efficiency of TUI with enhanced visualization of web endpoints.

Feature highlights that differentiate our app from others:
- Memorized search history; so users can see previously searched stock without searching and adding to the list when the app restarts. 
- Identified and calculated crossover points for long/short SMA indicators, providing actionable insights for users instead of raw SMA data.

## Objectives

1. Stock Data Display: Provide users with stock prices, intraday, monthly and yearly data.
2. Multi-Screen Navigation: Offer a seamless navigation experience between multiple screens: main screen, search screen, analytics screen and general news screen.
3. User-Friendly Terminal Interface: Leverage Ratatui to create an aesthetically pleasing and intuitive interface.
4. News Integration: Incorporate relevant news articles related to stocks.
5. Analysis Integration: Incorporate technicial indicators for actionable insights.

## Features

### Feature 1: Main Screen

The home screen. Consists of 4 parts. On the left is a list view of stocks, which initially will be empty until the user adds a stock. On the top right is a view showing stock data in a line chart; users can choose between displaying intraday, monthly, or yearly data. On the bottom left is detailed information of the stock currently chosen. On the bottom right is a list of general news headlines; users can navigate through the list and enter into a detailed view.

**Initial Main Screen**
![alt text](images/main_screen.png)

**Main Screen Chart(After a stock is added)**
![alt text](images/chart.gif)

### Feature 2: Search Stock Screen

User will be able to search for a stock and add it to the main screen by the company name or stock symbol.
![alt text](images/search_screen.gif)

### Feature 3: General News Screen

Display the complete selected news.

![alt text](/images/news_screen.gif)

### Feature 4: Analytics Screen

Displays issuer/company info, long/short simple moving average of price chart, and top gainer list. 

Enter o to display a detailed view of the SMA chart, with crossover (golden and death) coordinates listed. 
- Golden Cross: shorter crosses above the longer (i.e. the value of the shorter moving average transitions from being below the longer moving average to being above it). A signal to buy.
- Death Cross: opposite, signal to sell.

![alt text](/images/analytics_screen.png)

![alt text](/images/analytics_web.png)
## User's Guide

### Start & Shutdown
- start: see below Reproducibility Guide
- shutdown: Hit `Esc`

### Keyboards Instructions
Hit `Esc` anytime to quit the app. There is a keyboard instruction at the bottom of each page.

Main screen: 
- `s`: to enter the search screen and add a stock.
- `left` and `right` arrow keys: to switch between the stock list and general news list.
- `up` and `down` arrow keys: to go through a list and select a list item.
- `d`, `m`, `y`: when a stock in the list is selected, switch among intraday, monthly and yearly price charts.
- `enter`:
when a stock in the stock list is selected, to enter its analytics screen. 
when a news item in the news list is selected, to enter its news screen.

Search Screen:
- to search and select a stock:
`i`：to insert text to the search box,`enter` to search for stocks, and most similar stocks will appear on the screen as a list. Users can use `up` and `down` arrow keys to select which stock to add, and hit `enter` to add it to the main screen.
- `h`: return to the main screen.

Analysis Screen:
- `o`: to view a detailed SMA plot, with crossover points marked and annotated.
- ↓↑ to scroll the gainer list.
- `h`: to return to the main screen

## Reproducibility Guide

```sh
git clone https://github.com/rexhanh/1724_final_project.git
cd 1724_final_project/finance
cargo run
```
## Contribution
Yuanrong Han
- search screen
- main screen: stock list view
- news screen

Lin Sheng
- main screen: stock detailed view
- price charts - daily, monthly, yearly

Ziyue Gong
- analysis screen
- web served endpoint for SMA plot

## Lessons Learned
### Limitation of TUI:
The TUI is good for fast rendering and showing trends, making it ideal for quick overviews and real-time monitoring. However, it is limited in providing precise plots and lacks many advanced features offered by more mature libraries, as its visualizations are rendered at the pixel level. To address these limitations, we included a web-served endpoint powered by plotters. This allows users to access detailed, high-quality plots where they can read exact values and extract deeper insights from the data, going beyond the trends displayed in the TUI.

### Error Handle with TUI:
Error output to stderr can interfere with the TUI display, causing interruptions. To address this, we configured the fern crate to log errors and info messages to a file (error.log), ensuring the TUI remains unaffected while preserving logs for debugging and tracking.

### Availability of Third Party Service:
The API service has been reliable during our work with it; but it could be a potential concern. We can have better handling.

During implementation, we noticed some unlisted stock is still available by the service, so a user can search and add such a stock to the list, but the data for plots are missing since it is no longer listed. We added additional logic to display an alert message in this case.

## Video Demo

[![DEMO](https://img.youtube.com/vi/DmkLOqRHKGI/0.jpg)](https://www.youtube.com/watch?v=DmkLOqRHKGI)
