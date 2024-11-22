#include <cstdint>

enum Side {
    BID,
    ASK,
};

struct Order {
    Side side;
    int64_t price;
    uint64_t quantity;
    int64_t filled_quantity;
};

