---
marp: true
footer: PPP.cpp
---
<!-- Barvou nadpisů první a druhé úrovně je _tmavá zelená_ z vizuálního stylu
používaného do r. 2025. Nová tmavá zelená je na většíně projektorů špatně vidět.
Často málo kontrastuje s bílou -->
<style>
img[alt~="logo"] {
  position: absolute;
  top: 10px;
  right: 10px;
  width: 210px;
}

h1 {
    color: #009645
}

h2 {
    color: #009645
}

pre {
    background: #f8f8f8
}

img[alt~="title-image"] {
  position: absolute;
  bottom: 50px;
  right: 50px;
  width: 150px;
}
</style>

# What is a HashDoS attack  

## and how to mitigate it in C++

![title-image](./img/logo-Cech++-final-cropped.jpg)

---

<!-- Odtud začínají být vidět čísla slidů -->
<!-- paginate: true -->

## Table of Contents

1. A short story – how it all began  
2. What is HashDoS? – theory & terms
3. Demo
4. Mitigation in C++

---

### How HashDoS came to light

- In **2011**, admins noticed strange slowdowns
- Investigation revealed excessive CPU usage caused by hash collisions  
- Root cause: predictable hash functions
- Publicly demonstrated by Klink & Wälde (28C3)
- Led to security patches across ecosystems

[HashDoS disclosure (oCERT)](https://ocert.org/advisories/ocert-2011-003.html)

---

## 2. What is HashDoS?

### Key concepts

- **Hashing** – maps input to fixed-size value
- **Collisions** – different inputs, same hash
- **Hash-based collections** – e.g. `unordered_map`
  - Collisions are expected and handled (e.g. linked list in a bucket)
- **HashDoS** – attacker deliberately triggers worst-case behavior

---

## 2. HashDoS Attack Mechanism

### What’s happening under the hood

- Attacker sends many colliding keys
- Hash table becomes unbalanced
- Lookup times degrade from O(1) to O(n)
- Server resources are exhausted
- Affects:
  - Web servers, APIs, microservices, ...
  - Any app processing untrusted input in `unordered_map` or similar

---

## 3. Demo

### Simulating a HashDoS attack

---

## Demo Setup

- Built with xmake (thanks to Michal Žamboch)
- Using my own HTTP server (using httplib package)
- Using Postman to send requests

---

## First: demonstrating hash collisions

- Collisions exist in **every** hash function
- Finding them **is not easy** in practice
- Two approaches:
  - Exploit structural weakness (e.g. poor mixing)
  - Brute force (requires CPU/GPU time)
- I hoped to find a ready-made FNV-1a multicollision list...
  > but apparently Google doesn’t index the darknet 🙂

---

## Simulating the HashDoS attack

- Using a bad hash function: `hash(substr(0,6))`
- Easy to create **multi-collisions**
- Inputs with same hash value → stress the server

---

## 4. How to mitigate HashDoS in C++

### Best practices

- ✅ Use 64-bit apps – wider `size_t`, more entropy
- ✅ Sanitize input – whitelist values, limit sizes
- ✅ Use beter hash functions if needed and seed it (xxHash, siphash)
- ✅ Use collections **not based on hashes** – e.g. `std::map`, `std::multimap`

---

## Custom hash function example

```cpp
struct MyHash {
    size_t operator()(const std::string& str) const {
        XXH3_64bits_withSeed(str.c_str(), str.length(), seed);
    }

    size_t seed = std::random_device{}(); // Randomized seed
};
```

```cpp
// use MyHash as a template parameter
std::unordered_map<std::string, int, MyHash> collection;
```

---

## 5. C++ Associative Containers

| Container                 | Type      | Ordering | Duplicate Keys | Lookup | Hash-based |
|---------------------------|-----------|----------|----------------|--------|------------|
| `std::unordered_map`      | Unordered | No       | No             | O(1)*   | ✅        |
| `std::unordered_multimap` | Unordered | No       | Yes            | O(1)*   | ✅        |
| `std::map`                | Ordered   | Yes      | No             | O(log n) | ❌        |
| `std::multimap`           | Ordered   | Yes      | Yes            | O(log n) | ❌        |

\* Average-case complexity; can degrade to O(n) under collisions

---

## 6. Resources & Links

- [Hash DoS Attack | PPT](https://www.slideshare.net/slideshow/hash-dos-attack/30145445)
- [CVE-2011-3414 - Microsoft Security](https://learn.microsoft.com/en-us/security-updates/securitybulletins/2011/ms11-100#collisions-in-hashtable-may-cause-dos-vulnerability---cve-2011-3414)
- [Breaking Hash Functions - orlp.net](https://orlp.net/blog/breaking-hash-functions/)
- [xxHash - example of usage](https://xxhash.com/doc/v0.8.2/group___x_x_h3__family.html#gaa895f02acfc127b1d34d0d558a6773b0)

---

## Thank you

### Questions?
