# Impractical Rust Projects

I thought it would be fun and useful to practice Rust programming by implementing programs in Rust from the book *Impractical Python Projects* by Lee Vaughn.

This example is for the "Finding Palingrams" project.

## Featuring
* Using HashSet to speed up membership checks
* Writing a file of the results
* Using std::time::Instant to time the calculation
* String handling of UTF-8 strings containing grapheme cluster characters (see Extra Credit below) 

## Using
From the project root directory:
```
> cargo build --release
> ./target/release/palingram[.exe]
Time elapsed is: 222.0983ms
```
Produces file pairs.txt containing the palingrams found in search of the dictionary.txt containing 74K words.

The algorithm is interesting: loop thru each word comparing reversed prefixes to dictionary. If reversed prefix match is found and the rest of the root word following the matching prefix is a palindrome, then you've found a palingram phrase. Repeating search of root word from the other end looking for matching reversed suffix and a palindromic prefix completes the full search.

Use of the Hashset makes searching for word matches in the dictionary very fast and is also useful for weeding out duplicate phrases in the results when the root word is a palindrome (it produces the same phrase twice otherwise.)

## Extra Credit
The python project assumes ASCII characters only in the dictionary text. The rust version here processes UTF-8 strings in general. There is some slight overhead in this compared to the python version - which pretty makes the run times equivalent or ~0.5 seconds. However, using the same assumption in the rust version using slices - rust is about twice as fast.

String slicing in python using indices works on UTF-8 but you need to know when a character takes more than 1 byte:
* ```print("y̆y"[:1]=="y̆") # false```
* ```print("y̆y"[:1]=="y") # true```
* ```print("y̆y"[:2]=="y̆") # true```

Using Rust byte slicing ```"mystring[0..4]``` of the strings needed to be replaced with functions that operate on "graphemes" - multi-byte sequences making up a single "character". This functionality is provided by the [unicode-segmentation](https://crates.io/crates/unicode-segmentation) crate.

Those functions are included. It also has the benefit of further improving readability:
* ```left(str:&str,end:usize) -> String```
* ```right(str:&str,start:usize) -> String```
* ```reverse(str:&str) -> String```
* ```substring(str:&str,start:usize,end:usize) -> String```








