# Motivation  
We are motivated to develop a tool in rust for Linkedin job posting analysis. 
As job seekers and Linkedin users ourselves, we find it hard to keep track 
of all details of jobs postings while developing an overall understanding 
of the job market; so we are motivated to develop a 
tool to tackle these problems. <br>

Specifically, our tool provides a summary table of job postings, each column is 
a job detail/feature (e.g. salary) that supports ordering. So that users can easily
find jobs that align with their skills and needs. <br>

Also, the tool provides visualizaiton (e.g. frequency of skills needed)that communicate 
the overall job market to our user directly. So that users can quickly understand what the market 
needs and improve skills accordingly. <br>

Lastly, we think rust is an good programming language choice for our tool. With its support for concurrency and asynchronous request, 
we can make requests to and gather information from large amounts job pages efficiently. 

# Objective and key features

The job tool crawls, parse and summarizes Linkedin job informaiton, based on user LinkedIn profile and job interest, so that the user can quickly grasp knowledge of the overall job market without the overhead of processing huge amount of data themselves.

## Feature1:
authenticate user login to LinkedIn
## Feature2:
allow user search job postings with criteria
## Feature3:
generate a summary table, each column is a job feature (location, company name, salaries), 
support ordering (alphabetic for text)

## Feature4:
visual insights, e.g. freqency chart of most wanted skills

## Crates to use: 
Spider: retrieve HTML content from pages (page, AIResults, Page)

Spider, configuration: bypass anti-robot  

Spider, module chrome-common, authchallengeResponse: user login to Linkedin  

Spider, module openai, get job search results  

Spider, packages, Scraper: HTML parsing  

Ratatui: text user interface  

tokio: async tasks, works with spider website config, max-concurrent & delay-btw-request

tui-rs: for visualization in text user interface

reqwest:request interested webpage



# Tentative plan
## 1. setup initial page scraping
handle redirect user Linkedin login i.e. feature 1<br>
handle user job search with criteria i.e. feature 2<br>
get page of a list of links to job postings<br>
extract (by HTML tages or CSS selector) links to a list

## 2. parse and store job details for analysis
for each posting link, open the URL and parse <br>
specific details such as location, company name, total_applicants, etc., 
store in JSON or sqlite for analysis

## 3. job data analysis and visualization
feature 3 & 4

Each team member works on one of the above parts.