# SigmaOS Roadmap: Time-Series Forecasting
Forecast CPU, memory, and IO load trends using embedded ARIMA and LSTM.
## Goals
- ARIMA model fitting on rolling telemetry windows
- LSTM 8-step lookahead for scheduler hints
## Key Milestones
- [ ] Rolling window ring buffer
- [ ] ARIMA order selection (AIC/BIC)
- [ ] LSTM single-layer Rust implementation