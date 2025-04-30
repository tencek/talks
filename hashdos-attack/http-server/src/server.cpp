#define CPPHTTPLIB_REQUEST_URI_MAX_LENGTH 8192000
#include "httplib.h"

#define XXH_STATIC_LINKING_ONLY // expose unstable API
#include <xxhash.h>

#include "server.h"

void start_server(const char *host, int port)
{
    httplib::Server server;

    server.Get("/collision", handle_get_collision);

    server.Get("/hashdos", handle_get_hashdos);

    server.listen(host, port);
}

void handle_get_collision(const httplib::Request &request, httplib::Response &response)
{
    auto [hash_name, hash_fn] = create_hash_function(request.get_header_value("Hash-Fun"));
    for (const auto &param : request.params)
    {
        const auto hashed = hash_fn(param.first);
        response.set_header("Xxx-Hashed-" + param.first, std::to_string(hashed));
    }
    response.set_header("Xxx-Params-Count-Processed", std::to_string(request.params.size()));
    response.set_header("Xxx-Hash-Fun-Used", hash_name);
}

struct MyPoorHash
{
    std::size_t operator()(const std::string &str) const
    {
        return std::hash<std::string>()(str.substr(0, 6));
    }
};

void handle_get_hashdos(const httplib::Request &request, httplib::Response &response)
{
    // Use the custom hash function - the poor one
    auto collection = std::unordered_map<std::string, std::string, MyPoorHash>(request.params.begin(), request.params.end());
    response.set_header("Xxx-Params-Count-Processed", std::to_string(collection.size()));
}

class MyStrongHash
{
    unsigned char secret[XXH3_SECRET_SIZE_MIN];

public:
    MyStrongHash()
    {
        const auto now = std::chrono::system_clock::now().time_since_epoch().count();
        XXH3_generateSecret(secret, sizeof(secret), &now, sizeof(now));
    }
    size_t operator()(const std::string &str) const
    {
        return size_t{
            XXH3_64bits_withSecret(str.c_str(), str.length(), secret, sizeof(secret))};
    }
};

std::pair<std::string, HashFn> create_hash_function(const std::string &hash_type)
{
    std::unordered_map<std::string, std::function<std::pair<std::string, HashFn>()>> hash_functions = {
        {"poor", []()
         {
             return std::pair<std::string, MyPoorHash>("POOR", MyPoorHash());
         }},
        {"strong", []()
         {
             return std::pair<std::string, MyStrongHash>("STRONG", MyStrongHash());
         }},
    };

    // look into the hash_functions, return the function if found, otherwise return the std::hash function
    auto it = hash_functions.find(hash_type);
    if (it != hash_functions.end())
    {
        return it->second();
    }
    else
    {
        return std::pair<std::string, std::function<std::size_t(const std::string &)>>("FNV1a", std::hash<std::string>());
    }
}