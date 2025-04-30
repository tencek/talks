#include <iostream>
#include "server.h"

int main(int argc, char **argv)
{
    std::cout << "Starting server on port 12345..." << std::endl;
    start_server("0.0.0.0", 12345);

    return 0;
}
