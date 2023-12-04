# π-calculation in parallel 
# NumThreads ← - 4 NumProcs
# Step ← ×NumThreads 4
# x ← now
# /+wait≡spawn(|1 ⊙;⍥(|2.2 ⊙'+Step+÷∶8/×,)⌈÷NumThreads/× 50_1000_1000 0) ∵'+1_3×4⇡NumThreads
# &p - x now
Limit ← /× 100_1000_1000
NumThreads ← 1
Step ← ×NumThreads 4
x ← now
∵(+1_3)×4⇡NumThreads
≡spawn(|1 ⊙;⍥(|2.2 ⊙(+Step)+÷:8/×,)⌈÷NumThreads Limit 0)
⍤ "pi" <0.01⌵-3.1415/+wait
&p ⊂ NumThreads - x now

NumThreads ← 2
Step ← ×NumThreads 4
x ← now
∵(+1_3)×4⇡NumThreads
≡spawn(|1 ⊙;⍥(|2.2 ⊙(+Step)+÷:8/×,)⌈÷NumThreads Limit 0)
⍤ "pi" <0.01⌵-3.1415/+wait
&p ⊂ NumThreads - x now

NumThreads ← 3
Step ← ×NumThreads 4
x ← now
∵(+1_3)×4⇡NumThreads
≡spawn(|1 ⊙;⍥(|2.2 ⊙(+Step)+÷:8/×,)⌈÷NumThreads Limit 0)
⍤ "pi" <0.01⌵-3.1415/+wait
&p ⊂ NumThreads - x now

NumThreads ← 4
Step ← ×NumThreads 4
x ← now
∵(+1_3)×4⇡NumThreads
≡spawn(|1 ⊙;⍥(|2.2 ⊙(+Step)+÷:8/×,)⌈÷NumThreads Limit 0)
⍤ "pi" <0.01⌵-3.1415/+wait
&p ⊂ NumThreads - x now

NumThreads ← 5
Step ← ×NumThreads 4
x ← now
∵(+1_3)×4⇡NumThreads
≡spawn(|1 ⊙;⍥(|2.2 ⊙(+Step)+÷:8/×,)⌈÷NumThreads Limit 0)
⍤ "pi" <0.01⌵-3.1415/+wait
&p ⊂ NumThreads - x now

NumThreads ← 6
Step ← ×NumThreads 4
x ← now
∵(+1_3)×4⇡NumThreads
≡spawn(|1 ⊙;⍥(|2.2 ⊙(+Step)+÷:8/×,)⌈÷NumThreads Limit 0)
⍤ "pi" <0.01⌵-3.1415/+wait
&p ⊂ NumThreads - x now

NumThreads ← 7
Step ← ×NumThreads 4
x ← now
∵(+1_3)×4⇡NumThreads
≡spawn(|1 ⊙;⍥(|2.2 ⊙(+Step)+÷:8/×,)⌈÷NumThreads Limit 0)
⍤ "pi" <0.01⌵-3.1415/+wait
&p ⊂ NumThreads - x now
