### experiment 01 results
- nn: fc1(in_dim, 64) -> fc2(64, 1)
- epochs: 10000
- standardization: false
- learning rate decay scheduler: false
- layer normalization, dropout: false
- target scaling ($y$): false
- mini-batches: false

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
- learning rate decay scheduler: false
- layer normalization, dropout: false
- target scaling ($y$): false
- mini-batches: false

| metric | value |
| :--- | :--- |
| **train loss (mse)** | `0.1483` |
| **train accuracy** | `75.25%` |
| **test loss (mse)** | `0.3491` |
| **test mae** | `0.3990` |
| **test accuracy (within $\pm 20\%$)** | `59.84%` |

### experiment 03 results
- nn: fc1(in_dim, 128) -> fc2(128, 64) -> fc3(64, 1)
- epochs: 10000
- standardization: true
- learning rate decay scheduler: false
- layer normalization, dropout: false
- target scaling ($y$): false
- mini-batches: false

| metric | value |
| :--- | :--- |
| **train loss (mse)** | `0.1260` |
| **train accuracy** | `77.22%` |
| **test loss (mse)** | `0.3340` |
| **test mae** | `0.3922` |
| **test accuracy (within $\pm 20\%$)** | `60.78%` |

### experiment 04 results
- nn: fc1(in_dim, 128) -> fc2(128, 64) -> fc3(64, 1)
- epochs: 25000
- standardization: true
- learning rate decay scheduler: false
- layer normalization, dropout: false
- target scaling ($y$): false
- mini-batches: false

| metric | value |
| :--- | :--- |
| **train loss (mse)** | `0.1134` |
| **train accuracy** | `77.76%` |
| **test loss (mse)** | `0.3884` |
| **test mae** | `0.4110` |
| **test accuracy (within $\pm 20\%$)** | `59.71%` |

### experiment 05 results
- nn: fc1(in_dim, 128) -> fc2(128, 64) -> fc3(64, 1)
- epochs: 10000
- standardization: true
- learning rate decay scheduler: true
- layer normalization, dropout: false
- target scaling ($y$): false
- mini-batches: false

| metric | value |
| :--- | :--- |
| **train loss (mse)** | `0.1867` |
| **train accuracy** | `70.32%` |
| **test loss (mse)** | `0.2798` |
| **test mae** | `0.3579` |
| **test accuracy (within $\pm 20\%$)** | `65.19%` |

### experiment 06 results
- nn: fc1(in_dim, 128) -> fc2(128, 64) -> fc3(64, 1)
- epochs: 25000
- standardization: true
- learning rate decay scheduler: true
- layer normalization, dropout: true
- target scaling ($y$): false
- mini-batches: false

| metric | value |
| :--- | :--- |
| **train loss (mse)** | `0.6590` |
| **train accuracy** | `39.86%` |
| **test loss (mse)** | `0.6862` |
| **test mae** | `0.6232` |
| **test accuracy (within $\pm 20\%$)** | `39.10%` |

### experiment 07 results
- nn: fc1(in_dim, 128) -> fc2(128, 64) -> fc3(64, 1)
- epochs: 25000
- standardization: true
- learning rate decay scheduler: true
- layer normalization, dropout: false
- target scaling ($y$): false
- mini-batches: false

| metric | value |
| :--- | :--- |
| **train loss (mse)** | `0.1154` |
| **train accuracy** | `77.48%` |
| **test loss (mse)** | `0.3771` |
| **test mae** | `0.4050` |
| **test accuracy (within $\pm 20\%$)** | `59.57%` |

### experiment 08 results
- nn: fc1(in_dim, 128) -> fc2(128, 64) -> fc3(64, 1)
- epochs: 25000
- standardization: true
- learning rate decay scheduler: true
- layer normalization, dropout: false
- target scaling ($y$): true
- mini-batches: false

| metric | value |
| :--- | :--- |
| **train loss (mse)** | `0.0849` |
| **train accuracy** | `45.03%` |
| **test loss (mse)** | `0.4253` |
| **test mae** | `0.4260` |
| **test accuracy (within $\pm 20\%$)** | `59.38%` |

### experiment 09 results
- nn: fc1(in_dim, 128) -> fc2(128, 64) -> fc3(64, 1)
- epochs: 1000
- standardization: true
- learning rate decay scheduler: true
- layer normalization, dropout: false
- target scaling ($y$): true
- mini-batches: true

| metric | value |
| :--- | :--- |
| **train loss (mse)** | `0.0968` |
| **train accuracy** | `43.68%` |
| **test loss (mse)** | `0.6583` |
| **test mae** | `0.3786` |
| **test accuracy (within $\pm 20\%$)** | `65.41%` |

### experiment 09 results
- nn: fc1(in_dim, 128) -> fc2(128, 64) -> fc3(64, 1)
- epochs: 5000
- standardization: true
- learning rate decay scheduler: true
- layer normalization, dropout: false
- target scaling ($y$): true
- mini-batches: true

| metric | value |
| :--- | :--- |
| **train loss (mse)** | `0.0786` |
| **train accuracy** | `48.05%` |
| **test loss (mse)** | `0.4357` |
| **test mae** | `0.3882` |
| **test accuracy (within $\pm 20\%$)** | `65.62% ` |
