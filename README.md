# Impractical Rust Projects

I thought it would be fun and useful to practice Rust programming by implementing programs in Rust from the book *Impractical Python Projects* by Lee Vaughn.

This example is for the "Finding Palingrams" project.

## Featuring
* Using HashSet to speed up membership checks
* Writing a file of the results
* Using std::time::Instant to time the calculation
* String slicing

## Using
From the project root directory:
```
> cargo build --release
> ./target/release/palingram[.exe]
Time elapsed is: 127.0983ms
```
Produces file pairs.txt containing the palingrams found in search of the dictionary.txt containing 74K words.

By comparison, the python version from the book takes 2-3 times longer to run. Still under a second.

The algorithm is interesting loop thru each root word comparing reversed prefixes to dictionary. If match is found and rest of the root word after matching reversed suffix is a palindrome, then you've found a phrase.
Repeat search of root word from the other end looking for matching reversed suffix and a palindromic prefix.

Use of the Hashset makes searching for word matches in te dictionary very fast. and is also useful for weeding out duplicate phrases in the results when root word is a palindrome it produces the same phrase twice.

## Extra Credit
The python project assumed ASCII text. The rust version here basically ignores UTF-8 strings that have a different character count than byte length to avoid having to deal with words like 'fiancé'. 

Using slicing of the strings would need to be replaced with a "substring" function that is aware of "graphemes" or characters that span multiple bytes.

















