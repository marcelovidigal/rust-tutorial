### experiment 01 results
- nn: fc1(in_dim, 64) -> fc2(64, 1)
- epochs: 10000
- standardization: false

| metric | value |
| :--- | :--- |
| **train loss (mse)** | `0.2379` |
| **train accuracy** | `66.65%` |
| **test loss (mse)** | `0.2844` |
| **test mae** | `0.3613` |
| **test accuracy (within $\pm 20\%$)** | `64.53%` |

### experiment 02 results
- nn: fc1(in_dim, 128) -> fc2(128, 64) -> fc3(64, 1)
- epochs: 10000
- standardization: false

| metric | value |
| :--- | :--- |
| **train loss (mse)** | `0.1483` |
| **train accuracy** | `75.25%` |
| **test loss (mse)** | `0.3491` |
| **test mae** | `0.3990` |
| **test accuracy (within $\pm 20\%$)** | `59.84%` |

### experiment 02 results
- nn: fc1(in_dim, 128) -> fc2(128, 64) -> fc3(64, 1)
- epochs: 10000
- standardization: true

| metric | value |
| :--- | :--- |
| **train loss (mse)** | `` |
| **train accuracy** | `` |
| **test loss (mse)** | `` |
| **test mae** | `` |
| **test accuracy (within $\pm 20\%$)** | `` |
