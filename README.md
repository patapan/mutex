# mutex

The intention behind this repository is to design a system to ingest market data from crypto exchanges in a low latency manner with hardware CPU clock cycle benchmarks. The goal is to ingest at similar speeds to the NASDAQ at peak TPS (100,000+).

This design is a proof of concept.

The orderbook design will be influenced in part on the principles outlined by David Gross in his 2024 cppcon [video](https://www.youtube.com/watch?v=sX2nF1fW7kI) in terms of spatial memory optimization and look ups.

Additional features:

- [ ] Atomic matching engine
- [ ] MPMC Ringbuffer for incoming order stream
- [ ] Utilization of a thread pool for the orderbook 
- [ ] Multicast UDP streaming of completed Trades 
