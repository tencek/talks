#include <iostream>
#include <httplib.h>

int main(int argc, char **argv)
{
    httplib::Server svr;

    svr.Get("/", [](const httplib::Request &, httplib::Response &res)
            { res.set_content("Hello, World!", "text/plain"); });

    std::cout << "Starting server on port 12345..." << std::endl;
    svr.listen("0.0.0.0", 12345);

    return 0;
}
