fn hello() -> i64
{
    i64 a = 1*2 + 3;
    i64 b = 1 + 2;
    i64 c = (b + a)*2;
    f64 d = 3.14*2;
    print(a, b, c, d);
}

fn main(i64 argc) -> i64
{
    hello();
}
