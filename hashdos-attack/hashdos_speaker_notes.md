# What is a HashDoS attack and how to mitigate it in C++

## Slide 1: Title

Hello everyone. Thank you for coming to this talk.

First things first — I want to apologize for rescheduling this talk.
I want to be honest with you: I had a rough draft of the talk in mind for a while,
but I left the final preparation for the day before the original date.

I seriously underestimated how much time the topic would take.
At 2 a.m. last Wednesday, I barely had the demo working — and had no slides at all.
So I gave up and rescheduled. I’m sorry for the inconvenience, and I really appreciate your patience.

Anyway, today, I'd like to explain what the HashDoS attack is, how it works, and propose some mitigation techniques in C++.

## ## Slide 2: Table of Contents

Here is a quick overview of what we’ll cover.

We’ll begin with the story of hashdos origins

Then we’ll dive into technical explanation—hashing, collisions, and the actual attack.

Next, I’ll show a short demo using an HTTP server.

Finally, we’ll focus on how to defend against it in C++.

## Slide 3: A short story

Let's start with a short story.

In 2011, some server admins began noticing strange slowdowns — simple web requests were suddenly consuming a lot of CPU.

Upon investigation, it turned out to be hash table collisions.

Around the same time, two security researchers gave a talk at a security conference showing exactly this — how hash-based containers in many platforms could be
attacked with crafted inputs.

Their presentation explained and demonstrated the risk, which prompted urgent updates across ecosystems.

## Slide 4: What is HashDoS? – Key concepts

Let’s recap the terms.

Hashing turns inputs like strings into a fixed-size value, like an integer.

Collisions happen when two different inputs produce the same hash.

Hash-based collections like `unordered_map` use hashes for quick access.

Collisions aren’t a bug — they’re part of how hash tables are designed.

Under normal conditions, collisions are rare and easily handled, often by storing colliding elements in a linked list or similar structure.

But HashDoS is about violating those assumptions. An attacker crafts input that causes many collisions on purpose, turning the hash table complexity from big O of one into big O of n.

It’s a classic example of hacking — using a tool in a way that breaks its intended preconditions.

## Slide 5: Attack Mechanism

In normal usage, hash tables give very fast lookups.

But if many colliding keys are inserted, the internal structure degrades—often into a linked list.

This consumes CPU, leading to denial of service.

This isn’t just about web servers.

Any program that inserts untrusted user input into a hash-based container without validating or limiting it can be vulnerable.

That includes web APIs, CLI tools, desktop apps, and even IoT firmware — anywhere unordered_map is used on unchecked input.

The attack scales with the number of collisions and can degrade performance severely.

## Slide 6: Demo intro

Now, let’s switch to a demo.

I hoped to demonstrate the full attack, but quickly realized I'm not an experienced or motivated hacker.

Hacking takes time, and I’m not young enough to stay up all night coding and testing.

So instead, this will be a simulation.

## Slide 7: Demo setup

The demo uses my own lightweight HTTP server based on the `httplib` header-only library.

I’m sending requests with Postman.

## Slide 8: Collisions

As you can see I took advice from Michal Žamboch and I'm using xmake.

First, I’ll demonstrate that hash collisions do exist.

All hash functions have collisions — that’s a mathematical certainty.

But finding meaningful collisions in practice is hard.

You can try exploiting a hash function’s design, or just brute-force inputs until some collide.

I was hoping someone had already prepared a nice list of FNV-1a multicollisions — maybe even for 32-bit or 64-bit, which is what MSVC uses in std::hash.

But unfortunately, Google does not seem to index the darknet 😊

So in the end, I had to simulate the attack rather than use a real one.

## Slide 9: HashDoS Simulation

Now for the simulation of the HashDoS attack.

I created a very naive hash function that only uses the first 6 characters.

It’s easy to find many inputs that hash to the same value.

We’ll see how this can be abused to create performance problems.

## Slide 10: Mitigations

How do we defend against HashDoS?

First, prefer 64-bit apps—more bits means fewer collisions.

Second, sanitize your input. Filter values or restrict their size.

Most importantly: seed your hash function with semi-random values. This makes it harder for attackers to precompute collisions.

Use a secure hash like SipHash with a per-instance seed.

## Slide 11

### 🗣 Speaker Notes (for both slides)

> Here’s one way to mitigate: plug in your own hash function.  
> This example uses a slightly modified FNV-1a with a random seed — that seed helps make the hash unpredictable to an attacker.  
> C++ lets you pass your own hash functor to `unordered_map`.  
> This gives you control and makes the structure resistant to precomputed collisions.

## Slide 12: C++ Associative Containers

Let’s look at the main associative containers in C++.

Ordered containers like map and multimap use balanced trees, so their lookup is O(log n) and not affected by hashing.

Unordered containers like unordered_map are hash-based. They provide O(1) average lookup, but under collision-heavy scenarios, they degrade to O(n).

This is where HashDoS becomes a real threat — only for hash-based containers.

## Slide 12: Resources

If you’d like to read more, here are some resources.

These include background on the CVE from 2011, various hash functions, and some blog posts showing how common functions can be broken.

## Slide 12: Thank you

Thank you for your attention!

I’ll be happy to answer any questions.
