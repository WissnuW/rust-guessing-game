# OVERVIEW


 ``` rust
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small stupid"),
            Ordering::Greater => println!("Stupid too big!!!!"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
```
In the code snippet above, it explains that there has been a comparison of 2 values, the first value `guess` with the second value `secret_number`. Ensure that the value of `guess` is greater or less than the value of `secret_number`
If we have successfully guessed the correct number, then the comparison of the two values ​​will be stopped.
