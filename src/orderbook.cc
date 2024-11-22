#include <cstdint>
#include <map>
#include <functional>

#include "types.h"

namespace mutex {

struct Order { 
    int64_t price;
    uint64_t quantity;
    int64_t filled_volume;
};

class OrderBook {
    std::map<Order, std::vector<Order>, std::greater> bids;
    std::map<Order, std::vector<Order>, std::greater> asks;

    OrderBook() {

    }

    void place(Order& order) {

    }
};


} // namespace mutex

