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
we can make large amounts of requests to and gather job information efficiently. 

# Objective and key features

## Featur1:
allow user search job postings with criteria

## Feature2:
generate a summary table, each column is a job feature (location, company name, salaries), 
support ordering (alphabetic for text)
## Feature3:
visual insights, e.g. freqency chart of most wanted skills

## Crates to use: 
Scraper: HTML parsing

Ratatui: text user interface  

tokio: async tasks, works with spider website config, max-concurrent & delay-btw-request

reqwest:request interested webpage

# Tentative plan
## 1. async request to get job list
request to get a list of jobs based on user searching criteria, from which 
extract a list of job IDs (by HTML tages or CSS selector)
example request 
```
https://www.linkedin.com/jobs-guest/jobs/api/seeMoreJobPostings/search?keywords=developer
```
## 2. async requests to get job details, parse and store for analysis
for each job ID, request details at
```
https://www.linkedin.com/jobs-guest/jobs/api/jobPosting/4046539572
```

## 3. job data analysis and visualization

## 4. user interface
allow user input job criteria, to get analysis target
allow user view summary table (ordering, pagination, etc)
allow user to select which graph to draw