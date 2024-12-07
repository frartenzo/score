# A simple program to keep the score
Example:
```
score +10 'I have brushed my teeth'
```
Result:
```
Today's score is now 110.
```
## Features
- Every day starts with 100 points.

### Full list of examples

#### Add points to your today score:
```
score +10
```
Result:
```
Today's score is now 110.
```

#### Subtract points to your today score:
```
score -10
```
Result:
```
Today's score is now 90.
```

#### Add notes to logs:
```
score +30 'this is a note'
```

#### Get the score of today:
```
score today
```

#### Get a detailed report of today logs:
```
score report today
```
Result:
```
[Today's Report]
Monday, 15 November 2032

200 ┤╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶
    │
    │
    │
    │
150 ┤
    │
    │               ┌───┐
    │               │   └──
    │               │
100 ┤───────────────┘╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶
    │
    │
    │
    │
 50 ┤
    │
    │
    │
    │
  0 ┼───────────┬───────────┬───────────┬───────────┬
              06:00       12:00       18:00

Time    Score   Notes
=====   =====   =============  
08:13     +30   pushups
10:15     -10   ate chocolate
```