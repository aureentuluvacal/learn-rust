// Patterns can be irrefutable or refutable; they can match for any possible
// value or for a specific value, respecitively. Places patterns are used:
// 1. "match"
//    - Each arm is refutable
// 2. "if let"
//    - A condition in an if let is refutable
// 3. "while let"
//    - Refutable
// 4. for loops
//    - Irrefutable
// 5. "let", when we're assigning variables, the pattern is the variable name.
//    - Irrefutable
// 6. function parameters
//    - Irrefutable

// Can match ranges (a...c) in a match arm or use "_" in the last arm of match 
// as a catch all for any type.
