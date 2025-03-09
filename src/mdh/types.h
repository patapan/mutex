#pragma once

#include <cstdint>

enum Side {
    BID,
    ASK,
};

// A trade is an exchange of an asset for currency. It contains 2 orders.
struct Trade {
    uint64_t buy_id;
    uint64_t sell_id;
    int64_t price;
    uint64_t quantity;    
};