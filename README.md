# Binance Rust AWS (In Developing)
This repo is a to provde a set of API or websocket tools for people to easily develop Algo trading system to Binance. Temperarily, we are focusing on order updates and bookticker updates API as these are the heart of an order management system.


# Example Usage
Suppose that you put your `Binance API key` and `Binance API Secret` into AWS parameter store and set the local machine IP in Binance API management, you can run following code to collect order updates of your Binance account by

```bash
export AWS_PROFILE=YOUR_AWS_PROFILE && export AWS_REGION=REGION && AWS_ACCOUNT_ID=YOUR_ACCOUNT_ID
cargo run --release --bin order_update
```

Besides, you can also get the orderbook updates using
```bash
export AWS_PROFILE=YOUR_AWS_PROFILE && export AWS_REGION=REGION && AWS_ACCOUNT_ID=YOUR_ACCOUNT_ID
cargo run --release --bin bookticker
```

Order updates and Bookticker are important to trading system. For instance, if you place an limit buy order with price lower than current mid price by 10%, this order no longer be able to hit by market sell order and the order becomes stale order. Hence, at this time, an Order Management System (OMS) may need to update the Order price in order to make the order to be filled as fast as possible. This case happens when the market movement is very fast.


# Follow Up Tasks:
- Order Management Engine to keep track of order life cycle and deviation of order price and current best bid/ask prices.
- DB setup for data logging:  Log order terminal state to calcualte P&L subsequently ( like Grafana)
- FrontEnd to display real-time Orderbook updates and prices (if there is a DB, then we can concat data to make different bars)
-
