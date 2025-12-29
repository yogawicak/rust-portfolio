# Rust Concept
## 1. Ownership

### The 3 Rules of Ownership
1. Each value in Rust has a variable that’s called its owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

### Term Of Ownership and Borrowing
Think of "Borrowing" like lending a book to a friend.
- Ownership ( T ): You give the book to your friend as a gift. It's theirs now. You can't read it anymore. If they lose it, it's gone.
- Borrowing ( &T ): You let your friend read the book, but you still own it. They must give it back to you. While they have it, they can look at it, but they can't destroy it or give it away to someone else forever.
