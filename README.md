# ECE 1724 Final Project

## Members

**Team Members**:  
Yuanrong Han 1010787409  
Lin Sheng 1010798585  
Ziyue Gong 1005710740

## Motivation

## Objectives

1. Stock Data Display: Provide users with stock prices, intraday, monthly and yearly data.
2. Multi-Screen Navigation: Offer a seamless navigation experience between multiple screens: main screen, search screen, analytics screen and general news screen.
3. User-Friendly Terminal Interface: Leverage Ratatui to create an aesthetically pleasing and intuitive interface.
4. News Integration: Incorporate relevant news articles related to stocks.

## Features

### Feature 1: Main Screen

The first screen is the main screen. Main screen consists of 4 parts. On the left is a list view of stocks, initially will be empty until the user add a stock. On the top right is a view showing stock data in a line chart, user can choose between displaying intraday, monthly, or yearly data. The bottom right shows a detailed information of the stock currently choose

**Initial Main Screen**
![alt text](images/main_screen.png)

**Main Screen Chart(After a stock is added)**
![alt text](images/chart.gif)

### Feature 2: Search Stock Screen

User will be able to search for a stock and add it to the main screen by the company name or stock symbol.
![alt text](images/search_screen.gif)

### Feature 3: General News Screen

![alt text](/images/news_screen.gif)

### Feature 4: Analytics Screen

![alt text](/images/analytics_screen.png)

## User's Guide

**Main Screen**
Start the app by `cargo run`, you will be directed to the main screen.

Initial main screen will be mostly empty because no stocks are added. User can hit `s` to enter search screen.

If there is a stock in the list, you can use `up` and `down` arrow keys to select different stocks (if more than one stock added).

Once a stock is selected, you can hit `enter` to enter the anylytics screen.

Use `d`, `m`, `y` keys to switch between intraday, monthly and yearly chart if a stock is selected.

Use `left` and `right` arrow key to switch between the stock list and general news list.

Once a news item is selected, you can hit `enter` to enter general news screen.

You can hit `Esc` to quit the app.

**Search Screen**
Once in search screen, you can hit `i` to insert text to search for a stock. After finishing the inputs, hit `enter` to search for stocks, and most similar stocks will appear on the screen as a list. User can use `up` and `down` arrow keys to select which stock to add, and hit `enter` to add it to the main screen.

You can hit `h` to go back to main screen, or `Esc` to quit the app.

**Analytics Screen**
You can hit `h` to go back to main screen, or `Esc` to quit the app.

**General News Screen**
You can hit `h` to go back to main screen, or `Esc` to quit the app.

**At the bottom of each screen, there will be an instruction at the bottom.**

## Reproducibility Guide

```sh
git clone https://github.com/rexhanh/1724_final_project.git
cd 1724_final_project/finance
cargo run
```

## Lessons Learned

## Video Demo

[![DEMO](https://img.youtube.com/vi/g3cMjUPhcEE/0.jpg)](https://www.youtube.com/watch?v=g3cMjUPhcEE)
