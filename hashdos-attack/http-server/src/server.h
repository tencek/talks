#ifndef SERVER_H
#define SERVER_H

#include <httplib.h>

void start_server(const char *host, int port);
void handle_get_collision(const httplib::Request &req, httplib::Response &res);
void handle_get_hashdos(const httplib::Request &req, httplib::Response &res);

typedef std::function<std::size_t(const std::string &)> HashFn;
std::pair<std::string, HashFn> create_hash_function(const std::string &hash_type);

#endif // SERVER_H