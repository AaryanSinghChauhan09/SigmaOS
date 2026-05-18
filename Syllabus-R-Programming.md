# R Programming → SigmaR Runtime

> Maps the R Programming syllabus to `SigmaR` — R language runtime embedded in SigmaOS for statistical computing and legal data science.

---

## Unit I: Fundamentals of R

```r
# R is embedded in SigmaOS via SigmaR runtime
# Access: sigma-r --repl  or  sigma run script.R

# Variables and Data Types
x <- 42              # integer/double
name <- "SigmaOS"    # character
flag <- TRUE         # logical
z <- 2 + 3i          # complex
n <- NULL            # null
na_val <- NA         # missing value

# Operators
10 + 3; 10 - 3; 10 * 3; 10 / 3
10 %% 3       # modulo: 1
10 %/% 3      # integer division: 3
10 ^ 3        # power: 1000
x == 10; x != 5; x > 5; x >= 10
x & TRUE; x | FALSE; !FALSE   # logical

# Vectors
v <- c(1, 2, 3, 4, 5)
seq_v <- seq(1, 10, by=2)         # 1 3 5 7 9
rep_v <- rep(c(1,2), times=3)     # 1 2 1 2 1 2
len <- length(v)                   # 5
names(v) <- c("a","b","c","d","e")
v["a"]                             # 1 (named indexing)
v[c(1,3,5)]                        # 1 3 5 (vector indexing)
v[v > 3]                           # 4 5 (logical indexing)

# Vector recycling
c(1,2,3,4) + c(10,20)  # = c(11, 22, 13, 24) — 10,20 recycled

# Matrices and Arrays
m <- matrix(1:9, nrow=3, ncol=3)
dim(m)                       # 3 3
nrow(m); ncol(m)             # 3; 3
t(m)                         # transpose
m %*% m                      # matrix multiplication
m[1, ]                       # row 1
m[ , 2]                      # column 2

arr <- array(1:24, dim=c(2,3,4))  # 3D array

# Lists
lst <- list(pid=42, name="sigma-ui", running=TRUE, cores=c(1,2))
lst$name              # "sigma-ui"
lst[["pid"]]          # 42
lst[[4]]              # c(1,2)
length(lst)           # 4

# NULL and Pair lists
pl <- pairlist(a=1, b=2)

# Data Frames
df <- data.frame(
  process = c("init","sigma-ui","sigma-net"),
  pid     = c(1, 42, 43),
  cpu_pct = c(0.1, 15.4, 2.3),
  running = c(TRUE, TRUE, FALSE)
)
df$process            # column access
df[1, ]               # row 1
df[df$running == TRUE, ]  # filter
nrow(df); ncol(df)

# Data Input
df_csv   <- read.csv("/sigma/data/metrics.csv")
df_scan  <- scan("/sigma/data/values.txt")
lines    <- readLines("/sigma/log/kernel.log")
```

---

## Unit II: Functions, Strings, Factors, Loops

```r
# Functions
get_uptime <- function(unit = "hours") {
  seconds <- system.time(Sys.sleep(0))[[3]]
  if (unit == "hours") return(seconds / 3600)
  return(seconds)
}

# Higher-order functions
apply(m, 1, sum)               # apply sum over rows
sapply(1:5, function(x) x^2)  # [1 4 9 16 25]
lapply(lst, class)             # list of types
Map("+", c(1,2,3), c(10,20,30)) # element-wise add

# Variable scope
counter <- 0
increment <- function() {
  counter <<- counter + 1    # <<- assigns to parent scope
}

# String operations
s <- "SigmaOS Zenith 15.1"
nchar(s)                      # 20
toupper(s); tolower(s)
substr(s, 1, 7)               # "SigmaOS"
strsplit(s, " ")[[1]]         # c("SigmaOS","Zenith","15.1")
gsub("15.1", "16.0", s)       # replace
paste("SigmaOS", "Zenith", sep=" ")
paste0("v", 15.1)              # "v15.1"
sprintf("PID %d: %.1f%% CPU", 42, 15.4)
file.path("/sigma", "data", "log.csv")  # path construction

# Factors
os_types <- factor(c("Linux","Windows","SigmaOS","Linux","SigmaOS"))
levels(os_types)        # "Linux" "SigmaOS" "Windows"
nlevels(os_types)       # 3
table(os_types)         # frequency count

ord_perf <- factor(c("low","high","medium","high"),
                   levels=c("low","medium","high"), ordered=TRUE)
ord_perf[1] < ord_perf[2]  # TRUE

# Flow Control
if (uptime > 720) {
  cat("Long running system\n")
} else if (uptime > 24) {
  cat("Normal uptime\n")
} else {
  cat("Recently restarted\n")
}

# Vectorized if
ifelse(v > 3, "high", "low")

# Loops
for (proc in df$process) cat(proc, "\n")
while (retries < 3) { if (connect()) break; retries <- retries + 1 }
repeat { val <- scan(n=1); if (val == 0) break }
```

