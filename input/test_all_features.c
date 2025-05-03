// test_all_features.c

// A simple function with two parameters
int sum(int a, int b) {
    return a + b;
}

// A factorial function using a while‐loop
int fact(int n) {
    int r;
    r = 1;
    while (n > 1) {
        r = r * n;
        n = n - 1;
    }
    return r;
}

int main() {
    // 1) Print a string literal (with an escape)
    print("\n");

    print("====== Testing all features ======\n");

    // 2) Print another string
    print("________Print string________\n");
    print("Hello, world!\n");

    // 3) Function calls (no args, two args)
    print("________Function calls (no args, two args)________\n");
    print(sum(7, 5));
    print(fact(5));

    // 4) Casting
    print("________Casting________\n");
    print((int)123);

    // 5) Arithmetic
    print("________Arithmetic________\n");
    print(3 + 4 * 2 - 5);
    print(10 / 3);
    print(10 % 3);

    // 6) Unary minus and logical not
    print("________Unary minus and logical not________\n");
    print(-5);
    print(!0);
    print(!42);

    // 7) Comparisons
    print("________Comparisons________\n");
    print(1 < 2);
    print(2 <= 2);
    print(3 == 3);
    print(4 != 5);
    print(5 > 6);
    print(6 >= 6);

    // 8) Conditional operator
    print("________Conditional operator________\n");
    print(1 ? 100 : 200);
    print(0 ? 100 : 200);

    // 9) Bitwise
    print("________Bitwise________\n");
    print(6 & 3);
    print(6 | 3);
    print(6 ^ 3);

    // 10) Shifts
    print("________Shifts________\n");
    print(1 << 4);
    print(16 >> 2);

    // 11) sizeof
    print("________sizeof________\n");
    print(sizeof(int));
    print(sizeof(char));

    // 12) Char literal
    print("________Char literal________\n");
    char c;
    c = 'Z';
    print(c);

    // 13) Pointers
    print("________Pointers________\n");
    int x;
    x = 42;
    int *p;
    p = &x;
    print(*p);

    // 14) While loop again
    print("________While loop again________\n");
    int i;
    i = 0;
    while (i < 3) {
        print(i);
        i = i + 1;
    }

    return 42;
}
