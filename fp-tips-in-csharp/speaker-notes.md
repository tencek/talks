# Speaker Notes for "N tips to improve your C# code using FP"

## Slide 1: Title

Hello everyone, and thank you for joining me today. My name is Pavel Kučera, and I originally promised to give you 7 tips to improve your C# code using functional programming. But, using some clever optimization techniques, I managed to narrow it down to just 2 tips. So, let's get started!

---

## Slide 2: Tip #1: Use Pure Functions

Let’s start with something easy to digest. The first tip is to **use pure functions.**

This one is a kind of captain obvious. You don't have to be FP enthusiast to know that functional programming is about functions, right? But let's see if we could learn something new anyway.

Just a quick recap: Pure functions are functions that have no side effects, meaning they don’t modify any external state or rely on it. They always produce the same output for the same input—they are stateless and deterministic.

This makes them easier to test, reason about, and reuse.

And also fun to work with!

---

## Slide 3: Use Pure Functions - even in a non-FP codebase

Here’s the point of the first tip: You don’t have to rewrite your whole codebase in a functional style. Even if your codebase is huge, in legacy technology, and far from being functional, you can still benefit from using pure functions.

Whenever you modify your code, there's always an opportunity to introduce pure functions. You can isolate new code in pure functions, even if the rest of the code is not functional. This works great for bugfixes and for adding new features as well. It’s a way to improve your codebase gradually, without a big rewrite.

---

## Slide 4: Real world example - PeakFinder

Here’s a real-world example I found in the microanalysis C++ Quant project. I just rewrote it in C#. The class PeakFinder is, I believe, a good example of how not to use OOP. But I'm not going to blame the code, nor am I going to blame OOP. I just want to demonstrate that even heavily OOP code can be easily integrated with pure functions.

As you can see, to use the PeakFinder class, you need to create an instance, set over-voltage, set sensitivity, even set the spectrum you want to search the peaks in, and then call the GetPeaks method, providing a list of reference elements to get the peaks.

Alles klar?

---

## Slide 5: Example - new task

Now, let’s say you get a new requirement: “We need to implement a maximum energy limit for detected peaks.” The client code wants to get a list of peaks that are below a certain energy threshold.

---

## Slide 6: Temptation: Follow the existing pattern

The developer's temptation when modifying any code is to follow the existing patterns. And that's for a good reason! We want our codebase to be consistent. For the same reason, another developer may be tempted to rewrite the whole class in a way they think is better (e.g., FP). But this is almost never the best approach—it can be costly and introduce bugs.

So let's say you follow your instincts, you follow the pattern you see around. You add a new property, MaxEnergy, and find the right place to check it inside the big, bulky GetPeaks method. The changeset is pretty small (like 2 lines of code), so there is a good chance it passes the code review and gets merged. So is there anything wrong with this approach?

---

## Slide 7: Why this is bad

I think there is!

- Silent filtering logic buried in a loop
- Behavior changes based on hidden state
- Poor reusability and testability
- Level of nesting increases
- Complexity increases
- SRP violated
- You name it...

This is basically how technical debt is created.

Can we do better?

---

## Slide 8: Solution: Use pure function

Yes, we can! We can use FP, namely pure functions, to solve this problem.

A better solution is to write a pure function that filters the peaks by energy. This function is easy to test, easy to reuse, and keeps your domain logic separate from your control flow.

---

## Slide 9: Benefits of Pure Functions

The benefits of pure functions are clear: each function has a single responsibility, it’s easy to test, easy to reuse, and easy to read. By using a pure function, we were able to implement the new feature without modifying the existing code and without introducing any side effects.

---

## Slide 10: Performance matters in FP world too - Lazy evaluation

One may object that our FP solution introduced unnecessary memory overhead. And it looks like it does. We create a new list, fill it with all the peaks, and then filter it. But that's not because of FP, but because of the naive way we implemented it. With a little bit of care, we can make it more efficient by using lazy evaluation. Now we modify the original code, but we do not touch the logic. At least we hope we do not. In complicated spaghetti code, it may be hard to tell.

---

## Slide 11: Tip #1: Use Pure Functions - Conclusion

So, to wrap up this first tip: Pure functions can be used anywhere, in any codebase. They should be your first choice when writing new code, because they help improve code quality, make your code easier to test, and reduce bugs. Even if you’re not doing full-on functional programming, just using more pure functions will make a big difference over time.

---

## Slide 12: Tip #2: Try F# - What is F#?

Now, let’s move to the last tip, and the one I want to put the most weight on: Try F#.

What is F#?

F# is a functional-first language on .NET. By functional-first, I mean that it supports functional programming as its primary paradigm, but it also supports object-oriented and imperative programming. Anything you can do in C#, you can do in F#. It’s a first-class citizen of the .NET ecosystem.

It works seamlessly with C# and all .NET libraries, so you can easily use all the libraries you already know and love. And it works vice versa as well, so you can consume F# assemblies from C# without any issues. You can even have a mixed codebase with both C# and F# files in the same solution.

---

## Slide 13: Why use F#?

I think, for C# developers, F# is the most straightforward way to get into functional programming. It’s a great way to learn functional programming concepts, like immutability, higher-order functions, pattern matching, and more. I would definitely recommend trying F# for its awesome type system and its concise syntax.

---

## Slide 14: How to use F#?

Getting started is easy. If you have the .NET SDK installed, you can create a new F# project with the command: dotnet new console -lang F#. There are great resources online, like F# for fun and profit, the official F# docs, and dotnetfiddle, which lets you play with F# in your browser.

---

## Slide 15: Tip #2: Try F# - Conclusion

So, to wrap up: F# is a great way to expand your programming horizons. You don’t have to switch jobs or rewrite your codebase. Just try it out, and see what you learn! I promise, it will make you a better developer, no matter what language you use.

## Slide 16: Thank You

Thank you for your attention! Here's what I wanted to tell you in two bullets:

- Pure function is a programmer's best friend. You can use them anywhere, anytime, and they will make your code better.
- F# is a great way to learn functional programming concepts. So all the FP tips I missed today, you can learn by trying F#.

In case there are questions, I will be happy to answer them now.
