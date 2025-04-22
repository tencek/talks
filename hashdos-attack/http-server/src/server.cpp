#include "server.h"

void start_server(const char *host, int port)
{
    httplib::Server svr;

    svr.Get("/", [](const httplib::Request &, httplib::Response &res)
            { res.set_content("Hello, World!", "text/plain"); });

    svr.listen(host, port);
}