---

## Unit III: Packages, Data & Visualization

```r
# Package management
install.packages("ggplot2")
library(ggplot2)
.libPaths()              # installed libraries

# External data
df_csv  <- read.csv("metrics.csv")
df_json <- jsonlite::fromJSON("config.json")
df_xl   <- readxl::read_excel("report.xlsx")
library(XML); df_xml <- xmlParse("data.xml")

# Working with dates
Sys.Date()
as.Date("2026-05-18")
format(Sys.Date(), "%d %B %Y")

# Database connectivity
library(RMySQL)
con <- dbConnect(MySQL(), user='sigma', password='pass',
                 dbname='sigmaos', host='localhost')
df <- dbGetQuery(con, "SELECT * FROM processes WHERE running=1")
dbDisconnect(con)

# Charts and Graphs
library(ggplot2)

# Histogram
ggplot(df, aes(x=cpu_pct)) +
  geom_histogram(bins=20, fill="#6C63FF") +
  theme_dark() + ggtitle("CPU Usage Distribution")

# Box plot
ggplot(df, aes(x=process, y=cpu_pct)) +
  geom_boxplot(fill="#4CAF50") + theme_minimal()

# Bar chart
ggplot(df, aes(x=process, y=cpu_pct, fill=process)) +
  geom_bar(stat="identity") + theme_dark()

# Line graph
ggplot(ts_df, aes(x=time, y=cpu)) +
  geom_line(color="#00BCD4", size=1.2) + theme_dark()

# Scatter plot
ggplot(df, aes(x=cpu_pct, y=mem_mb, color=process)) +
  geom_point(size=3) + theme_dark()

# Pie chart
pie(table(df$process), main="Process Distribution", col=rainbow(5))
```

---

## Unit IV: Legal Data Science with R

```r
library(stringr); library(tm); library(wordcloud)
library(igraph); library(e1071); library(class)

# Regular Expressions for legal text
legal_docs <- readLines("/sigma/legal/cases.txt")
citations <- str_extract_all(legal_docs, "\\d{4} [A-Z]+ \\d+")
sections  <- str_extract_all(legal_docs, "Section \\d+[A-Z]?")

# Text Corpus and preprocessing
corpus <- VCorpus(VectorSource(legal_docs))
corpus <- tm_map(corpus, content_transformer(tolower))
corpus <- tm_map(corpus, removeWords, stopwords("en"))
corpus <- tm_map(corpus, stemDocument)

# Term-Document Matrix
tdm <- TermDocumentMatrix(corpus)
freq <- rowSums(as.matrix(tdm))
wordcloud(names(freq), freq, max.words=50, colors=brewer.pal(8,"Dark2"))

# Legal Network Analysis
citation_graph <- graph_from_edgelist(citation_pairs)
most_cited <- which.max(degree(citation_graph, mode="in"))

# Document similarity (cosine)
dtm <- DocumentTermMatrix(corpus)
sim_matrix <- proxy::simil(as.matrix(dtm), method="cosine")
heatmap(as.matrix(sim_matrix))

# Naive Bayes classification of court decisions
nb_model <- naiveBayes(x=train_features, y=train_labels)
predictions <- predict(nb_model, test_features)

# SVM classification
svm_model <- svm(verdict ~ ., data=train_df, kernel="radial")
svm_pred <- predict(svm_model, test_df)

# KNN
knn_pred <- knn(train_features, test_features, train_labels, k=5)

# Model evaluation
conf_matrix <- table(Predicted=predictions, Actual=test_labels)
accuracy <- sum(diag(conf_matrix)) / sum(conf_matrix)
```

---

## SigmaR Integration

| R Feature | SigmaOS Integration |
|---|---|
| `ggplot2` | Renders via `SigmaViz` engine |
| `RMySQL` | Connects to `SigmaDB` |
| `readLines` | Reads from `SovereignFS` |
| `parallel` | Uses SigmaOS multi-core scheduler |
| `shiny` | SigmaWeb runtime integration |

*Last updated: 2026-05-18 | SigmaOS Zenith v15.1*
