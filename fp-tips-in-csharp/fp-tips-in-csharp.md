---
marp: true
footer: "2025-06-26 - ChemiSem Inclusive Innovation Event"
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
  bottom: 20px;
  right: 10px;
  width: 210px;
}
</style>

![logo](../img/edhouse_logo.png)

# `N` tips to improve your C# code using `FP`

![title-image](./img/thumbs-up.png)

`var FP = "Functional Programming";`
`var N = 2;`

---

<!-- Odtud začínají být vidět čísla slidů -->
<!-- paginate: true -->
<!-- footer: Tip #1: Use Pure Functions -->

## Tip #1: Use Pure Functions

### Pure functions

- No side effects (e.g., no modifying object state, globals, no I/O)
- No state: Same input → same output

### Why?

- Easier to test
- Easier to reason about
- Promotes immutability
- Ready for concurrency

![bg right:40% width:500px](./img/pure-function.png)

---
<!-- footer: Tip #1: Use Pure Functions -->
### Use Pure Functions - even in non-FP code base

- 🧩 You don't need to rewrite everything to FP
- 🏝️ Isolate new code in pure functions
- ✅ works great for
  - 🔧 Bugfixing
  - 🌱 Adding new functionality

---

### Real world example - PeakFinder

```csharp
public class PeakFinder
{
    public double OverVoltage { get; set; }
    public double Sensitivity { get; set; }
    public Spectrum Spectrum { get; set; }

    public List<Peak> GetPeaks(List<ReferenceElement> references)
    {
        // Complex logic here
        ...
        return peaks;
    }
}
```

- Based on real code - [quant/quant_legacy/include/quant/PeakID.h](https://bro-gitlab.w2k.feico.com/sky/microanalysis/projects/quant/-/blob/35683b4cfcaf73815297919d166979bdd6b50891/quant_legacy/include/quant/PeakID.h)

---

### Example - new task

💬 _"We need to implement a maximum energy limit for detected peaks."_ ![bg right:40% width:400px](./img/new-task.png)

---

### ❌ Temptation: Follow the existing pattern

```csharp
public class PeakFinder
{
    public double? MaxEnergy { get; set; }

    public List<Peak> GetPeaks(List<ReferenceElement> references)
    {
      foreach (...)
      {
        // complex peak processing logic

        if (MaxEnergy.HasValue && peak.Energy <= MaxEnergy.Value)
        {          
          peaks.Add(peak);
        }
      }
    }
}
```

---

### 💥 Why this is bad

❌ Silent filtering logic buried in loop
❌ Behavior changes based on hidden state
❌ Poor reusability & testability
❌ Level of nesting increases
❌ Complexity increases
❌ SRP violated
❌ You name it...

This is how technical debt begins (and grows)

---

### ✅ Solution: Use pure function

```csharp
public static class PeakFilter
{
    public static List<Peak> ByMaxEnergy(double maxEnergy, List<Peak> peaks) =>
        peaks.Where(p => p.Energy <= maxEnergy).ToList();
}
```

```csharp
public class PeakFinder
{
  public List<Peak> GetPeaksFiltered(List<ReferenceElement> references, double maxEnergy)
  {
    var allPeaks = GetPeaks(references);
    return PeakFilter.ByMaxEnergy(maxEnergy, allPeaks);
  }
}
```

---

### ✅ Benefits of Pure Functions

Benefits of the FP Way

- 🔍 Focus: One function = one responsibility (SRP)
- 🧪 Testability: Simply write `PeakFilter.ByMaxEnergy` unit tests
- 📦 Reusability: Reuse it elsewhere if needed
- 🧠 Readability: Clear separation of concerns

✨ _We added a feature without changing any core logic or internal state._

---

### ✅ Performance matters in FP world too - Lazy evaluation

```csharp
public IEnumerable<Peak> GetPeaks(IEnumerable<Reference> refs)
{
    foreach (...)
    {
        ...
        yield return peak;
    }
}
```

```csharp
public static class PeakFilter
{
    public static IEnumerable<Peak> ByMaxEnergy(IEnumerable<Peak> peaks, double maxEnergy) =>
        peaks.Where(p => p.Energy <= maxEnergy);
}
```

---

### Tip #1: Use Pure Functions - Conclusion

- PF can be used anywhere
- PF should be the first choice for new code
- PF help improve code quality

---

## Tip #2: Try F#
<!-- footer: "" -->

### What is F#?

- Functional-first language on .NET
- Strongly typed, statically checked
- Seamless interop with C# and .NET libraries
- Well supported by the .NET ecosystem

![bg right:40% width:500px](./img/f-sharp-logo.png)

---

### Why use F#?
<!-- footer: Tip #2: Try F# -->

- Experience a different way of thinking about problems
- See how immutability, pattern matching, and type inference feel in practice
- Awesome type system
- Conciseness
- Well supported - even Vim supports F#!
- It’s fun!

---

### How to use F#?

Getting started

- `dotnet new console -lang F#`
- Try [F# for fun and profit](https://fsharpforfunandprofit.com/)
- Explore [the official F# docs](https://docs.microsoft.com/en-us/dotnet/fsharp/)
- Play with F# in [Try F#](https://dotnetfiddle.net/)

- Use F# to explore the .NET framework interactively
- Use F# for testing
- Use F# for domain driven design (DDD)

---

### Tip #2: Try F# - Conclusion

- F# is a great way to expand your programming horizons
- You don’t have to switch jobs or rewrite your codebase
- Just try it out, and see what you learn!
