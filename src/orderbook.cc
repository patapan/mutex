#include <cstdint>
#include <map>
#include <functional>

#include "types.h"

namespace mutex {

auto timePriceHigh = [](){};

class OrderBook {
    std::map<Order, std::vector<Order>, decltype(timePriceHigh)> bids;
    // bids are sorted by highest price first, then by time
    std::map<Order, std::vector<Order>, decltype(timePriceHigh)> asks;

    OrderBook() {

    }

    void place(Order& order) {

    }
};


} // namespace mutex

