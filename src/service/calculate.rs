pub fn gcd(mut n: u64, mut m: u64) -> u64
{
    while m != 0
    {
        if m < n
        {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

pub fn add_num(mut x: u64, mut y: u64) -> u64
{
    if x != 0 && y != 0
    {
        x + y
    }else{
        0
    }
}
