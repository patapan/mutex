#include <cstdint>
#include <map>
#include <functional>

#include "types.h"
#include "order.h"

namespace mutex {

class OrderBook {
    // Bids are sorted by highest price first, then by time.
    std::map<Order, std::vector<Order>, std::greater<Order>> bids;
    std::map<Order, std::vector<Order>, std::less<Order>> asks;

    OrderBook() {

    }

    void place(Order& order) {

    }
};


} // namespace mutex

