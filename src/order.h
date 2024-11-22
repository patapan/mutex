#pragma once

#include <cstdint>

struct Order { 
    int64_t price;
    uint64_t quantity;
    int64_t filled_volume;
    int64_t entry_timestamp;
};