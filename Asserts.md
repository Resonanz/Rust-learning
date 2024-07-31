Wow, this is unexpected: https://rust.godbolt.org/z/jcrbeKeYP

This code demonstrates that ADDING asserts can REDUCE the size of your code. Why is this?

This Reddit post has a short discussion: https://www.reddit.com/r/rust/comments/11swwhb/rusts_two_kinds_of_assert_make_for_better_code/

> Asserts are almost never a detectable performance hit, unless you're asserting in a hot loop, or doing some particularly complex checks. You should always use assert!, rather than debug_assert!, unless you have strong reasons to do otherwise. Like, a costly check, or you have benchmarked your code and identified that eliminating a runtime assertion gives a measurable speedup.

>In fact, **assert! can often improve your performance**, because a well-placed assertion may help the compiler to **eliminate many other redundant checks**. It's a common trick when dealing with autovectorization: **asserting that the lengths of slices are correct before the loop starts helps to eliminate all the bounds checks in the loop**.
