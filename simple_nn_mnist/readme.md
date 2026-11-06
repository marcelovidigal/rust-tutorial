### experiment 01 results
- nn: fc1(in_dim, 128) -> fc2(128, 64) -> fc3(64, classes)
- epochs: 1000
- standardization: false

| metric | value |
| :--- | :--- |
| **train loss (mse)** | `0.0198` |
| **train accuracy** | `99.62%` |
| **test loss (mse)** | `0.0932` |
| **test accuracy (within $\pm 20\%$)** | `97.46%` |

### experiment 02 results
- nn: fc1(in_dim, 128) -> fc2(128, 64) -> fc3(64, classes)
- epochs: 1000
- standardization: true

| metric | value |
| :--- | :--- |
| **train loss (mse)** | `0.0180` |
| **train accuracy** | `99.71%` |
| **test loss (mse)** | `0.0930` |
| **test accuracy (within $\pm 20\%$)** | `97.40%` |
